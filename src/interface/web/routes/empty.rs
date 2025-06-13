use actix_web::{post, web, HttpResponse, Responder};
use crate::interface::web::state::AppState;
use crate::application::auth::dto::{LoginEmailPasRequestDto, LoginEmailPasResponseDto};

#[post("/empty")]
pub async fn empty (
    data: web::Data<AppState>,
    payload: web::Json<LoginEmailPasRequestDto>,
) -> impl Responder {
    tracing::error!("new request");
    HttpResponse::Ok()
}
