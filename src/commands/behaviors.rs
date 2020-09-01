use crate::{
    server::{
        utility,
        server_profile::Server,
        client::client_profile::Client,
    },
    commands::Commands
};

use std::{
    collections::HashMap,
    net::TcpStream
};


struct Request {}

struct Info {}

struct HeartBeat {
    params: Option<HashMap<String, String>>,
}

struct Connect {
    params: Option<HashMap<String, String>>,
}

struct Disconnect {}

struct ClientUpdate {}

struct ClientInfo {
    params: Option<HashMap<String, String>>,
}

struct ClientRemove {
    params: Option<HashMap<String, String>>,
}

struct Client {
    params: Option<HashMap<String, String>>,
}

struct Success {
    params: Option<HashMap<String, String>>,
}

struct Error {}





pub trait Runnables<T> {
    fn execute(&self, stream: &TcpStream, input: T);
}

trait ParameterControl {
    fn get_params(&self) -> Option<HashMap<String, String>>;
}






impl Runnables<&Client> for Request {
    fn execute(&self, stream: &TcpStream, _input: &Client) {
        println!("Server: Cannot execute Request command");
        utility::transmit_data(stream, Commands::Error(Error {}).to_string().as_str());
    }
}

impl Runnables<&Server> for Request {
    fn execute(&self, stream: &TcpStream, _input: &Server) {
        println!("Server: Cannot execute Request command");
        utility::transmit_data(stream, Commands::Error(Error {}).to_string().as_str());
    }
}

impl Runnables<&Server> for Request {
    fn execute(&self, stream: &TcpStream, _input: &mut [u8; 1024]) {
        println!("Server: Cannot execute Request command");
        utility::transmit_data(stream, Commands::Error(Error {}).to_string().as_str());
    }
}


impl Runnables<&Client> for Info {
    fn execute(&self, stream: &TcpStream, _input: &Client) {
        println!("Server: Invalid command sent");
        utility::transmit_data(stream, Commands::Error(Error {}).to_string().as_str());
    }
}

impl Runnables<&Server> for Info {
    fn execute(&self, stream: &TcpStream, input: &Server) {
        println!("Server: info requested");

        let params: HashMap<String, String> = [(String::from("name"), input.get_name()), (String::from("owner"), input.get_author())].iter().cloned().collect();
        let command = Commands::Info( Info {params: Some(params),} );

        utility::transmit_data(stream, command.to_string().as_str());
    }
}

impl Runnables<&mut [u8; 1024]> for Info {
    fn execute(&self, stream: &TcpStream, _input: &mut [u8; 1024]) {
        println!("Server: Invalid command sent");
        utility::transmit_data(stream, Commands::Error(Error {}).to_string().as_str());
    }
}


impl Runnables<&Client> for HeartBeat {
    fn execute(&self, stream: &TcpStream, input: &Client) {
        *input.get_last_heartbeat().lock().unwrap() = Instant::now();
        utility::transmit_data(Commands::Type(Success {params: None,}).to_string().as_str());
    }
}

impl Runnables<&Server> for HeartBeat {
    fn execute(&self, stream: &TcpStream, _input: &Server) {
        println!("Server: Invalid command sent");
        utility::transmit_data(stream, Commands::Error(Error {}).to_string().as_str());
    }
}

impl Runnables<&mut [u8; 1024]> for HeartBeat {
    fn execute(&self, stream: &TcpStream, _input: &mut [u8; 1024]) {
        println!("Server: Invalid command sent");
        utility::transmit_data(stream, Commands::Error(Error {}).to_string().as_str());
    }
}


impl Runnables<&Client> for Connect {
    fn execute(&self, stream: &TcpStream, _input: &Client) {
        println!("Server: Invalid command sent");
        utility::transmit_data(stream, Commands::Error(Error {}).to_string().as_str());
    }
}

