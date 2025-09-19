use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Role;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountDTO {
    pub id: Uuid,
    pub description: String,
    pub name: String,
    pub username: String,
    pub server_url: String,
    pub mailboxes: Vec<MailboxDTO>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MailboxDTO {
    pub id: String,
    pub account_id: Uuid,
    pub parent_id: Option<String>,
    pub name: String,
    pub role: String,
    pub sort_order: u32,
    pub total_emails: u32,
    pub unread_emails: u32,
    pub total_threads: u32,
    pub unread_threads: u32,
    pub is_subscribed: bool,
}
