use actix_web::{HttpResponse, Responder, web};

use crate::infra::repository::UserRepository;
use crate::use_cases::use_cases::UserUseCase;
use crate::utils::logger::Logger;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn users() -> impl Responder {
    let use_cases = UserUseCase::new(UserRepository::new(Logger));

    use_cases.get_users();

    HttpResponse::Ok().body(
        "Body..."
    )
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(index)));
    cfg.service(web::resource("/users").route(web::get().to(users)));
}