use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde_json::json; // Assurez-vous d'inclure cette utilisation pour la macro `json!`

async fn saluer() -> impl Responder {
    // Correction : Ajoutez les parenthèses manquantes après `to_string()` et incluez `use serde_json::json;` pour la macro `json!`.
    HttpResponse::Ok().body(json!({"message": "Api Rust is running on port 8080"}).to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(saluer))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
