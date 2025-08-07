use crate::application::usecase::sign_up_usecase::dto::{
    CreateApiKeyRequestDto, SignUpRequestDto
};
use crate::application::usecase::auth_usecase::dto::{
    LoginApiKeyRequestDto, LoginEmailPasRequestDto, RefreshTokenRequestDto,
};
use crate::interface::web::state::AppState;
use actix_web::{post, web, HttpResponse, Responder};

#[post("/login")]
pub async fn login(
    data: web::Data<AppState>,
    payload: web::Json<LoginEmailPasRequestDto>,
) -> impl Responder {
    let dto = payload.into_inner();
    let result = data.login_with_email_passwd_use_case.clone().execute(dto).await;

    match result {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Internal error"
        })),
    }
}

#[post("/refresh")]
pub async fn refresh(
    data: web::Data<AppState>,
    payload: web::Json<RefreshTokenRequestDto>,
) -> impl Responder {
    let dto = payload.into_inner();
    let result = data.refresh_token_use_case.clone().execute(dto).await;

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
    let result = data.login_with_api_key_use_case.clone().execute(dto).await;

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
        .create_api_key_use_case
        .clone()
        .execute(dto)
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
