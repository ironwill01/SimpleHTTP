use server::core::{Config , Server};

fn main() -> Result<() , Box<dyn std::error::Error>> {
    let cfg = Config::new("127.0.0.1:8080");

    let mut server = Server::new(cfg.unwrap())?;
    
    server.start();

    Ok(())
}
