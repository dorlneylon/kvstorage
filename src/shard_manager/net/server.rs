use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::io::AsyncWriteExt;
use crate::utils::{distributor::CommandDistributor, command_processor::process_command};

pub mod rpc_query {
    tonic::include_proto!("rpc_query");
}

pub async fn run(conn_addr: &str, shards: u32, offset: u32) -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting master server at: {}", conn_addr);
    let listener = TcpListener::bind(conn_addr).await?;
    let command_distributor = Arc::new(
        CommandDistributor::new(shards, offset)
    );

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        let distributor = command_distributor.clone();
        
        tokio::spawn(async move {
            let headers = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\n";
            match distributor.map_command(&mut socket).await {
                Ok((cmd, addr)) => {
                    match process_command(&distributor, (cmd, addr)).await {
                        Ok(ans) => {
                            if let Err(res) = socket.write_all((headers.to_owned() + &ans).as_bytes()).await {
                                println!("Error occured: {}", res);
                            }
                            Ok(())
                        },
                        Err(ans) => {
                            if let Err(res) = socket.write_all((headers.to_owned() + &ans.to_string()).as_bytes()).await {
                                println!("Error occured: {}", res);
                            }
                            Ok(())
                        }
                    }
                },
                _ => {
                    println!("Failed to map command");
                    Err(())
                },
            }
        });
    }
}
