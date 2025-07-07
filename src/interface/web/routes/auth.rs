use crate::application::usecase::auth_usecase::dto::CreateApiKeyRequestDto;
use crate::application::usecase::auth_usecase::dto::{
    LoginApiKeyRequestDto, LoginEmailPasRequestDto,
};
use crate::interface::web::state::AppState;
use actix_web::{post, web, HttpResponse, Responder};

#[post("/login")]
pub async fn login(
    data: web::Data<AppState>,
    payload: web::Json<LoginEmailPasRequestDto>,
) -> impl Responder {
    let dto = payload.into_inner();
    let result = data.login_use_case.clone().login(dto).await;

    match result {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Internal error"
        })),
    }
}

#[post("/loginapikey")]
pub async fn loginapikey(
    data: web::Data<AppState>,
    payload: web::Json<LoginApiKeyRequestDto>,
) -> impl Responder {
    let dto = payload.into_inner();
    let result = data.login_api_key_use_case.clone().login(dto).await;

    match result {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Internal error"
        })),
    }
}

#[post("/createapikey")]
pub async fn createapikey(
    data: web::Data<AppState>,
    payload: web::Json<CreateApiKeyRequestDto>,
) -> impl Responder {
    let dto = payload.into_inner();
    let result = data
        .create_apikey_use_case
        .clone()
        .create_api_key(dto)
        .await;

    match result {
        Ok(v) => {
            return HttpResponse::Ok().json(v);
        }
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Internal error"
        })),
    }
}
