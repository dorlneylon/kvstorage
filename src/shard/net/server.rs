use std::path::Path;
use std::sync::{Arc, RwLock};
use crate::utils::log::Logger;
use std::net::SocketAddr;
use tonic::transport::Server;
use crate::common::commands::Command;
use self::rpc_query::QueryResponse;
use self::rpc_query::request_server::Request;

use super::requests::RequestHandler;
use rpc_query::{Query, request_server::RequestServer};

pub mod rpc_query {
    tonic::include_proto!("rpc_query");
}

pub struct App(Arc<RwLock<RequestHandler>>, String, String); // logpath and filepath respectively

#[tonic::async_trait]
impl Request for App {
    async fn request_query(&self, request: tonic::Request<Query>) -> Result<tonic::Response<QueryResponse>, tonic::Status> {
        let command: Command = serde_json::from_str(&request.into_inner().command).unwrap();

        let mut req = self.0.write().expect("Unable to lock");
        let res = req.process(&command);
       
        match &res {
            Err(e) => Ok(tonic::Response::new(QueryResponse { message: e.to_string() })),
            Ok(v) => {
                // since only one thread can access storage there is no need in extra lock at logger.
                let _yauv = Logger::log(&command, &res, self.1.as_str());
                let _another = Logger::store(req.get_storage().get_map(), self.2.as_str());
                Ok(tonic::Response::new(QueryResponse { message: (*v).clone() }))
            }
        }
    }
}

pub async fn run(conn_addr: &str, cache_addr: &str, filepath: &str, logpath: &str) {
    let filepath = filepath.to_owned();
    let logpath = logpath.to_owned();

    let request_handler: Arc<RwLock<RequestHandler>>;

    if Path::new(&filepath).exists() {
        request_handler = Arc::new(RwLock::new(RequestHandler::new_from(cache_addr, &filepath)));
    } else {
        request_handler = Arc::new(RwLock::new(RequestHandler::new(cache_addr)));
    }

    let req_service = App(request_handler, logpath, filepath);
    let addr: SocketAddr = conn_addr.parse().expect("Unable to parse socket address");

    Server::builder()
                .add_service(RequestServer::new(req_service))
                .serve(addr)
                .await
                .unwrap();
}
