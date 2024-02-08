use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;

mod models;
mod routes;


async fn saluer() -> impl Responder {

    HttpResponse::Ok().body(json!({"message": "Api Rust is running on port 8080"}).to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(saluer))
           .route("/aeroport", web::get().to(routes::aeroport_routes::get_aeroport))
    })
    .bind("localhost:8080")?
    .run()
    .await
}
