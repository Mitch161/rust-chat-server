use crate::commands::Commands;
use std::net::TcpStream;
use std::io::Error;
use std::io::Write;

pub fn transmit_data(stream: &mut TcpStream, data: &str) -> Result<(), Error> {
    println!("Transmitting...");
    println!("data: {}", data);
    
    /* TODO
     * need some sort of handling for io erros if data fails to write to the stream,
     * currently all if a error happens, its stored in '_', ignoring it.
     */


    /*
     * This will throw an error and crash any thread, including the main thread, if
     * the connection is lost before transmitting. Maybe change to handle any exceptions
     * that may occur.
     */
    let _ = stream.write(data.to_string().as_bytes())?;
    stream.flush()?;
    Ok(())
}

