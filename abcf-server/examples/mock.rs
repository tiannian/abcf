use std::io;
use abcf_server::Server;

#[tokio::main]
async fn main() -> io::Result<()> {
    env_logger::init();
    let server = Server::bind("127.0.0.1:26658").await.unwrap();
    server.run().await.unwrap();
    Ok(())
}
