pub mod routes;

use actix_web::{web, App, HttpServer};

pub async fn start_server() {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api").configure(routes::config))
    })
    .bind("127.0.0.1:8080")
    .expect("Can't bind to port 8080")
    .run()
    .await.expect("Failed to run server")
}