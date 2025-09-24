use my_assitant::{
    db::{create_shema, query_data},
    fetch,
};
use sqlx::{Connection, SqliteConnection};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // fetch().await?;
    let mut conn = SqliteConnection::connect("sqlite::memory:").await?;
    create_shema(&mut conn).await?;
    query_data(&mut conn).await?;
    Ok(())
}
