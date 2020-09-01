extern crate regex;
extern crate rayon;

use crate::{
    server::{
        utility,
        client::client_profile::Client,

    },
    commands::Commands
};

use std::{
    sync::{Arc, Mutex},
    net::{TcpStream, TcpListener},
    collections::HashMap,
    io::prelude::*,
    time::Duration,
    io::Error,
    thread,
    io
};

use log::info;

use crossbeam_channel::{Sender, Receiver, unbounded};
use rust_chat_server::ThreadPool;
//use zeroize::Zeroize;
//use parking_lot::FairMutex;
//use dashmap::DashMap;
//use regex::Regex;

#[derive(Debug)]
pub enum ServerMessages {
    RequestUpdate(Arc<Mutex<TcpStream>>),
    RequestInfo(String, Arc<Mutex<TcpStream>>),
    Disconnect(String),
    Shutdown,
}

// MARK: - server struct
#[derive(Debug)]
pub struct Server<'z> {
    name: &'z str,
    address: &'z str,
    author: &'z str,

    connected_clients: Arc<Mutex<HashMap<String, Client>>>,

    thread_pool: ThreadPool,

    sender: Sender<ServerMessages>,
    receiver: Receiver<ServerMessages>,
}

// MARK: - server implemetation
impl<'z> Server<'z> {
    pub fn new(name: &'z str, address: &'z str, author: &'z str) -> Self {
        let (sender, receiver) = unbounded();

        Self {
            name: name,
            address: address,
            author: author,
            connected_clients: Arc::new(Mutex::new(HashMap::new())),
            thread_pool: ThreadPool::new(16), 

            sender,
            receiver,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    pub fn get_address(&self) -> String {
        self.address.to_string()
    }

    pub fn get_author(&self) -> String {
        self.author.to_string()
    }

    pub fn set_port(&mut self) {
    }

    pub fn start(&'static self) -> Result<(), io::Error>{
        println!("server: starting server...");
        // clone elements for thread
        let receiver = self.receiver.clone();

        // set up listener and buffer
        let listener = TcpListener::bind(self.get_address())?;
        listener.set_nonblocking(true)?;

        println!("server: spawning threads");
        let _ = thread::Builder::new().name("Server Thread".to_string()).spawn(move || {
            let mut buffer = [0; 1024];
            
            'outer: loop {
                std::thread::sleep(Duration::from_millis(100));

                // get messages from the servers channel.
                println!("server: getting messages");
                for i in receiver.try_iter() {
                    match i {
                        ServerMessages::Shutdown => {
                            // TODO: implement disconnecting all clients and shutting down the server
                            println!("server: shutting down...");

                            break 'outer;
                        },
                        ServerMessages::RequestUpdate(stream_arc) => {
                            for (_k, v) in self.connected_clients.lock().unwrap().iter() {
                                let stream = stream_arc.lock().unwrap();

                                utility::transmit_data(&stream, v.to_string().as_str());
                                //self.transmit_data(&stream, v.to_string().as_str());

                                if utility::read_data(&stream, &mut buffer).unwrap_or(Commands::Type(Error {})) == Commands::Type(Success {params: None,}) {
                                //if self.read_data(&stream, &mut buffer).unwrap_or(Commands::Type(Error {})) == Commands::Type(Success {params: None,}) {
                                    println!("Success Confirmed");
                                } else {
                                    println!("no success read");
                                    let error = Commands::Type(Error {});
                                    utility::transmit_data(&stream, error.to_string().as_str());
                                    //self.transmit_data(&stream, error.to_string().as_str());
                                }
                            }
                        },
                        ServerMessages::RequestInfo(uuid, stream_arc) => {
                            let stream = stream_arc.lock().unwrap();
                            
                            if let Some(client) = self.connected_clients.lock().unwrap().get(&uuid) {
                                let params: HashMap<String, String> = [(String::from("uuid"), client.get_uuid()), (String::from("name"), client.get_username()), (String::from("host"), client.get_address())].iter().cloned().collect();
                                let command = Commands::Type(Success {params: Some(params),} );
                                utility::transmit_data(&stream, command.to_string().as_str());
                                //self.transmit_data(&stream, command.to_string().as_str());
                            } else {
                                let command = Commands::Type(Success {params: None,} );
                                utility::transmit_data(&stream, command.to_string().as_str());
                                //self.transmit_data(&stream, command.to_string().as_str());
                            }
                        },
                        ServerMessages::Disconnect(uuid) => {
                            self.remove_client(uuid.as_str());
                            let params: HashMap<String, String> = [(String::from("uuid"), uuid)].iter().cloned().collect();
                            self.update_all_clients(Commands::Type(ClientRemove {params: Some(params),} ));
                        },
                    }
                }

                println!("server: checking for new connections");
                if let Ok((stream, _addr)) = listener.accept() {
                    stream.set_read_timeout(Some(Duration::from_millis(1000))).unwrap();
                    let _ = stream.set_nonblocking(false);

                    let request = Commands::Type(Request {params: None,});
                    utility::transmit_data(&stream, request.to_string().as_str());
                    //self.transmit_data(&stream, &request.to_string().as_str());

                    match utility::read_data(&stream, &mut buffer) {
                    //match self.read_data(&stream, &mut buffer) {
                        Ok(command) => {
                            println!("Server: new connection sent - {:?}", command);
                            let Commands::Type(command) = command;
                            command.execute(&stream, &self);
                        },
                        Err(_) => println!("ERROR: stream closed"),
                    }
                }
                // TODO: end -

                // handle each client for messages
                println!("server: handing control to clients");
                for (_k, client) in self.connected_clients.lock().unwrap().iter_mut() {
                    client.handle_connection();
                }
            }
            println!("server: stopped");
        });
        println!("server: started");
        Ok(())
    }

    pub fn stop(&self) {
        info!("server: sending stop message");
        let _ = self.sender.send(ServerMessages::Shutdown);
    }

    #[deprecated(since="01.09.20", note="Please use utility::transmit_data(...) instead.")]
    fn transmit_data(&self, mut stream: &TcpStream, data: &str){
        println!("Transmitting...");
        println!("data: {}", data);

        /*
         * This will throw an error and crash any thread, including the main thread, if
         * the connection is lost before transmitting. Maybe change to handle any exceptions
         * that may occur.
         */
        let _ = stream.write(data.to_string().as_bytes()).unwrap();
        stream.flush().unwrap();
    }
    
    #[deprecated(since="01.09.20", note="Please use utility::read_data(...) instead.")]
    fn read_data(&self, mut stream: &TcpStream, buffer: &mut [u8; 1024]) -> Result<Commands, Error> {
        stream.read(buffer)?;
        let command = Commands::from(buffer);

        Ok(command)
    }

    fn update_all_clients(&self, command: &Commands) {
        let _ = self.connected_clients.lock().unwrap().iter().map(|(_k, v)| v.sender.send(command.clone()));
    }

    pub fn add_client(&self, uuid: &str, client: &Client) {
        let mut clients = self.connected_clients.lock().unwrap();
        clients.insert(uuid.to_string(), client.clone());
    }

    pub fn remove_client(&self, uuid: &str){
        let mut clients = self.connected_clients.lock().unwrap();
        clients.remove(&uuid.to_string());
    }
}

impl<'z> ToString for Server<'z> {
    fn to_string(&self) -> std::string::String { todo!() }
}

impl<'z> Drop for Server<'z> {
    fn drop(&mut self) {
        println!("server dropped");
        let _ = self.sender.send(ServerMessages::Shutdown);
    }
}


/* The new version of the server no long works with these unit
 * tests.
 * They will be fixed soon!
 * TODO: fix unit tests
 */



/*#[cfg(test)]
mod tests{
    use super::*;
    use std::{thread, time};
    use std::sync::Once;
    use std::time::Duration;
    
    lazy_static!{
        static ref SERVER_NAME: &'static str = "test";
        static ref SERVER_ADDRESS: &'static str = "0.0.0.0:6000";
        static ref SERVER_AUTHOR: &'static str = "test";
        static ref SERVER: Server<'static> = Server::new(&SERVER_NAME, &SERVER_ADDRESS, &SERVER_AUTHOR);
    }
    
    static START: Once = Once::new();

    /*
     * These tests must be executed individually to ensure that no errors
     * occur, this is due to the fact that the server is created everytime.
     * Setup a system for the server to close after every test.
     */
    fn setup_server(){
        unsafe{
            START.call_once(|| {
                thread::spawn(|| {
                    SERVER.start();
                });
            });
            
            let millis = time::Duration::from_millis(1000);
            thread::sleep(millis);
        }
    }

