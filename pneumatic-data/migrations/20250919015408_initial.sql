CREATE TABLE accounts (
    id BLOB PRIMARY KEY,
    description TEXT,
    server_url TEXT NOT NULL,
    username TEXT NOT NULL,
    name TEXT NOT NULL,
    access_token TEXT,
    refresh_token TEXT,
    expires TEXT,
    mailbox_state TEXT,
    thread_state TEXT,
    identity_state TEXT,

    email_state TEXT,
    email_query_state TEXT,
    email_query_last_id TEXT,
    email_query_loaded BOOLEAN NOT NULL DEFAULT false
);

CREATE TABLE account_email (
    account_id BLOB NOT NULL,
    email_id TEXT NOT NULL,
    PRIMARY KEY (account_id, email_id)
);

CREATE TABLE oauth2_challenges (
    id INTEGER PRIMARY KEY,
    email TEXT NOT NULL,
    server_url TEXT NOT NULL,
    csrf_token TEXT NOT NULL,
    verifier TEXT NOT NULL,
    token_url TEXT
);
