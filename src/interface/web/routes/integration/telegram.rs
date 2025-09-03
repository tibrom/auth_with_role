use crate::application::usecase::integration::telegram::dto::{
    TelegramDataDTO,
    InitDataDTO,
};
use crate::interface::web::state::AppState;
use actix_web::{post, web, HttpResponse, Responder};
use rand::rand_core::le;
use serde::Deserialize;
use urlencoding::decode;

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

#[post("/telegram/login")]
pub async fn auth_telegram(
    data: web::Data<AppState>,
    payload: web::Json<TelegramDataDTO>,
) -> impl Responder {
    let dto = payload.into_inner();

    let result = data
        .auth_telegram_use_case
        .execute(dto)
        .await;

    match result {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Internal error"
        })),
    }
}



#[post("/telegram/miniapp")]
async fn telegram_miniapp(data: web::Data<AppState>, payload: web::Json<InitDataDTO>) -> impl Responder {
    let dto = payload.into_inner();

    let result = data
        .auth_telegram_mini_app_use_case
        .execute(dto)
        .await;
    
    match result {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Internal error"
        })),
    }
    
}