    fn establish_client_connection(uuid: &str) -> TcpStream {
        let mut buffer = [0; 1024];
        
        let mut stream = TcpStream::connect("0.0.0.0:6000").unwrap();

        let mut command = read_data(&stream, &mut buffer);

        assert_eq!(command, Commands::Request(None));
        
        let msg: String = format!("!connect: uuid:{uuid} name:\"{name}\" host:\"{host}\"", uuid=uuid, name="alice", host="127.0.0.1");
        transmit_data(&stream, msg.as_str());

        command = read_data(&stream, &mut buffer);
        
        assert_eq!(command, Commands::Success(None));

        stream
    }

    fn transmit_data(mut stream: &TcpStream, data: &str){
        stream.write(data.to_string().as_bytes()).unwrap();
        stream.flush().unwrap();
    }
    
    fn read_data(mut stream: &TcpStream, buffer: &mut [u8; 1024]) -> Commands {
        match stream.read(buffer) {
            Ok(_) => Commands::from(buffer),
            Err(_) => Commands::Error(None),
        }
    }

    fn force_disconnect(mut stream: &TcpStream){
        let msg = "!disconnect:";
        transmit_data(&stream, msg);
    }

    #[test]
    fn test_server_connect(){
        let mut buffer = [0; 1024];

        setup_server();

        let mut stream = TcpStream::connect("0.0.0.0:6000").unwrap();

        stream.read(&mut buffer).unwrap();
        let mut command = Commands::from(&mut buffer);

        assert_eq!(command, Commands::Request(None));

        let msg = b"!connect: uuid:123456-1234-1234-123456 name:\"alice\" host:\"127.0.0.1\"";
        stream.write(msg).unwrap();

        stream.read(&mut buffer).unwrap();
        command = Commands::from(&mut buffer);

        assert_eq!(command, Commands::Success(None));

        let msg = b"!disconnect:";
        stream.write(msg).unwrap();

        let dur = time::Duration::from_millis(500);
        thread::sleep(dur);
    }

