use sqlx::{Connection, Error, SqliteConnection};

pub async fn create_shema() -> Result<(), Error> {
    let mut conn = SqliteConnection::connect("sqlite::memory:").await?;
    let res = sqlx::query("select * from announcements")
        .execute(&mut conn)
        .await?;
    println!("res: {res:?}");
    Ok(())
}
