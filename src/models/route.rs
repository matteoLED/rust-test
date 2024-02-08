pub struct Route {
    origine: String,
    destination: String,
    avion: String,
}

impl Route {
    fn new(origine: &str, destination: &str, avion: &str) -> Route {
        Route {
            origine: origine.to_string(),
            destination: destination.to_string(),
            avion: avion.to_string(),
        }
    }
}
