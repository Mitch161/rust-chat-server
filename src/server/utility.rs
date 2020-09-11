use crate::commands::Commands;
use std::net::TcpStream;
use std::io::Error;

pub fn transmit_data(stream: &mut TcpStream, data: &str) -> Result<(), Error> {
    println!("Transmitting...");
    println!("data: {}", data);

    /*
     * This will throw an error and crash any thread, including the main thread, if
     * the connection is lost before transmitting. Maybe change to handle any exceptions
     * that may occur.
     */
    let _ = stream.write(data.to_string().as_bytes())?;
    stream.flush()?;
    Ok(())
}

