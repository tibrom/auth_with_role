use crate::application::sign_up_usecase::dto::SignUpRequestDto;
use crate::interface::web::state::AppState;
use actix_web::{post, web, HttpResponse, Responder};

#[post("/signup")]
pub async fn signup(
    data: web::Data<AppState>,
    payload: web::Json<SignUpRequestDto>,
) -> impl Responder {
    let dto = payload.into_inner();
    let result = data.sign_up_use_case.clone().execute(dto).await;

    match result {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Internal error"
        })),
    }
}
