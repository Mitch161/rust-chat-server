extern crate regex;

use std::{
    sync::Arc,
    sync::Mutex,
    net::{Shutdown, TcpStream},
    io::prelude::*,
    io::Error as IoError,
    //collections::HashMap,
    time::{Instant, Duration},
};

use crossbeam::{
    Sender,
    Receiver,
    TryRecvError,
    unbounded
};

//use zeroize::Zeroize;
use log::info;

use crate::{
    server::{
        utility,
        //server_profile::Server,
        server_profile::ServerMessages,
    },
    commands::Commands,
    commands::CommandsAPI,
    commands::GenerateFrom,
    commands::behaviors::{
        Runnables,
        Success,
        Error,
    },

};

//use parking_lot::FairMutex;
//use dashmap::DashMap;

#[derive(Debug)]
pub struct Client {
    uuid: String,
    username: String,
    address: String,

    last_heartbeat: Arc<Mutex<Instant>>,

    stream_arc: Arc<Mutex<TcpStream>>,

    pub sender: Sender<Commands>,
    receiver: Receiver<Commands>,

    server_sender: Sender<ServerMessages>,
}

impl Client {
    pub fn new(stream: TcpStream, server_sender: Sender<ServerMessages>, uuid: &str, username: &str, address: &str) -> Self {
        let (sender, receiver): (Sender<Commands>, Receiver<Commands>) = unbounded();
        stream.set_read_timeout(Some(Duration::from_secs(1))).unwrap();

        Client {
            stream_arc: Arc::new(Mutex::new(stream)),
            uuid: uuid.to_string(),
            username: username.to_string(),
            address: address.to_string(),

            sender,
            receiver,

            server_sender,

            last_heartbeat: Arc::new(Mutex::new(Instant::now())),
        }
    }

    #[allow(dead_code)]
    pub fn get_uuid(&self) -> String {
        self.uuid.clone()
    }

    #[allow(dead_code)]
    pub fn get_username(&self) -> String {
        self.username.clone()
    }

    #[allow(dead_code)]
    pub fn get_address(&self) -> String {
        self.address.clone()
    }

    pub fn get_last_heartbeat(&self) -> Arc<Mutex<Instant>> {
        self.last_heartbeat.clone()
    }

    pub fn get_server_sender(&self) -> &Sender<ServerMessages> {
        &self.server_sender
    }

    pub fn get_stream_arc(&self) -> Arc<Mutex<TcpStream>> {
        self.stream_arc.clone()
    }

    // TODO: - add heartbeat timer.
    pub fn handle_connection(&mut self) {
        let mut buffer = [0; 1024];

        // TODO: - Check heartbeat
        {
            info!("heartbeat")
        }
        
        info!("{}: handling connection", self.uuid);
        match self.read_data(&mut buffer) {
            Ok(command) => {
                // match incomming commands
                println!("command");
                command.run(&mut self.stream_arc.lock().unwrap(), &self);
            },
            Err(_) => {
                // no data was read
            },
        }

        println!("buffer");
        // test to see if there is anything for the client to receive from its channel
        match self.receiver.try_recv() {
            /*command is on the channel*/ 
            Ok(Commands::ClientRemove(Some(params))) => {
                let mut retry: u8 = 3;
                'retry_loop: loop {
                    if retry < 1 {
                        let _ = utility::transmit_data(&mut self.stream_arc.lock().unwrap(), Commands::Error.to_string().as_str());
                        break 'retry_loop;
                    } else {                    
                        let _ = utility::transmit_data(&mut self.stream_arc.lock().unwrap(), Commands::ClientRemove(Some(params.clone())).to_string().as_str());

                        if let Some(success) = self.read_data(&mut buffer).unwrap_or(Box::new(Error)).downcast_ref::<Success>() {
                            break 'retry_loop;
                        } else {
                            retry -= 1;
                        }
                    }
                }
            },
            Ok(Commands::Client(Some(params))) => {
                let mut retry: u8 = 3;
                'retry_loop: loop {
                    if retry < 1 {
                        let _ = utility::transmit_data(&mut self.stream_arc.lock().unwrap(), Commands::Error.to_string().as_str());
                        break 'retry_loop;
                    } else {
                        let _ = utility::transmit_data(&mut self.stream_arc.lock().unwrap(), Commands::Client(Some(params.clone())).to_string().as_str());
                        
                        if let Some(success) = self.read_data(&mut buffer).unwrap_or(Box::new(Error)).downcast_ref::<Success>() {
                            break 'retry_loop;
                        } else {
                            retry -= 1;
                        }
                    }
                }
            },
            /*no data available yet*/
            Err(TryRecvError::Empty) => {},
            _ => {},
        }
        println!("---Client Thread Exit---");
    }

    // move into a drop perhaps
    #[allow(dead_code)]
    pub fn disconnect(&mut self){
        self.stream_arc.lock().unwrap().shutdown(Shutdown::Both).expect("shutdown call failed");
    }

    fn read_data(&mut self, buffer: &mut [u8; 1024]) -> Result<Box<dyn Runnables<Client>>, IoError> {
        self.stream_arc.lock().unwrap().read(buffer)?;
        let command = <CommandsAPI as GenerateFrom<&mut [u8; 1024], Client>>::generate_from(buffer);

        Ok(command)
    }
}

impl ToString for Client {
    fn to_string(&self) -> std::string::String { todo!() }
}

impl Drop for Client {
    fn drop(&mut self) {
        let _ = self.stream_arc.lock().unwrap().write_all(Commands::Disconnect.to_string().as_bytes());
        let _ = self.stream_arc.lock().unwrap().shutdown(Shutdown::Both);
    }
}