impl Runnables<&Server> for Connect {
    fn execute(&self, stream: &TcpStream, input: &Server) {
        let map = self.params.unwrap();

        let mut uuid = map.get("uuid");
        let mut username = map.get("name");
        let mut address = map.get("host");

        if uuid.is_some() && username.is_some() && address.is_some() {
            uuid = uuid.unwrap();
            username = username.unwrap();
            address = address.unwrap();

            println!("{}", format!("Server: new Client connection: _addr = {}", address ));
            
            let client = Client::new(stream, input.get_sender().clone(), uuid, username, address);

            input.add_client(uuid.as_str(), &client);

            let params: HashMap<String, String> = [(String::from("name"), username.clone()), (String::from("host"), address.clone()), (String::from("uuid"), uuid.clone())].iter().cloned().collect();
            let new_client = Commands::Client( Client {params: Some(params)} );

            input.update_all_clients(&new_client);
        } else {
            println!("Server: Invalid command sent");
            utility::transmit_data(stream, Commands::Error(Error {}).to_string().as_str());
        }
    }
}

impl Runnables<&mut [u8; 1024]> for Connect {
    fn execute(&self, stream: &TcpStream, _input: &mut [u8; 1024]) {
        println!("Server: Invalid command sent");
        utility::transmit_data(stream, Commands::Error(Error {}).to_string().as_str());
    }
}







impl Runnables<&Client> for Disconnect {
    fn execute(&self, _stream: &TcpStream, input: &Client) {
        input.get_server_sender().send(ServerMessages::Disconnect(input.get_uuid())).expect("sending message to server failed");
        input.get_stream_arc().lock().unwrap().shutdown(Shutdown::Both).expect("shutdown call failed");
    }
}

impl Runnables<&Server> for Disconnect {
    fn execute(&self, stream: &TcpStream, _input: &Server) {
        println!("Server: Invalid command sent");
        utility::transmit_data(stream, Commands::Error(Error {}).to_string().as_str());
    }
}

impl Runnables<&mut [u8; 1024]> for Disconnect {
    fn execute(&self, stream: &TcpStream, _input: &mut [u8; 1024]) {
        println!("Server: Invalid command sent");
        utility::transmit_data(stream, Commands::Error(Error {}).to_string().as_str());
    }
}


impl Runnables<&Client> for ClientUpdate {
    fn execute(&self, stream: &TcpStream, input: &Client) {
        utility::transmit_data(stream, Commands::Success(Success {params: None,}).to_string().as_str());
        let _ = input.get_server_sender().send(ServerMessages::RequestUpdate(input.get_stream_arc().clone()));
    }
}

impl Runnables<&Server> for ClientUpdate {
    fn execute(&self, stream: &TcpStream, _input: &Server) {
        println!("Server: Invalid command sent");
        utility::transmit_data(stream, Commands::Error(Error {}).to_string().as_str());
    }
}

impl Runnables<&mut [u8; 1024]> for ClientUpdate {
    fn run(&self, stream: &TcpStream, _input: &mut [u8; 1024]) {
        println!("Server: Invalid command sent");
        utility::transmit_data(stream, Commands::Error(Error {}).to_string().as_str());
    }
}


impl Runnables<&Client> for ClientInfo {
    fn execute(&self, stream: &TcpStream, input: &Client) {
        let map = self.params.unwrap();

        let mut uuid = map.get("uuid");
        if uuid.is_some() {
            uuid = uuid.unwrap();
            let _ = input.get_server_sender().send(ServerMessages::RequestInfo(uuid.clone(), input.get_stream_arc().clone()));
        } else {
            println!("Server: Invalid command sent");
            utility::transmit_data(stream, Commands::Error(Error {}).to_string().as_str());
        }
    }
}

impl Runnables<&Server> for ClientInfo {
    fn execute(&self, stream: &TcpStream, _input: &Server) {
        println!("Server: Invalid command sent");
        utility::transmit_data(stream, Commands::Error(Error {}).to_string().as_str());
    }
}

impl Runnables<&mut [u8; 1024]> for ClientInfo {
    fn run(&self, stream: &TcpStream, _input: &mut [u8; 1024]) {
        println!("Server: Invalid command sent");
        utility::transmit_data(stream, Commands::Error(Error {}).to_string().as_str());
    }
}






impl Runnables<&Client> for ClientRemove {
    fn execute(&self, stream: &TcpStream, _input: &Client) {
        println!("Server: Invalid command sent");
        utility::transmit_data(stream, Commands::Error(Error {}).to_string().as_str());
    }
}

