use std::net::TcpListener;
use z2p::run;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.0:8000")?;
    run(listener)?.await
}
