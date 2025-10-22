use serde::Deserialize;
use sqlx::prelude::FromRow;

#[derive(Deserialize, Debug, FromRow)]
pub struct Announcement {
    #[sqlx(rename = "announcement_id")]
    pub announcementId: i64,
    #[sqlx(rename = "announcement_title")]
    pub announcementTitle: String,
    #[sqlx(rename = "announcement_time")]
    pub announcementTime: i64,
    #[sqlx(rename = "adjunct_type")]
    pub adjunctType: String,
    #[sqlx(rename = "adjunct_url")]
    pub adjunctUrl: String,
}