impl Runnables<&Server> for ClientRemove {
    fn execute(&self, stream: &TcpStream, _input: &Server) {
        println!("Server: Invalid command sent");
        utility::transmit_data(stream, Commands::Error(Error {}).to_string().as_str());
    }
}

impl Runnables<&mut [u8; 1024]> for ClientRemove {
    fn execute(&self, stream: &TcpStream, input: &mut [u8; 1024]) {
        let mut retry: u8 = 3;
        'retry_loop: loop {
            if retry < 1 {
                utility::transmit_data(stream, Commands::Error(Error {}).to_string().as_str());
                break 'retry_loop;
            } else {                    
                utility::transmit_data(stream, Commands::ClientRemove(ClientRemove {params: self.params,}).to_string().as_str());

                if utility::read_data(stream, input).unwrap_or(Commands::Error(Error {})) == Commands::Success(Success {params: None,}) {
                    break 'retry_loop;
                } else {
                    retry -= 1;
                }
            }
        }
    }
}


impl Runnables<&Client> for Client {
    fn execute(&self, stream: &TcpStream, _input: &Client) {
        println!("Server: Invalid command sent");
        utility::transmit_data(stream, Commands::Error(Error {}).to_string().as_str());
    }
}

impl Runnables<&Server> for Client {
    fn execute(&self, stream: &TcpStream, _input: &Server) {
        println!("Server: Invalid command sent");
        utility::transmit_data(stream, Commands::Error(Error {}).to_string().as_str());
    }
}

impl Runnables<&mut [u8; 1024]> for Client {
    fn execute(&self, stream: &TcpStream, input: &mut [u8; 1024]) {
        let mut retry: u8 = 3;
        'retry_loop: loop {
            if retry < 1 {
                utility::transmit_data(stream, Commands::Error(Error {}).to_string().as_str());
                break 'retry_loop;
            } else {
                utility::transmit_data(stream, Commands::Client(Client {params: self.params,}).to_string().as_str());
                
                if utility::read_data(stream, input).unwrap_or(Commands::Error(Error {})) == Commands::Success(Success {params: None}) {
                    break 'retry_loop;
                } else {
                    retry -= 1;
                }
            }
        }
    }
}


impl Runnables<&Client> for Success {
    fn execute(&self, stream: &TcpStream, _input: &Client) {
        println!("Server: Cannot execute Success command");
        utility::transmit_data(stream, Commands::Error(Error {}).to_string().as_str());
    }
}

impl Runnables<&Server> for Success {
    fn execute(&self, stream: &TcpStream, _input: &Server) {
        println!("Server: Cannot execute Success command");
        utility::transmit_data(stream, Commands::Error(Error {}).to_string().as_str());
    }
}

impl Runnables<&mut [u8; 1024]> for Success {
    fn execute(&self, stream: &TcpStream, _input: &mut [u8; 1024]) {
        println!("Server: Cannot execute Success command");
        utility::transmit_data(stream, Commands::Error(Error {}).to_string().as_str());
    }
}


impl Runnables<&Client> for Error {
    fn execute(&self, stream: &TcpStream, _input: &Client) {
        println!("Server: Cannot execute Success command");
        utility::transmit_data(stream, Commands::Error(Error {}).to_string().as_str());
    }
}

impl Runnables<&Server> for Error {
    fn execute(&self, stream: &TcpStream, _input: &Server) {
        println!("Server: Cannot execute Success command");
        utility::transmit_data(stream, Commands::Error(Error {}).to_string().as_str());
    }
}

impl Runnables<&mut [u8; 1024]> for Error {
    fn execute(&self, stream: &TcpStream, _input: &mut [u8; 1024]) {
        println!("Server: Cannot execute Success command");
        utility::transmit_data(stream, Commands::Error(Error {}).to_string().as_str());
    }
}







