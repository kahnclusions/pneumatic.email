use jmap_client::client::{Client, Credentials};

/// Get a JMAP session username and account name from the server.
pub async fn get_jmap_account(server_url: &str, token: &str) -> anyhow::Result<(String, String)> {
    println!("Connecting to JMAP server {server_url} with token {token}");
    let credentials = Credentials::bearer(token);
    let client = Client::new()
        .credentials(credentials)
        .connect(server_url)
        .await?;

    println!("Got a valid JMAP client");

    let session = client.session();

    let account = session
        .account(client.default_account_id())
        .map(|acc| acc.to_owned())
        .ok_or(anyhow::anyhow!("Logged in but no account? Weird!"))?;

    println!("Got a valid JMAP session");

    Ok((session.username().to_owned(), account.name().to_owned()))
}
