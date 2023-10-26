use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use sea_orm::{Database, DatabaseConnection};

pub async fn db_connect() -> DatabaseConnection {
    dotenv().ok();
    let database_url = dotenv!("DATABASE_URL");

    let database = Database::connect(database_url).await.unwrap();

    database
    // println!("Cool Man hello !!!!!");
}
