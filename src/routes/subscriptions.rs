use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct FromData {
    email: String,
    name: String,
}

pub async fn subscribe(_form: web::Form<FromData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
