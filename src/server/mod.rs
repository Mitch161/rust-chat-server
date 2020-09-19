pub mod client;
pub mod server_profile;
pub mod utility;

use std::{
    net::TcpListener,
    sync::Arc,
    io::Error as IoError,
    thread,
};

use server_profile::Server;

pub struct ServerModel {
    server_arc: Arc<Server>,
}

impl ServerModel {
    pub fn new(server_name: &str, server_address: &str, server_email: &str) -> Self {
        Self {
            server_arc: Arc::new(Server::new(server_name, server_address, server_email)),
        }
    }

    pub fn start(&self) -> Result<(), IoError> {
        println!("server: starting server...");
        
        // set up listener and buffer
        let mut buffer = [0; 1024];
        let listener = TcpListener::bind(self.server_arc.get_address())?;
        listener.set_nonblocking(true)?;
        
        let server = self.server_arc.clone();

        println!("server: spawning threads");
        let _ = thread::Builder::new().name("Server Thread".to_string()).spawn(move || {
            while server.step(&mut buffer, &listener) {}
            
            println!("server: stopped");
        });

        println!("server: started");
        Ok(())
    }

    pub fn stop(&self) {
        self.server_arc.stop();
    }
}
