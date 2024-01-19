use std::io::{Read, Error};
use std::net::{TcpStream, TcpListener};
use std::sync::{Arc, RwLock};
use crate::utils::commands::Command;
use crate::utils::log::{log, store};
use super::requests::RequestHandler;
use super::pool::ThreadPool;
use std::io::Write;
use std::path::Path;

pub fn run(conn_addr: &str, cache_addr: &str, filepath: &str, logpath: &str) {
    let filepath = filepath.to_owned();
    let logpath = logpath.to_owned();

    let listener = TcpListener::bind(conn_addr).map_err(|_| panic!("Failed to bind to address")).unwrap();
    let request_handler: Arc<RwLock<RequestHandler>>;

    if Path::new(&filepath).exists() {
        request_handler = Arc::new(RwLock::new(RequestHandler::new_from(cache_addr, &filepath)));
    } else {
        request_handler = Arc::new(RwLock::new(RequestHandler::new(cache_addr)));
    }

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let req = Arc::clone(&request_handler);
        let filepath_clone = filepath.clone();
        let logpath_clone = logpath.clone();
        pool.execute(move || {
            if let Ok(mut stream) = stream {
                let headers = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\n";
                let ans = match map_command(&mut stream) {
                    Ok(command) => {
                        let mut locked = req.write().expect("Unable to lock");
                        let res = locked.process(&command);
                        let _yauv = log(&command, &res, &logpath_clone);
                        let _another = store(locked.get_storage().get_map(), &filepath_clone);
                        match res {
                            Err(e) => e.to_string(),
                            Ok(v) => v
                        }
                    },
                    Err(e) => {
                        println!("Failed to map command: {}", e);
                        e.to_string()
                    },
                };
                match stream.write_all((headers.to_owned() + &ans).as_bytes()) {
                    Ok(_) => println!("Response sent to client"),
                    Err(e) => println!("Failed to send response: {}", e),
                }
            }
        });
    }
}

fn map_command(stream: &mut TcpStream) -> Result<Command, Error> {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(_) => {
            println!("Request:\n{}\n\n", String::from_utf8_lossy(&buffer[..]));
            let request_str = String::from_utf8_lossy(&buffer[..]);
            let mut body = "";
            if let Some(pos) = request_str.find("\r\n\r\n") {
                let end = request_str.find("\0").unwrap_or(request_str.len());
                body = &request_str[pos + 4..end];
            }
            println!("Body:\n{}\n", body);
            Ok(Command::new(&body))
        },
        Err(e) => Err(e),
    }
}
