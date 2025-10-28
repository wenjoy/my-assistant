use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Deserialize, Serialize, Debug, FromRow)]
pub struct Announcement {
    #[sqlx(rename = "announcement_id")]
    pub announcementId: String,
    #[sqlx(rename = "announcement_title")]
    pub announcementTitle: String,
    #[sqlx(rename = "announcement_time")]
    pub announcementTime: i64,
    #[sqlx(rename = "adjunct_type")]
    pub adjunctType: String,
    #[sqlx(rename = "adjunct_url")]
    pub adjunctUrl: String,
}
