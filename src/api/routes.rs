use actix_web::{web,HttpResponse, Responder};

async fn index() -> impl  Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn users() -> impl  Responder {
    HttpResponse::Ok().body("Listing users...")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(index)));
    cfg.service(web::resource("/users").route(web::get().to(users)));
}