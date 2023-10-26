use sea_orm::Database;

mod database;
mod routes;

pub async fn run() {
    // let dtb = Database::connect(databasr_uri).await;

    let dtb = database::init_db().await;
    let app = routes::create_routes(dtb);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    // database::init_db().await;
}
