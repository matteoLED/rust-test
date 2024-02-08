use serde::Serialize;

#[derive(Serialize)]
pub struct Aeroport {
    pub code: String,
    pub nom: String,
    pub localisation: String,
}

impl Aeroport {
    pub fn new(code: &str, nom: &str, localisation: &str) -> Aeroport {
        Aeroport {
            code: code.to_string(),
            nom: nom.to_string(),
            localisation: localisation.to_string(),
        }
    }
}