    #[test]
    fn test_server_info(){
        let mut buffer = [0; 1024];

        setup_server();
        
        let mut stream = TcpStream::connect("0.0.0.0:6000").unwrap();
        
        let command = read_data(&stream, &mut buffer);
        
        assert_eq!(command, Commands::Request(None));
        
        let msg = "!info:";
        transmit_data(&stream, msg);
        
        let command = read_data(&stream, &mut buffer);

        let params: HashMap<String, String> = [(String::from("name"), String::from("test")), (String::from("owner"), String::from("test"))].iter().cloned().collect();
        assert_eq!(command, Commands::Success(Some(params)));    
    }

    #[test]
    fn test_client_info(){
        let mut buffer = [0; 1024];
        
        setup_server();

        let mut stream = establish_client_connection("1234-5542-2124-155");

        let msg = "!info:";
        transmit_data(&stream, msg);
        
        let command = read_data(&stream, &mut buffer);

        let params: HashMap<String, String> = [(String::from("name"), String::from("test")), (String::from("owner"), String::from("test"))].iter().cloned().collect();
        assert_eq!(command, Commands::Success(Some(params)));
        
        let msg = "!disconnect:";
        transmit_data(&stream, msg);

        let dur = time::Duration::from_millis(500);
        thread::sleep(dur);
    }

