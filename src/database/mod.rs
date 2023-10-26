mod db;
use crate::database::users::Column;
pub mod prelude;
pub mod tasks;
pub mod users;

use db::db_connect;
use sea_orm::DatabaseConnection;

pub async fn init_db() -> DatabaseConnection {
    db_connect().await
}
