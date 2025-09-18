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

CREATE TABLE mailboxes (
    account_id BLOB NOT NULL,
    id TEXT NOT NULL,
    parent_id TEXT,
    name TEXT NOT NULL,
    role TEXT,
    sort_order INTEGER NOT NULL DEFAULT 0,
    total_emails INTEGER NOT NULL,
    unread_emails INTEGER NOT NULL,
    total_threads INTEGER NOT NULL,
    unread_threads INTEGER NOT NULL,
    is_subscribed BOOLEAN NOT NULL DEFAULT false,

    query_state TEXT, -- the latest state of the query selecting email from this mailbox
    query_last_id TEXT, -- the last e-mail ID cached
    query_loaded BOOLEAN NOT NULL DEFAULT false,

    PRIMARY KEY (account_id, id)
);

CREATE TABLE identities (
    account_id BLOB NOT NULL,
    id TEXT NOT NULL,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    reply_to TEXT,
    bcc TEXT,
    text_signature TEXT NOT NULL,
    html_signature TEXT NOT NULL,
    may_delete BOOLEAN NOT NULL DEFAULT false,
    PRIMARY KEY (account_id, id)
);

CREATE TABLE addresses (
    address TEXT NOT NULL PRIMARY KEY,
    name TEXT
);

CREATE TABLE email (
    account_id BLOB NOT NULL,
    id TEXT NOT NULL,
    blob_id TEXT NOT NULL,
    thread_id TEXT NOT NULL,
    size INTEGER NOT NULL DEFAULT 0,
    keywords TEXT NOT NULL DEFAULT '',
    received_at TEXT NOT NULL,
    message_id TEXT NOT NULL DEFAULT '',
    from_address TEXT NOT NULL,
    from_name TEXT,
    to_addr TEXT,
    cc_addr TEXT,
    bcc_addr TEXT,
    subject TEXT,
    preview TEXT,
    has_text_body BOOLEAN NOT NULL DEFAULT false,
    has_html_body BOOLEAN NOT NULL DEFAULT false,
    has_attachments BOOLEAN NOT NULL DEFAULT false,
    PRIMARY KEY (account_id, id)
);

CREATE TABLE mailbox_email (
    account_id BLOB NOT NULL,
    mailbox_id TEXT NOT NULL,
    email_id TEXT NOT NULL,
    PRIMARY KEY (account_id, mailbox_id, email_id)
);

CREATE TABLE threads (
    account_id BLOB NOT NULL,
    id TEXT NOT NULL,
    email_ids TEXT NOT NULL,
    PRIMARY KEY (account_id, id)
);

CREATE TABLE query_states (
    account_id BLOB NOT NULL,
    name TEXT NOT NULL,
    state TEXT NOT NULL,
    PRIMARY KEY (account_id, name)
);

CREATE TABLE email_body_parts (
    account_id BLOB NOT NULL,
    email_id  TEXT NOT NULL,
    part_id TEXT,
    blob_id TEXT,
    body_type TEXT,
    size INTEGER NOT NULL,
    content_type TEXT NOT NULL,
    disposition TEXT,
    charset TEXT,
    cid TEXT,
    name TEXT,
    PRIMARY KEY (account_id, email_id, part_id)
);

CREATE TABLE email_body_values (
    account_id BLOB NOT NULL,
    email_id  TEXT NOT NULL,
    part_id TEXT,
    value TEXT NOT NULL,
    is_encoding_problem BOOLEAN NOT NULL DEFAULT false,
    is_truncated BOOLEAN NOT NULL DEFAULT false,
    PRIMARY KEY (account_id, email_id, part_id)
);

CREATE TABLE attachments (
    account_id BLOB NOT NULL,
    email_id  TEXT NOT NULL,
    part_id TEXT,
    value TEXT NOT NULL,
    is_encoding_problem BOOLEAN NOT NULL DEFAULT false,
    is_truncated BOOLEAN NOT NULL DEFAULT false,
    PRIMARY KEY (account_id, email_id, part_id)
);

CREATE TABLE contacts (
    id BLOB NOT NULL PRIMARY KEY,
    data BLOB
);

CREATE TABLE contact_email (
    email TEXT NOT NULL PRIMARY KEY,
    contact_id BLOB NOT NULL
);

