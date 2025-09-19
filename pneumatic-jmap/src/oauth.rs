use chrono::{DateTime, Utc};
use oauth2::basic::BasicClient;
use oauth2::{reqwest, AuthType, EndpointNotSet, PkceCodeVerifier};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    Scope, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use hickory_client::client::{Client, SyncClient};
use hickory_client::op::DnsResponse;
use hickory_client::rr::{DNSClass, Name, RData, Record, RecordType};
use hickory_client::udp::UdpClientConnection;
use tokio::task::spawn_blocking;

use crate::account::get_jmap_account;

/// Details of a JMAP server that we can use OAuth to 
/// authenticate with.
#[derive(Debug, Serialize, Deserialize)]
pub struct JmapOauthServer {
    pub token_endpoint: String,
    pub authorization_endpoint: String,
    pub device_authorization_endpoint: String,
    pub introspection_endpoint: String,
}

/// Given an e-mail address, try to lookup the associated jmap domain.
/// If we can auto-detect the JMAP service on this e-mail's domain, then
/// we can default to this. Otherwise the user will have to specify their
/// JMAP server by hand.
#[tracing::instrument(skip(email))]
pub async fn lookup_jmap_domain(email: &str) -> anyhow::Result<String> {
    let (_name, domain) = email.split_once("@").unwrap();

    // TODO: use the system provided DNS.
    let address = "8.8.8.8:53".parse().unwrap();
    let domain2 = domain.to_owned();
    let response = spawn_blocking(move || {
        let domain = domain2.clone();
        let full_domain = format!("_jmap._tcp.{domain}.");
        println!("Look up a JMAP record for this domain: {}", full_domain);
        let name = Name::from_str_relaxed(full_domain.as_str()).unwrap();
        let conn = UdpClientConnection::new(address).unwrap();
        let client = SyncClient::new(conn);
        let response: DnsResponse = client.query(&name, DNSClass::IN, RecordType::SRV).unwrap();
        println!("Response: {:?}", response);
        response
    })
    .await?;

    let answers: &[Record] = response.answers();
    let res = answers.iter().find_map(|record| {
        if let Some(RData::SRV(value)) = record.data() {
            Some(value.target().to_string())
        } else {
            None
        }
    });
    println!("Res: {:?}", res);
    let domain = res.unwrap_or(domain.to_string());
    println!("Using domain: {}", domain);
    Ok(domain)
}

/// Given a base URL, which can be just a domain name, lookup its .well-known directory 
/// to find an oauth server, which tells us we can use oauth to login with this server.
#[tracing::instrument]
pub async fn lookup_oauth_server(base_url: &str) -> anyhow::Result<(String, JmapOauthServer)> {
    let mut base_url = if base_url.contains("@") {
        lookup_jmap_domain(base_url).await?
    } else {
        // Otherwise just use it as the URL
        base_url.to_string()
    };

    if base_url.ends_with(".") {
        base_url.remove(base_url.len() - 1);
    }

    let base_url = if base_url.starts_with("https") {
        base_url
    } else {
        format!("https://{base_url}").to_string()
    };

    let base_url = if base_url.ends_with("/") {
        base_url
    } else {
        format!("{base_url}/").to_string()
    };

    // Once we have a URL, try to discover the oauth details.
    let result = reqwest::get(format!("{base_url}.well-known/oauth-authorization-server")).await?;
    let json = result.json().await?;

    let base_url = if base_url.ends_with("/") {
        let (head, _tail) = base_url.split_at(base_url.len() - 1);
        head.to_string()
    } else {
        base_url
    };

    Ok((base_url, json))
}

#[tracing::instrument]
fn get_client() -> anyhow::Result<
    BasicClient<EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointNotSet>,
> {
    let client = BasicClient::new(ClientId::new("pneumatic".to_string()))
        .set_auth_type(AuthType::RequestBody)
        .set_redirect_uri(RedirectUrl::new("pneumatic://oauth-complete".to_string())?);
    // .set_client_secret(ClientSecret::new(
    //     "fdkasjfdaklbvcj3h32d9dhjdsubmzq18d0398fjhkdsfh3892hfjqbvmbkfhvhhjubfdv832gvf"
    //         .to_string(),
    // ))

    Ok(client)
}

pub struct JmapOauthChallenge {
  pub auth_url: String,
  pub csrf_token: String,
  pub verifier: String
}

#[tracing::instrument(skip(email))]
pub async fn start_authentication(
    server_url: String,
    oauth_server: &JmapOauthServer,
    email: &str,
) -> anyhow::Result<JmapOauthChallenge> {
    let client = get_client()?
        .set_auth_uri(AuthUrl::new(
            oauth_server.authorization_endpoint.to_string(),
        )?)
        .set_token_uri(TokenUrl::new(oauth_server.token_endpoint.to_string())?);

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("read".to_string()))
        .add_scope(Scope::new("write".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    Ok(JmapOauthChallenge { 
      auth_url: auth_url.to_string(), 
      verifier: pkce_verifier.secret().to_string(), 
      csrf_token: csrf_token.secret().to_owned() 
    })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthChallenge {
    pub id: u32,
    pub email: String,
    pub server_url: String,
    pub csrf_token: String,
    pub verifier: String,
    pub token_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JmapOauthAccessToken {
  pub access_token: String,
  pub refresh_token: Option<String>,
  pub expires: Option<DateTime<Utc>>,
  pub username: String,
  pub account_name: String
}

/// Given an auth_code from a successful oauth login, and the challenge that
/// was generated at the start of the oauth flow, try to exchange the auth code
/// for an access token from the JMAP server.
#[tracing::instrument(skip(auth_code, challenge))]
pub async fn exchange_code_for_token(
    auth_code: &str,
    challenge: OAuthChallenge,
) -> anyhow::Result<JmapOauthAccessToken> {
    let http_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Client should build");

    let pkce_verifier = PkceCodeVerifier::new(challenge.verifier);

    // Now you can trade it for an access token.
    let client = get_client()?.set_token_uri(TokenUrl::new(challenge.token_url)?);

    println!(
        "Going to use code={} and verifier={}",
        auth_code,
        pkce_verifier.secret().to_string()
    );

    let token_result = client
        .exchange_code(AuthorizationCode::new(auth_code.to_string()))
        .set_pkce_verifier(pkce_verifier)
        .request_async(&http_client)
        .await?;

    println!("Got a token result");

    let access_token = token_result.access_token().secret().to_owned();
    let refresh_token = token_result.refresh_token().map(|t| t.secret().to_owned());
    let expires = token_result
        .expires_in()
        .map(|expires| Utc::now() + expires);

    let (username, account_name) = get_jmap_account(&challenge.server_url, &access_token).await?;

    Ok(JmapOauthAccessToken {
      access_token,
      refresh_token,
      expires,
      username,
      account_name
    })
}

