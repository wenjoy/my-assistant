use serde::{Deserialize, Deserializer, Serialize};
use sqlx::prelude::FromRow;
use std::str::FromStr;

fn deserialize_string_to_integer<'d, D: Deserializer<'d>>(
    deserializer: D,
) -> Result<i64, D::Error> {
    let s: &str = Deserialize::deserialize(deserializer)?;
    let parse_result = i64::from_str(s);
    parse_result.map_err(serde::de::Error::custom)
}

#[derive(Deserialize, Serialize, Debug, FromRow)]
pub struct Announcement {
    #[serde(deserialize_with = "deserialize_string_to_integer")]
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
