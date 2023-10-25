use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct SampleJson {
    messsage: String,
    count: i32,
    username: String,
}

pub async fn get_json() -> Json<SampleJson> {
    let data = SampleJson {
        messsage: "Cool".to_owned(),
        count: 3,
        username: "YoYoMan".to_owned(),
    };

    Json(data)
}
