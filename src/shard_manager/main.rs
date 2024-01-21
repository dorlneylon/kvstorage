mod net;
mod tests;
mod utils;
#[path = "../common/mod.rs"] mod common;
use net::server::run;
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[clap(short, long, default_value = "127.0.0.1")]
    address: String,
    #[clap(short, long, default_value = "7080")]
    port: u16,
    #[clap(short, long, default_value = "1")]
    shards: u32,
    #[clap(short, long, default_value = "6080")]
    offset: u32,
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let conn_addr = args.address;
    let shards = args.shards;
    let offset = args.offset;

    if conn_addr.is_empty() {
        panic!("Not enough arguments. Use ./shard --help to see the usage.");
    }

    run(format!("{}:{}", conn_addr, args.port).as_str(), shards, offset).await
}