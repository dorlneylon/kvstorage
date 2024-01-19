mod net;
mod cache;
mod utils;
mod tests;
mod storage;
use net::server::run;

fn main() {
    let settings = config::Config::builder()
        .add_source(config::File::with_name("config.toml"))
        .build()
        .unwrap_or_else(|_| panic!("Failed to read config.toml"));

    let conn_addr = settings.get::<String>("env.ADDRESS").unwrap_or_else(|_| panic!("ADDRESS not found in config.toml"));
    let cache_addr = settings.get::<String>("env.CLIENT").unwrap_or_else(|_| panic!("CLIENT not found in config.toml"));
    let filepath = settings.get::<String>("env.FILEPATH").unwrap_or_else(|_| panic!("FILEPATH not found in config.toml"));
    let logpath = settings.get::<String>("env.LOGPATH").unwrap_or_else(|_| panic!("LOGPATH not found in config.toml"));

    if !conn_addr.is_empty() {
        run(&conn_addr, &cache_addr, &filepath, &logpath);
    }
}

