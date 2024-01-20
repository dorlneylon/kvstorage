mod utils;
mod storage;
mod cache;
mod net;
mod tests;
#[path = "../common/mod.rs"] mod common;
use net::server::run;
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[clap(short, long, default_value = "127.0.0.1")]
    address: String,
    #[clap(short, long, default_value = "6080")]
    port: u16,
    #[clap(short, long, default_value = "memcache://localhost:11211?timeout=10&tcp_nodelay=true")]
    memcached_addr: String,
    #[clap(short, long, default_value = "./shard.json")]
    filepath: String,
    #[clap(short, long, default_value = "./server.log")]
    logpath: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let conn_addr = args.address;
    let port = args.port;
    let cache_addr = args.memcached_addr;
    let filepath = args.filepath;
    let logpath = args.logpath;

    if vec![&conn_addr, &cache_addr, &filepath, &logpath].iter().any(|&x| x.is_empty()) || port == 0 {
        panic!("Not enough arguments. Use ./shard --help to see the usage.");
    }

    if !conn_addr.is_empty() {
        run(format!("{}:{}", conn_addr, port).as_str(), &cache_addr, &filepath, &logpath).await;
    }
}

