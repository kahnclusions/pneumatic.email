use jmap_client::client::{Client, Credentials};

/// Get a JMAP session username and account name from the server.
#[tracing::instrument(skip(token))]
pub async fn get_jmap_account(server_url: &str, token: &str) -> anyhow::Result<(String, String)> {
    tracing::debug!("Get JMAP account using server_url: {server_url} and token: {token}");

    let credentials = Credentials::bearer(token);
    let client = Client::new()
        .credentials(credentials)
        .connect(server_url)
        .await?;
    let session = client.session();
    let account = session
        .account(client.default_account_id())
        .map(|acc| acc.to_owned())
        .ok_or(anyhow::anyhow!("Logged in but no account? Weird!"))?;

    tracing::debug!("Got a valid JMAP session and account from the server.");

    Ok((session.username().to_owned(), account.name().to_owned()))
}
