use actix_web::{post, web, HttpResponse, Responder};
use crate::interface::web::state::AppState;
use crate::application::auth::dto::{LoginEmailPasRequestDto, LoginEmailPasResponseDto};

#[post("/login")]
pub async fn login(
    data: web::Data<AppState>,
    payload: web::Json<LoginEmailPasRequestDto>,
) -> impl Responder {
    let dto = payload.into_inner();
    let result = data.login_use_case.clone().login(dto).await;

    match result {
        Ok(LoginEmailPasResponseDto::Success { auth_data }) => {
            HttpResponse::Ok().json(auth_data)
        }
        Ok(LoginEmailPasResponseDto::Error { err_msg }) => {
            HttpResponse::Unauthorized().json(serde_json::json!({ "error": err_msg }))
        }
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Internal error"
        })),
    }
}
