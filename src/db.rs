use sqlx::{Connection, Error, SqliteConnection};

pub async fn create_shema(conn: &mut SqliteConnection) -> Result<(), Error> {
    let res = sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS announcements (
        announcement_title TEXT,
        announcement_time INTEGER,
        announcement_id INTEGER PRIMARY KEY AUTOINCREMENT,
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

pub async fn query_data(conn: &mut SqliteConnection) -> Result<(), Error> {
    let res = sqlx::query("select * from announcements")
        .execute(conn)
        .await?;
    println!("query: {res:?}");
    Ok(())
}
