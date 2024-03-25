use tokio::net::{TcpListener, TcpStream};
use yaq_r::handshakes::*;
use bytes::Bytes;
use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::str::from_utf8;
use std::sync::{Arc, Mutex};
use apache_avro::{Schema, to_avro_datum, from_value, from_avro_datum};
use apache_avro::types::Value;

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:39000").await.unwrap();

    println!("Listening");

    loop {
        let (socket, _) = listener.accept().await.unwrap();

        println!("Accepted");
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(socket: TcpStream) -> Result<(), Box<dyn Error>> {

    let reader_schema = Schema::parse_str(HANDSHAKE_REQUEST_SCHEMA).unwrap();
    let response_schema = Schema::parse_str(HANDSHAKE_RESPONSE_SCHEMA).unwrap();
    let mut buffer = vec![0u8;1024];

    loop {
        // Wait for the socket to be readable
        socket.readable().await?;

        // Try to read data, this may still fail with `WouldBlock`
        // if the readiness event is a false positive.
        match socket.try_read(&mut buffer) {
            Ok(n) => {
                buffer.truncate(n);
                let _ = if n > 0 {
                    println!("Buffer: {buffer:?}");
                    let value = from_avro_datum(
                        &reader_schema, &mut &buffer[4..], None
                    ).expect("Failed to read buffer");
                    println!("buffer: {value:?}");
                    from_value::<HandshakeRequest>(
                        &from_avro_datum(
                            &reader_schema,
                            &mut &(to_avro_datum::<Value>(&reader_schema, value.clone().into()).unwrap())[..],
                            None
                        ).unwrap()
                    ).unwrap();
                } else {panic!("That was unexpected!")};
                println!("All Done reading Message! Time to send a response!");
                let response = HandshakeResponse{
                    match_: HandshakeMatch::BOTH,
                    server_hash: Some(from_utf8(&[32u8;16]).unwrap().to_string()),
                    server_protocol: None,
                    meta: None,
                };
                let write_result = socket.try_write(&to_avro_datum::<Value>(&response_schema, response.into()).unwrap()).unwrap();
                println!("write_result = {write_result:?}");
                continue;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                println!("We ran into a WouldBlock error");
                break;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    // Write the response to the client
    Ok(())
}