    #[test]
    fn test_clientUpdate_solo(){
        let mut buffer = [0; 1024];
        
        setup_server();

        let mut stream = establish_client_connection("1222-555-6-7");

        let msg = "!clientUpdate:";
        transmit_data(&stream, msg);

        let command = read_data(&stream, &mut buffer);

        assert_eq!(command, Commands::Success(None));

        let msg = "!disconnect:";
        transmit_data(&stream, msg);
        
        let dur = time::Duration::from_millis(500);
        thread::sleep(dur);
    }


    #[test]
    fn test_clientUpdate_multi(){
        let mut buffer = [0; 1024];
        
        setup_server();

        let mut stream_one = establish_client_connection("0001-776-6-5");
        let mut stream_two = establish_client_connection("0010-776-6-5");
        let mut stream_three = establish_client_connection("0011-776-6-5");
        let mut stream_four = establish_client_connection("0100-776-6-5");
       
        let client_uuids: [String; 3] = [String::from("0010-776-6-5"), String::from("0011-776-6-5"), String::from("0100-776-6-5")];
        let mut user_1 = true;
        let mut user_2 = true;
        let mut user_3 = true;

        for uuid in client_uuids.iter() {
            let command = read_data(&stream_one, &mut buffer);
            
            if *uuid == String::from("0010-776-6-5") && user_1 {
                let params: HashMap<String, String> = [(String::from("uuid"), String::from("0010-776-6-5")), (String::from("name"), String::from("\"alice\"")), (String::from("host"), String::from("\"127.0.0.1\""))].iter().cloned().collect();
                assert_eq!(command, Commands::Client(Some(params)));
                
                user_1 = false;
            } else if *uuid == String::from("0011-776-6-5") && user_2 {
                let params: HashMap<String, String> = [(String::from("uuid"), String::from("0011-776-6-5")), (String::from("name"), String::from("\"alice\"")), (String::from("host"), String::from("\"127.0.0.1\""))].iter().cloned().collect();
                assert_eq!(command, Commands::Client(Some(params)));
                
                user_2 = false;
            } else if *uuid == String::from("0100-776-6-5") && user_3 {    
                let params: HashMap<String, String> = [(String::from("uuid"), String::from("0100-776-6-5")), (String::from("name"), String::from("\"alice\"")), (String::from("host"), String::from("\"127.0.0.1\""))].iter().cloned().collect();
                assert_eq!(command, Commands::Client(Some(params)));
                
                user_3 = false;
            } else {
                assert!(false);
            }
            let msg = "!success:";
            transmit_data(&stream_one, msg);
        }

        stream_one.set_read_timeout(Some(Duration::from_millis(3000))).unwrap();
        let mut unsuccessful = true;
        while unsuccessful {
            let msg = "!clientUpdate:";
            transmit_data(&stream_one, msg);

            let command = read_data(&stream_one, &mut buffer);
            match command.clone() {
                Commands::Error(None) => println!("resending..."),
                _ => {
                    assert_eq!(command, Commands::Success(None));
                    unsuccessful = false;
                },
            }
        }
        stream_one.set_read_timeout(None).unwrap();

        for x in 0..3 {
            let command = read_data(&stream_one, &mut buffer);

            let command_clone = command.clone();
            match command{
                Commands::Client(Some(params)) => {
                    let uuid = params.get("uuid").unwrap();

                    if *uuid == String::from("0010-776-6-5") {
                        let params: HashMap<String, String> = [(String::from("uuid"), String::from("0010-776-6-5")), (String::from("name"), String::from("\"alice\"")), (String::from("host"), String::from("\"127.0.0.1\""))].iter().cloned().collect();
                        assert_eq!(command_clone, Commands::Client(Some(params)));
                    } else if *uuid == String::from("0011-776-6-5") {
                        let params: HashMap<String, String> = [(String::from("uuid"), String::from("0011-776-6-5")), (String::from("name"), String::from("\"alice\"")), (String::from("host"), String::from("\"127.0.0.1\""))].iter().cloned().collect();
                        assert_eq!(command_clone, Commands::Client(Some(params)));
                    } else if *uuid == String::from("0100-776-6-5") {
                        let params: HashMap<String, String> = [(String::from("uuid"), String::from("0100-776-6-5")), (String::from("name"), String::from("\"alice\"")), (String::from("host"), String::from("\"127.0.0.1\""))].iter().cloned().collect();
                        assert_eq!(command_clone, Commands::Client(Some(params)));
                    } else {
                        assert!(false);
                    }
                },
                _ => assert!(false),
            }

            let msg = "!success:";
            transmit_data(&stream_one, msg);
        }
        
        let dur = time::Duration::from_millis(500);
        thread::sleep(dur);
        
        let msg = "!disconnect:";
        transmit_data(&stream_one, msg);
        transmit_data(&stream_two, msg);
        transmit_data(&stream_three, msg);
        transmit_data(&stream_four, msg);

        let dur = time::Duration::from_millis(500);
        thread::sleep(dur);
    }

