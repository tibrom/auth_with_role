use actix_web::{post, web, HttpResponse, Responder};
use crate::interface::web::state::AppState;
use crate::application::sign_up::dto::{SignUpRequestDto, SignUpResponseDto};

#[post("/signup")]
pub async fn signup(
    data: web::Data<AppState>,
    payload: web::Json<SignUpRequestDto>,
) -> impl Responder {
    let dto = payload.into_inner();
    let result = data.sign_up_use_case.clone().sign_up(dto).await;

    match result {
        Ok(SignUpResponseDto::Success { user }) => {
            HttpResponse::Ok().json(user)
        }
        Ok(SignUpResponseDto::Error { err_msg }) => {
            HttpResponse::Unauthorized().json(serde_json::json!({ "error": err_msg }))
        }
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Internal error"
        })),
    }
}
