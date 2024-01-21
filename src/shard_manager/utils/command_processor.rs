use std::io::Error;
use crate::{common::commands::Command, net::server::rpc_query::{request_client::RequestClient, Query}};
use super::distributor::CommandDistributor;
use async_recursion::async_recursion;

#[async_recursion]
pub async fn process_command(distributor: &CommandDistributor, (cmd, addr): (Command, String)) -> Result<String, Error> {
    let client = RequestClient::connect(addr.clone()).await.map_err(|e| {
        let ans = format!("Failed to connect to client at {}: {}", addr, e);
        println!("The client is unavailable at the moment: {}", ans);
        Error::new(std::io::ErrorKind::AddrNotAvailable, ans)
    });

    if client.is_err() {
        return Ok(format!("The client is unavailable at the moment: {}", client.unwrap_err()));
    }
    let mut client = client.unwrap();

    let _ = client.ping(tonic::Request::new(Query { command: "".to_string() })).await.map_err(|e| {
        let ans = format!("Failed to ping the client at {}: {}.", addr, e);
        println!("The client is unavailable at the moment: {}", ans);
        e
    }).unwrap().into_inner().message;

    match cmd {
        Command::Clear() => {
            for client in distributor.offset..distributor.offset+distributor.num_shards {
                let addr = format!("http://{}:{}", &addr.split(':').nth(1).unwrap()[2..], client);
        
                let serialized_cmd = match distributor.serialize_cmd(Command::Clear()).await {
                    Ok(s) => s,
                    Err(e) => {
                        let ans = format!("Failed to serialize command: {}", e);
                        println!("{}", ans);
                        return Err(e);
                    }
                };

                let mut client = RequestClient::connect(addr.clone()).await.map_err(|e| {
                    let ans = format!("Failed to connect to client at {}: {}", addr, e);
                    println!("{}", ans);
                    Error::new(std::io::ErrorKind::AddrNotAvailable, ans)
                }).unwrap();
        
                match send_command_to_client(&mut client, serialized_cmd).await {
                    Ok(_) => {},
                    Err(e) => return Err(e)
                };
            }
        
            Ok("OK".to_string())
        },
        Command::Transact(commands) => {
            let mut tasks: Vec<(Command, String)> = vec![];
            let mut ans = "".to_string();

            for cmd in commands {
                let client = distributor.which(cmd.get_key()) + distributor.offset;
                let addr = format!("http://{}:{}", &addr.split(':').nth(1).unwrap()[2..], client);
                tasks.push((cmd, addr));
            }
            
            for (idx, cmd) in tasks.iter().enumerate() {
                match process_command(distributor, (*cmd).clone()).await {
                    Ok(res) => {
                        ans += &res;
                        ans += "\n";
                    },
                    Err(e) => {
                        println!("Failed to process transaction: {}", e);

                        for i in 0..idx {
                            let client = distributor.which(tasks[i].0.get_key()) + distributor.offset;
                            let addr = format!("http://{}:{}", addr.split(':').nth(1).unwrap(), client);

                            let val = process_command(distributor, (Command::Rollback(tasks[i].0.get_key(), 1), addr.clone())).await.unwrap();
                            let _ = process_command(distributor, (Command::Set(tasks[i].0.get_key(), val), addr));
                        }

                        return Err(e);
                    }
                }
            }

            Ok(ans)
        },
        Command::Unknown(_) => {
            Ok("Unknown command".to_string())
        },
        _ => {
            let serialized_cmd = match distributor.serialize_cmd(cmd.clone()).await {
                Ok(s) => s,
                Err(e) => {
                    let ans = format!("Failed to serialize command: {}", e);
                    println!("{}", ans);
                    return Err(e);
                }
            };

            send_command_to_client(&mut client, serialized_cmd).await
        }
    }
}

async fn send_command_to_client(client: &mut RequestClient<tonic::transport::Channel>, serialized_cmd: String) -> Result<String, Error> {
    let request = tonic::Request::new(Query { command: serialized_cmd });

    let response = client.request_query(request).await.map_err(|e| {
        let ans = format!("Failed to send command to client at {}", e);
        println!("{}", ans);
        e
    }).unwrap().into_inner().message;

    Ok(response)
}
