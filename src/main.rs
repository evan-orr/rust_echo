use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::prelude::*;
use std::thread;

const BUFFER_SIZE: usize = 512;

fn respond_to_client(mut client_stream: TcpStream)
{
    let mut buffer = [0 as u8; BUFFER_SIZE];
    while match client_stream.read(&mut buffer){ 
        Ok(result) => {
            let result = client_stream.write(&mut buffer[0..result]);
            client_stream.flush().unwrap();
            match result{
                Err(msg) => println!("Error echoing to {}: {}", client_stream.peer_addr().unwrap(), msg),
                Ok(sent) => println!("Sent {} bytes to {}", sent, client_stream.peer_addr().unwrap())
            }
            true
        },
        Err(_) => {
            println!("Error, terminating connection with {}", client_stream.peer_addr().unwrap());
            client_stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    }{}
}

fn main() -> std::io::Result<()>{
    println!("TCP Echo Server\n===============");
    
    let listener = TcpListener::bind("0.0.0.0:7")?;

    for stream in listener.incoming(){
        match stream{
            Ok(stream)=>{
                println!("Accepting client {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    respond_to_client(stream)
                });
            },
            Err(connection_err)=>println!("Couldn't connect with client: {}", connection_err)
        }
    }

    Ok(())
}
