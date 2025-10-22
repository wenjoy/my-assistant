use serde::Deserialize;
#[derive(Deserialize, Debug)]
pub struct Announcement {
    pub(crate) announcementId: String,
    pub(crate) announcementTitle: String,
    pub(crate) announcementTime: i64,
    pub(crate) adjunctType: String,
    pub(crate) adjunctUrl: String,
}
