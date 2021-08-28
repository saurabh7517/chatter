use std::net::{TcpListener};
use std::io::{Read, Write};
use std::str;
fn main() {
    let hostname_and_port: &str = "127.0.0.1:8000";
    let listener = TcpListener::bind(hostname_and_port).expect("Could not bind");
    let mut buf = [0;512];
    let mut messageSentToClient;
    println!("Starting server....");
    let mut counter :usize = 0;
    println!("Waiting for client to establish connection...");
    let incoming_connections = listener.accept();
    
    match incoming_connections {
        Ok(mut conn) => {

            println!("Connection established with client :: {} ", conn.1);
            loop {
                match conn.0.read( &mut buf) {
                    Ok(datasize) => {
                        println!("Data read from stream :: {}", datasize);
                        let message =  str::from_utf8(&mut buf[0..datasize]).expect("cannot convert to utf-8").to_owned();
                        println!("Message from client :: {} :: {}",conn.1,message);
                        buf = [0;512];
                        
                        if message.contains("quit"){
                        // Disconnect client from server
                            println!("Shutting down server");
                            break;
                        }
                    },
                    Err(error) => {
                        println!("Error reading from client :: {}", error);
                        break;
                    }
                }
                messageSentToClient = format!("Data received at server :: {}", counter);
                counter += 1;

                conn.0.write(messageSentToClient.as_bytes()).expect("unable to write data to buffer");

            }
        },
        Err(error) => {
            println!("Unable to accept connection on :: {} :: with error :: {}", hostname_and_port,     error);
        }
    }
}