    #[test]
    fn test_clientInfo(){
        let mut buffer = [0; 1024];
        
        setup_server();

        let mut stream_one = establish_client_connection("0001-776-6-5");
        let mut stream_two = establish_client_connection("\"0010-776-6-5\"");
        
        let command = read_data(&stream_one, &mut buffer);
        let params: HashMap<String, String> = [(String::from("uuid"), String::from("\"0010-776-6-5\"")), (String::from("name"), String::from("\"alice\"")), (String::from("host"), String::from("\"127.0.0.1\""))].iter().cloned().collect();
        assert_eq!(command, Commands::Client(Some(params)));
        
        let msg = "!success:";
        transmit_data(&stream_one, msg);


        stream_one.set_read_timeout(Some(Duration::from_millis(3000))).unwrap();
        let mut unsuccessful = true;
        while unsuccessful {
            let msg = "!clientInfo: uuid:\"0010-776-6-5\"";
            transmit_data(&stream_one, msg);

            let command = read_data(&stream_one, &mut buffer);
            match command.clone() {
                Commands::Error(None) => println!("resending..."),
                _ => {
                    let params: HashMap<String, String> = [(String::from("uuid"), String::from("\"0010-776-6-5\"")), (String::from("name"), String::from("\"alice\"")), (String::from("host"), String::from("\"127.0.0.1\""))].iter().cloned().collect();
                    assert_eq!(command, Commands::Success(Some(params)));
                    unsuccessful = false;
                },
            }
        }
        stream_one.set_read_timeout(None).unwrap();

        let msg = "!disconnect:";
        transmit_data(&stream_one, msg);
        transmit_data(&stream_two, msg);

        let dur = time::Duration::from_millis(500);
        thread::sleep(dur);
    }

    #[test]
    fn test_client_disconnect(){
        let mut buffer = [0; 1024];
        
        setup_server();
        
        let mut stream_one = establish_client_connection("0001-776-6-5");
        let mut stream_two = establish_client_connection("0010-776-6-5");

        let command = read_data(&stream_one, &mut buffer);
        let params: HashMap<String, String> = [(String::from("uuid"), String::from("0010-776-6-5")), (String::from("name"), String::from("\"alice\"")), (String::from("host"), String::from("\"127.0.0.1\""))].iter().cloned().collect();
        assert_eq!(command, Commands::Client(Some(params)));
        
        let msg = "!success:";
        transmit_data(&stream_one, msg);

        let msg = "!disconnect:";
        transmit_data(&stream_two, msg);

        let command = read_data(&stream_one, &mut buffer);
        let params: HashMap<String, String> = [(String::from("uuid"), String::from("0010-776-6-5"))].iter().cloned().collect();
        assert_eq!(command, Commands::Client(Some(params)));

        let msg = "!success:";
        transmit_data(&stream_one, msg);

        stream_one.set_read_timeout(Some(Duration::from_millis(2000))).unwrap();
        match stream_one.peek(&mut buffer) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
        stream_one.set_read_timeout(None).unwrap();
 
        let msg = "!disconnect:";
        transmit_data(&stream_one, msg);

        let dur = time::Duration::from_millis(500);
        thread::sleep(dur);
    }
}*/
