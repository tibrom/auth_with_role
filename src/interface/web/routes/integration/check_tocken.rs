use crate::application::usecase::integration::check_token::dto::CheckTokenRequestDto;
use crate::interface::web::state::AppState;
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;


#[post("/checkjwt")]
pub async fn check_token(
    req: HttpRequest,
    data: web::Data<AppState>,
    payload: web::Json<CheckTokenRequestDto>
) -> impl Responder {
    let dto = payload.into_inner();
    let api_key = match req.headers().get("Authorization") {
        Some(header_value) => match header_value.to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid Authorization header"
            })),
        },
        None => return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Missing Authorization header"
        })),
    };

    let result = data
        .check_token_use_case
        .execute(dto, api_key)
        .await;

    match result {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Internal error"
        })),
    }
}
