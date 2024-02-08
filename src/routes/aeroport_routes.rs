use actix_web::{HttpResponse, Responder};
use crate::models::aeroport::Aeroport;


pub async fn get_aeroport() -> impl Responder {

    let aeroports = vec![
        Aeroport {
            code: "CDG".to_string(),
            nom: "Charles de Gaulle".to_string(),
            localisation: "Paris, France".to_string(),
        },
        Aeroport {
            code: "JFK".to_string(),
            nom: "John F. Kennedy International Airport".to_string(),
            localisation: "New York, USA".to_string(),
        },
    ];


    HttpResponse::Ok().json(aeroports)
}
