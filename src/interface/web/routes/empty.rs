use crate::application::auth::dto::LoginEmailPasRequestDto;
use crate::interface::web::state::AppState;
use actix_web::{post, web, HttpResponse, Responder};

#[post("/empty")]
pub async fn empty(
    data: web::Data<AppState>,
    payload: web::Json<LoginEmailPasRequestDto>,
) -> impl Responder {
    tracing::error!("new request");
    HttpResponse::Ok()
}
