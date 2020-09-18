pub mod client;
pub mod server_profile;
pub mod utility;

use std::{
    net::{TcpStream, TcpListener},
    io::prelude::*,
    time::Duration,
    io::Error as IoError,
    thread,
    io
};

use server_profile::Server;

pub struct ServerModel {
    server: Server,
}

impl ServerModel {
    pub fn new(server_name: &str, server_address: &str, server_email: &str) -> Self {
        Self {
            server: Server::new(server_name, server_address, server_email),
        }
    }

    pub fn start(&mut self) -> Result<(), IoError> {
        println!("server: starting server...");
        
        // set up listener and buffer
        let mut buffer = [0; 1024];
        let listener = TcpListener::bind(self.server.get_address())?;
        listener.set_nonblocking(true)?;
        
        println!("server: spawning threads");
        let _ = thread::Builder::new().name("Server Thread".to_string()).spawn(move || {
            while self.server.start(&mut buffer, &listener) {}
            
            println!("server: stopped");
        });

        println!("server: started");
        Ok(())
    }

    pub fn stop(&self) {
        self.server.stop();
    }
}