impl PartialEq for Request {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl PartialEq for Info {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl PartialEq for HeartBeat {
    fn eq(&self, other: &Self) -> bool {
        match (self.params, other.get_params()) {
            (None, Some(_other_params)) => false,
            (Some(_params), None) => false,
            (None, None) => true,
            (Some(params), Some(other_params)) => {
                let mut result = false;
                
                if params.len() == other_params.len() {
                    for (key, value) in params.iter() {
                        if let Some(other_value) = other_params.get(key) {
                            if value != other_value {
                                result = false;
                                break;
                            } else {
                                result = true;
                            }
                        }
                    }
                }

                result
            },
        }
    }
}

impl PartialEq for Connect {
    fn eq(&self, other: &Self) -> bool {
        match (self.params, other.get_params()) {
            (None, Some(_other_params)) => false,
            (Some(_params), None) => false,
            (None, None) => true,
            (Some(params), Some(other_params)) => {
                let mut result = false;
                
                if params.len() == other_params.len() {
                    for (key, value) in params.iter() {
                        if let Some(other_value) = other_params.get(key) {
                            if value != other_value {
                                result = false;
                                break;
                            } else {
                                result = true;
                            }
                        }
                    }
                }

                result
            },
        }
    }
}

impl PartialEq for Disconnect {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl PartialEq for ClientUpdate {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl PartialEq for ClientInfo {
    fn eq(&self, other: &Self) -> bool {
        match (self.params, other.get_params()) {
            (None, Some(_other_params)) => false,
            (Some(_params), None) => false,
            (None, None) => true,
            (Some(params), Some(other_params)) => {
                let mut result = false;
                
                if params.len() == other_params.len() {
                    for (key, value) in params.iter() {
                        if let Some(other_value) = other_params.get(key) {
                            if value != other_value {
                                result = false;
                                break;
                            } else {
                                result = true;
                            }
                        }
                    }
                }

                result
            },
        }
    }
}

impl PartialEq for ClientRemove {
    fn eq(&self, other: &Self) -> bool {
        match (self.params, other.get_params()) {
            (None, Some(_other_params)) => false,
            (Some(_params), None) => false,
            (None, None) => true,
            (Some(params), Some(other_params)) => {
                let mut result = false;
                
                if params.len() == other_params.len() {
                    for (key, value) in params.iter() {
                        if let Some(other_value) = other_params.get(key) {
                            if value != other_value {
                                result = false;
                                break;
                            } else {
                                result = true;
                            }
                        }
                    }
                }

                result
            },
        }
    }
}

impl PartialEq for Client {
    fn eq(&self, other: &Self) -> bool {
        match (self.params, other.get_params()) {
            (None, Some(_other_params)) => false,
            (Some(_params), None) => false,
            (None, None) => true,
            (Some(params), Some(other_params)) => {
                let mut result = false;
                
                if params.len() == other_params.len() {
                    for (key, value) in params.iter() {
                        if let Some(other_value) = other_params.get(key) {
                            if value != other_value {
                                result = false;
                                break;
                            } else {
                                result = true;
                            }
                        }
                    }
                }

                result
            },
        }
    }
}

impl PartialEq for Success {
    fn eq(&self, other: &Self) -> bool {
        match (self.params, other.get_params()) {
            (None, Some(_other_params)) => false,
            (Some(_params), None) => false,
            (None, None) => true,
            (Some(params), Some(other_params)) => {
                let mut result = false;
                
                if params.len() == other_params.len() {
                    for (key, value) in params.iter() {
                        if let Some(other_value) = other_params.get(key) {
                            if value != other_value {
                                result = false;
                                break;
                            } else {
                                result = true;
                            }
                        }
                    }
                }

                result
            },
        }
    }
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}





impl ToString for Request {
    fn to_string(&self) -> std::string::String {
        String::from("!request:")
    }
}

impl ToString for Info {
    fn to_string(&self) -> std::string::String {
        String::from("!info:")
    }
}

impl ToString for HeartBeat {
    fn to_string(&self) -> std::string::String {
        let mut out_string = String::from("!heartbeat:");

        if self.params.is_some(`) {
            let hash_map = self.params.borrow().as_ref().unwrap();
            for (k, v) in hash_map.iter() {
                out_string.push_str(" ");
                out_string.push_str(k.as_str());
                out_string.push_str(":");

                if v.contains(":") {
                    out_string.push_str(format!("\"{}\"",v.as_str()).as_str())
                } else {
                    out_string.push_str(v.as_str());
                }
            }
        }
        out_string
    }
}

impl ToString for Connect {
    fn to_string(&self) -> std::string::String {
        let mut out_string = String::from("!connect:");

        if self.params.is_some(`) {
            let hash_map = self.params.borrow().as_ref().unwrap();
            for (k, v) in hash_map.iter() {
                out_string.push_str(" ");
                out_string.push_str(k.as_str());
                out_string.push_str(":");

                if v.contains(":") {
                    out_string.push_str(format!("\"{}\"",v.as_str()).as_str())
                } else {
                    out_string.push_str(v.as_str());
                }
            }
        }
        out_string
    }
}

impl ToString for Disconnect {
    fn to_string(&self) -> std::string::String {
        String::from("!disconnect:")
    }
}

impl ToString for ClientUpdate {
    fn to_string(&self) -> std::string::String {
        String::from("!clientUpdate:")
    }
}

impl ToString for ClientInfo {
    fn to_string(&self) -> std::string::String {
        let mut out_string = String::from("!clientInfo:");

        if self.params.is_some(`) {
            let hash_map = self.params.borrow().as_ref().unwrap();
            for (k, v) in hash_map.iter() {
                out_string.push_str(" ");
                out_string.push_str(k.as_str());
                out_string.push_str(":");

                if v.contains(":") {
                    out_string.push_str(format!("\"{}\"",v.as_str()).as_str())
                } else {
                    out_string.push_str(v.as_str());
                }
            }
        }
        out_string
    }
}

impl ToString for ClientRemove {
    fn to_string(&self) -> std::string::String {
        let mut out_string = String::from("!clientRemove:");

        if self.params.is_some(`) {
            let hash_map = self.params.borrow().as_ref().unwrap();
            for (k, v) in hash_map.iter() {
                out_string.push_str(" ");
                out_string.push_str(k.as_str());
                out_string.push_str(":");

                if v.contains(":") {
                    out_string.push_str(format!("\"{}\"",v.as_str()).as_str())
                } else {
                    out_string.push_str(v.as_str());
                }
            }
        }
        out_string
    }
}

impl ToString for Client {
    fn to_string(&self) -> std::string::String {
        let mut out_string = String::from("!client:");

        if self.params.is_some(`) {
            let hash_map = self.params.borrow().as_ref().unwrap();
            for (k, v) in hash_map.iter() {
                out_string.push_str(" ");
                out_string.push_str(k.as_str());
                out_string.push_str(":");

                if v.contains(":") {
                    out_string.push_str(format!("\"{}\"",v.as_str()).as_str())
                } else {
                    out_string.push_str(v.as_str());
                }
            }
        }
        out_string
    }
}

impl ToString for Success {
    fn to_string(&self) -> std::string::String {
        let mut out_string = String::from("!success:");

        if self.params.is_some(`) {
            let hash_map = self.params.borrow().as_ref().unwrap();
            for (k, v) in hash_map.iter() {
                out_string.push_str(" ");
                out_string.push_str(k.as_str());
                out_string.push_str(":");

                if v.contains(":") {
                    out_string.push_str(format!("\"{}\"",v.as_str()).as_str())
                } else {
                    out_string.push_str(v.as_str());
                }
            }
        }
        out_string
    }
}

impl ToString for Error {
    fn to_string(&self) -> std::string::String {
        let mut out_string = String::from("!error:")
    }
}


impl ParameterControl for HeartBeat {
    fn get_params(&self) -> Option<HashMap<String, String>> {
        self.params
    }
}

impl ParameterControl for Connect {
    fn get_params(&self) -> Option<HashMap<String, String>> {
        self.params
    }

}

impl ParameterControl for ClientInfo {
    fn get_params(&self) -> Option<HashMap<String, String>> {
        self.params
    }
}

impl ParameterControl for ClientRemove {
    fn get_params(&self) -> Option<HashMap<String, String>> {
        self.params
    }
}

impl ParameterControl for Client {
    fn get_params(&self) -> Option<HashMap<String, String>> {
        self.params
    }
}

impl ParameterControl for Success {
    fn get_params(&self) -> Option<HashMap<String, String>> {
        self.params
    }
}
