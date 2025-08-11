use crate::application::usecase::integration::telegram::dto::TelegramDataDTO;
use crate::interface::web::state::AppState;
use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;


#[post("/telegram/auth")]
pub async fn auth_telegram(
    data: web::Data<AppState>,
    payload: web::Json<TelegramDataDTO>,
) -> impl Responder {
    let dto = payload.into_inner();

    let result = data
        .auth_telegram_use_case_concrete
        .execute(dto)
        .await;

    match result {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Internal error"
        })),
    }
}
