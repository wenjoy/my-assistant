use sqlx::{Connection, SqliteConnection};

pub async fn create_shema(db_url: &str) {
    let conn = SqliteConnection::connect("sqlite:memory:").await;
    sqlx::query("select * from announcements").execute(&mut conn);
}
