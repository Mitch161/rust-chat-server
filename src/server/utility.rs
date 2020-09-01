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

pub fn read_data(mut stream: &TcpStream, buffer: &mut [u8; 1024]) -> Result<Commands, Error> {
    stream.read(buffer)?;
    let command = Commands::from(buffer);

    Ok(command)
}

