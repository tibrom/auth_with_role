use crate::application::usecase::integration::telegram::dto::TelegramDataDTO;
use crate::interface::web::state::AppState;
use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TelegramQuery {
    token: String,
}

#[post("/telegram/link")]
pub async fn link_telegram(
    data: web::Data<AppState>,
    payload: web::Json<TelegramDataDTO>,
    query: web::Query<TelegramQuery>,
) -> impl Responder {
    let dto = payload.into_inner();
    let token = &query.token;

    let result = data
        .link_telegram_account_use_case
        .execute(dto, token.clone())
        .await;

    match result {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Internal error"
        })),
    }
}
