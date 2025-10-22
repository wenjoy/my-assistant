use sqlx::{Error, SqliteConnection, sqlite::SqliteRow};

use crate::model::Announcement;

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

pub async fn query_all_data(conn: &mut SqliteConnection) -> Result<Vec<SqliteRow>, Error> {
    let res = sqlx::query("select * from announcements")
        .fetch_all(conn)
        .await?;

    Ok(res)
}

pub async fn query_latest_data(conn: &mut SqliteConnection) -> Result<SqliteRow, Error> {
    let res = sqlx::query("SELECT * from announcements ORDER BY announcement_time DESC LIMIT 1")
        .fetch_one(conn)
        .await?;
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

pub async fn initial_database(conn: &mut SqliteConnection) -> Result<(), Error> {
    create_shema(conn).await?;
    Ok(())
}

pub async fn clear_database(conn: &mut SqliteConnection) -> Result<(), Error> {
    sqlx::query("DELETE FROM announcements")
        .execute(conn)
        .await?;
    println!("Database cleared");
    Ok(())
}
