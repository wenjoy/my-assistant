use sqlx::{
    Connection, Error, Row, SqliteConnection,
    sqlite::{SqliteQueryResult, SqliteRow},
};

use crate::Announcement;

pub async fn create_shema(conn: &mut SqliteConnection) -> Result<(), Error> {
    let res = sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS announcements (
        announcement_id INTEGER PRIMARY KEY AUTOINCREMENT,
        announcement_title TEXT,
        announcement_time INTEGER,
        adjunct_type TEXT,
        adjunct_url TEXT
        );
        ",
    )
    .execute(conn)
    .await?;
    println!("create: {res:?}");
    Ok(())
}

pub async fn query_data(conn: &mut SqliteConnection) -> Result<Vec<SqliteRow>, Error> {
    let res = sqlx::query("select * from announcements")
        .fetch_all(conn)
        .await?;
    for item in &res {
        println!(
            "{}",
            item.try_get::<String, _>("announcement_title").unwrap()
        );
    }
    Ok(res)
}

pub async fn insert_data(conn: &mut SqliteConnection, data: Announcement) -> Result<(), Error> {
    let res = sqlx::query(
        "INSERT INTO announcements (
        announcement_id,
        announcement_title,
        announcement_time,
        adjunct_type,
        adjunct_url
        ) VALUES(?,?,?,?,?)",
    )
    .bind(data.announcementId)
    .bind(data.announcementTitle)
    .bind(data.announcementTime)
    .bind(data.adjunctType)
    .bind(data.adjunctUrl)
    .execute(conn)
    .await?;
    println!("insert: {res:?}");
    Ok(())
}
