use my_assitant::{
    db::{create_shema, insert_data, query_data},
    fetch,
};
use sqlx::{Connection, SqliteConnection};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let result = fetch().await?;
    let mut conn = SqliteConnection::connect("sqlite:data.db").await?;
    create_shema(&mut conn).await?;
    let announcements = query_data(&mut conn).await?;
    // for item in result.announcements {
    //     insert_data(&mut conn, item).await?;
    // }
    Ok(())
}
