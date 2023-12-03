use clap::Parser;

use server::Server;

mod server;

#[tokio::main()]
async fn main() -> anyhow::Result<()> {
    let args = CLA::parse();
    let mut serv = Server::from(&args);
    serv.run().await
}

#[derive(Parser, Debug)]
struct CLA {
    #[arg(short = 's', default_value_t = 8000_u16)]
    port_pages: u16,

    #[arg(short = 'g', default_value_t = 3000_u16)]
    port_games: u16,
}
