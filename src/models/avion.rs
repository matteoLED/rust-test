pub struct Avion {
    immatriculation: String,
    modele: String,
    capacite: u32,
}

impl Avion {
    fn new(immatriculation: &str, modele: &str, capacite: u32) -> Avion {
        Avion {
            immatriculation: immatriculation.to_string(),
            modele: modele.to_string(),
            capacite,
        }
    }
}
