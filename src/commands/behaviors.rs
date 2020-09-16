use crate::{
    server::{
        utility,
        server_profile::Server,
        server_profile::ServerMessages,
        client::client_profile::Client as ClientProfile,
    },
    commands::Commands
};

use std::{
    collections::HashMap,
    net::{Shutdown, TcpStream},
    time::Instant,
    borrow::Borrow,
};

use downcast_rs::Downcast;

pub struct Request;

pub struct Info;

pub struct HeartBeat {
    params: Option<HashMap<String, String>>,
}

pub struct Connect {
    params: Option<HashMap<String, String>>,
}

pub struct Disconnect;

pub struct ClientUpdate;

pub struct ClientInfo {
    params: Option<HashMap<String, String>>,
}

pub struct ClientRemove {
    params: Option<HashMap<String, String>>,
}

pub struct Client {
    params: Option<HashMap<String, String>>,
}

pub struct Success {
    params: Option<HashMap<String, String>>,
}

pub struct Error;





pub trait Runnables<T>: Downcast {
    fn run(&self, stream: &mut TcpStream, _input: &T) {
        println!("Server: Invalid Command");
        utility::transmit_data(stream, Commands::Error.to_string().as_str());
    }

    fn to_string(&self) -> String {
        self.to_string()
    }
}
downcast_rs::impl_downcast!(Runnables<T>);


trait ParameterControl {
    fn get_params(&self) -> Option<HashMap<String, String>>;
}




/*
 * Commands recieved from user within server struct
 */
impl Runnables<Server> for Request {}

impl Runnables<Server> for Info {
    fn run(&self, stream: &mut TcpStream, input: &Server) {
        println!("Server: info requested");

        let params: HashMap<String, String> = [(String::from("name"), input.get_name()), (String::from("owner"), input.get_author())].iter().cloned().collect();
        let command = Commands::Success(Some(params));

        utility::transmit_data(stream, command.to_string().as_str());
    }
}

impl Runnables<Server> for HeartBeat {}

impl Runnables<Server> for Connect {
    fn run(&self, stream: &mut TcpStream, input: &Server) {
        let map = self.params.unwrap();

        let uuid = map.get("uuid");
        let username = map.get("name");
        let address = map.get("host");

        //if uuid.is_some() && username.is_some() && address.is_some() {
        match (uuid, username, address) {
            (Some(uuid), Some(username), Some(address)) => {
                println!("{}", format!("Server: new Client connection: _addr = {:?}", address ));
                
                let client = ClientProfile::new(stream, input.get_sender(), uuid, username, address);

                input.add_client(uuid.as_str(), &client);

                let params: HashMap<String, String> = [(String::from("name"), username.clone()), (String::from("host"), address.clone()), (String::from("uuid"), uuid.clone())].iter().cloned().collect();
                let new_client = Commands::Client(Some(params));

                input.update_all_clients(&new_client);
            },
            _ => {
                println!("Server: Invalid command sent");
                utility::transmit_data(stream, Commands::Error.to_string().as_str());
            },
        }
    }
}

impl Runnables<Server> for Disconnect {}

impl Runnables<Server> for ClientUpdate {}

impl Runnables<Server> for ClientInfo {}

impl Runnables<Server> for ClientRemove {}

impl Runnables<Server> for Client {}

impl Runnables<Server> for Success {}

impl Runnables<Server> for Error {}




/*
 * Commands recieved from user within client struct
 */
impl Runnables<ClientProfile> for Request {}

impl Runnables<ClientProfile> for Info {}

impl Runnables<ClientProfile> for HeartBeat {
    fn run(&self, stream: &mut TcpStream, input: &ClientProfile) {
        *input.get_last_heartbeat().lock().unwrap() = Instant::now();
        utility::transmit_data(stream, Commands::Success(None).to_string().as_str());
    }
}

impl Runnables<ClientProfile> for Connect {}

impl Runnables<ClientProfile> for Disconnect {
    fn run(&self, _stream: &mut TcpStream, input: &ClientProfile) {
        input.get_server_sender().send(ServerMessages::Disconnect(input.get_uuid())).expect("sending message to server failed");
        input.get_stream_arc().lock().unwrap().shutdown(Shutdown::Both).expect("shutdown call failed");
    }
}

impl Runnables<ClientProfile> for ClientUpdate {
    fn run(&self, stream: &mut TcpStream, input: &ClientProfile) {
        utility::transmit_data(stream, Commands::Success(None).to_string().as_str());
        let _ = input.get_server_sender().send(ServerMessages::RequestUpdate(input.get_stream_arc().clone()));
    }
}

impl Runnables<ClientProfile> for ClientInfo {
    fn run(&self, stream: &mut TcpStream, input: &ClientProfile) {
        let map = self.params.unwrap();

        let uuid = map.get("uuid");

        match uuid {
            Some(uuid) => {
                let _ = input.get_server_sender().send(ServerMessages::RequestInfo(uuid.clone(), input.get_stream_arc()));
            },
            _ => {
                println!("Server: Invalid command sent");
                utility::transmit_data(stream, Commands::Error.to_string().as_str());
            },
        }
    }
}

impl Runnables<ClientProfile> for ClientRemove {}

impl Runnables<ClientProfile> for Client {}

impl Runnables<ClientProfile> for Success {}

impl Runnables<ClientProfile> for Error {}




/* From this point onwards is all the implementations 
 * for PartialEq, ToString, and ParameterControl for 
 * each command struct.
 *
 *
 * PartialEq<T> implemented for Request
 */
impl PartialEq for Request {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl PartialEq<Info> for Request {
    fn eq(&self, other: &Info) -> bool {
        false
    }
}

impl PartialEq<HeartBeat> for Request {
    fn eq(&self, other: &HeartBeat) -> bool {
        false
    }
}

impl PartialEq<Connect> for Request {
    fn eq(&self, other: &Connect) -> bool {
        false
    }
}

impl PartialEq<Disconnect> for Request {
    fn eq(&self, other: &Disconnect) -> bool {
        false
    }
}

impl PartialEq<ClientUpdate> for Request {
    fn eq(&self, other: &ClientUpdate) -> bool {
        false
    }
}

impl PartialEq<ClientInfo> for Request {
    fn eq(&self, other: &ClientInfo) -> bool {
        false
    }
}

impl PartialEq<ClientRemove> for Request {
    fn eq(&self, other: &ClientRemove) -> bool {
        false
    }
}

impl PartialEq<Client> for Request {
    fn eq(&self, other: &Client) -> bool {
        false
    }
}

impl PartialEq<Success> for Request {
    fn eq(&self, other: &Success) -> bool {
        false
    }
}

impl PartialEq<Error> for Request {
    fn eq(&self, other: &Error) -> bool {
        false
    }
}

impl PartialEq<Commands> for Request {
    fn eq(&self, other: &Commands) -> bool {
        if let Commands::Request = other {
            return true;
        }

        false
    }
}


/*
 * PartialEq<T> implemented for Info
 */
impl PartialEq for Info {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl PartialEq<Request> for Info {
    fn eq(&self, other: &Request) -> bool {
        false
    }
}

impl PartialEq<HeartBeat> for Info {
    fn eq(&self, other: &HeartBeat) -> bool {
        false
    }
}

impl PartialEq<Connect> for Info {
    fn eq(&self, other: &Connect) -> bool {
        false
    }
}

impl PartialEq<Disconnect> for Info {
    fn eq(&self, other: &Disconnect) -> bool {
        false
    }
}

impl PartialEq<ClientUpdate> for Info {
    fn eq(&self, other: &ClientUpdate) -> bool {
        false
    }
}

impl PartialEq<ClientInfo> for Info {
    fn eq(&self, other: &ClientInfo) -> bool {
        false
    }
}

impl PartialEq<ClientRemove> for Info {
    fn eq(&self, other: &ClientRemove) -> bool {
        false
    }
}

impl PartialEq<Client> for Info {
    fn eq(&self, other: &Client) -> bool {
        false
    }
}

impl PartialEq<Success> for Info {
    fn eq(&self, other: &Success) -> bool {
        false
    }
}

impl PartialEq<Error> for Info {
    fn eq(&self, other: &Error) -> bool {
        false
    }
}

impl PartialEq<Commands> for Info {
    fn eq(&self, other: &Commands) -> bool {
        if let Commands::Info = other {
            return true;
        }

        false
    }
}

/*
 * PartialEq<T> implemented for HeartBeat
 */
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

impl PartialEq<Request> for HeartBeat {
    fn eq(&self, other: &Request) -> bool {
        false
    }
}

impl PartialEq<Info> for HeartBeat {
    fn eq(&self, other: &Info) -> bool {
        false
    }
}

impl PartialEq<Connect> for HeartBeat {
    fn eq(&self, other: &Connect) -> bool {
        false
    }
}

impl PartialEq<Disconnect> for HeartBeat {
    fn eq(&self, other: &Disconnect) -> bool {
        false
    }
}

impl PartialEq<ClientUpdate> for HeartBeat {
    fn eq(&self, other: &ClientUpdate) -> bool {
        false
    }
}

impl PartialEq<ClientInfo> for HeartBeat {
    fn eq(&self, other: &ClientInfo) -> bool {
        false
    }
}

impl PartialEq<ClientRemove> for HeartBeat {
    fn eq(&self, other: &ClientRemove) -> bool {
        false
    }
}

impl PartialEq<Client> for HeartBeat {
    fn eq(&self, other: &Client) -> bool {
        false
    }
}

impl PartialEq<Success> for HeartBeat {
    fn eq(&self, other: &Success) -> bool {
        false
    }
}

impl PartialEq<Error> for HeartBeat {
    fn eq(&self, other: &Error) -> bool {
        false
    }
}

impl PartialEq<Commands> for HeartBeat {
    fn eq(&self, other: &Commands) -> bool {
        if let Commands::HeartBeat(_) = other {
            return true;
        }

        false
    }
}

/*
 * PartialEq<T> implemented for Connect
 */
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

impl PartialEq<Request> for Connect {
    fn eq(&self, other: &Request) -> bool {
        false
    }
}

impl PartialEq<Info> for Connect {
    fn eq(&self, other: &Info) -> bool {
        false
    }
}

impl PartialEq<HeartBeat> for Connect {
    fn eq(&self, other: &HeartBeat) -> bool {
        false
    }
}

impl PartialEq<Disconnect> for Connect {
    fn eq(&self, other: &Disconnect) -> bool {
        false
    }
}

impl PartialEq<ClientUpdate> for Connect {
    fn eq(&self, other: &ClientUpdate) -> bool {
        false
    }
}

impl PartialEq<ClientInfo> for Connect {
    fn eq(&self, other: &ClientInfo) -> bool {
        false
    }
}

impl PartialEq<ClientRemove> for Connect {
    fn eq(&self, other: &ClientRemove) -> bool {
        false
    }
}

impl PartialEq<Client> for Connect {
    fn eq(&self, other: &Client) -> bool {
        false
    }
}

impl PartialEq<Success> for Connect {
    fn eq(&self, other: &Success) -> bool {
        false
    }
}

impl PartialEq<Error> for Connect {
    fn eq(&self, other: &Error) -> bool {
        false
    }
}

impl PartialEq<Commands> for Connect {
    fn eq(&self, other: &Commands) -> bool {
        if let Commands::Connect(_) = other {
            return true;
        }

        false
    }
}

/*
 * PartialEq<T> implemented for Disconnect
 */
impl PartialEq for Disconnect {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl PartialEq<Request> for Disconnect {
    fn eq(&self, other: &Request) -> bool {
        false
    }
}

impl PartialEq<Info> for Disconnect {
    fn eq(&self, other: &Info) -> bool {
        false
    }
}

impl PartialEq<HeartBeat> for Disconnect {
    fn eq(&self, other: &HeartBeat) -> bool {
        false
    }
}

impl PartialEq<Connect> for Disconnect {
    fn eq(&self, other: &Connect) -> bool {
        false
    }
}

impl PartialEq<ClientUpdate> for Disconnect {
    fn eq(&self, other: &ClientUpdate) -> bool {
        false
    }
}

impl PartialEq<ClientInfo> for Disconnect {
    fn eq(&self, other: &ClientInfo) -> bool {
        false
    }
}

impl PartialEq<ClientRemove> for Disconnect {
    fn eq(&self, other: &ClientRemove) -> bool {
        false
    }
}

impl PartialEq<Client> for Disconnect {
    fn eq(&self, other: &Client) -> bool {
        false
    }
}

impl PartialEq<Success> for Disconnect {
    fn eq(&self, other: &Success) -> bool {
        false
    }
}

impl PartialEq<Error> for Disconnect {
    fn eq(&self, other: &Error) -> bool {
        false
    }
}

impl PartialEq<Commands> for Disconnect {
    fn eq(&self, other: &Commands) -> bool {
        if let Commands::Disconnect = other {
            return true;
        }

        false
    }
}

/*
 * PartialEq<T> implemented for ClientUpdate
 */
impl PartialEq for ClientUpdate {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl PartialEq<Request> for ClientUpdate {
    fn eq(&self, other: &Request) -> bool {
        false
    }
}

impl PartialEq<Info> for ClientUpdate {
    fn eq(&self, other: &Info) -> bool {
        false
    }
}

impl PartialEq<HeartBeat> for ClientUpdate {
    fn eq(&self, other: &HeartBeat) -> bool {
        false
    }
}

impl PartialEq<Connect> for ClientUpdate {
    fn eq(&self, other: &Connect) -> bool {
        false
    }
}

impl PartialEq<Disconnect> for ClientUpdate {
    fn eq(&self, other: &Disconnect) -> bool {
        false
    }
}

impl PartialEq<ClientInfo> for ClientUpdate {
    fn eq(&self, other: &ClientInfo) -> bool {
        false
    }
}

impl PartialEq<ClientRemove> for ClientUpdate {
    fn eq(&self, other: &ClientRemove) -> bool {
        false
    }
}

impl PartialEq<Client> for ClientUpdate {
    fn eq(&self, other: &Client) -> bool {
        false
    }
}

impl PartialEq<Success> for ClientUpdate {
    fn eq(&self, other: &Success) -> bool {
        false
    }
}

impl PartialEq<Error> for ClientUpdate {
    fn eq(&self, other: &Error) -> bool {
        false
    }
}

impl PartialEq<Commands> for ClientUpdate {
    fn eq(&self, other: &Commands) -> bool {
        if let Commands::ClientUpdate = other {
            return true;
        }

        false
    }
}

/*
 * PartialEq<T> implemented for ClientInfo
 */
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

impl PartialEq<Request> for ClientInfo {
    fn eq(&self, other: &Request) -> bool {
        false
    }
}

impl PartialEq<Info> for ClientInfo {
    fn eq(&self, other: &Info) -> bool {
        false
    }
}

impl PartialEq<HeartBeat> for ClientInfo {
    fn eq(&self, other: &HeartBeat) -> bool {
        false
    }
}

impl PartialEq<Connect> for ClientInfo {
    fn eq(&self, other: &Connect) -> bool {
        false
    }
}

impl PartialEq<Disconnect> for ClientInfo {
    fn eq(&self, other: &Disconnect) -> bool {
        false
    }
}

impl PartialEq<ClientUpdate> for ClientInfo {
    fn eq(&self, other: &ClientUpdate) -> bool {
        false
    }
}

impl PartialEq<ClientRemove> for ClientInfo {
    fn eq(&self, other: &ClientRemove) -> bool {
        false
    }
}

impl PartialEq<Client> for ClientInfo {
    fn eq(&self, other: &Client) -> bool {
        false
    }
}

impl PartialEq<Success> for ClientInfo {
    fn eq(&self, other: &Success) -> bool {
        false
    }
}

impl PartialEq<Error> for ClientInfo {
    fn eq(&self, other: &Error) -> bool {
        false
    }
}

impl PartialEq<Commands> for ClientInfo {
    fn eq(&self, other: &Commands) -> bool {
        if let Commands::ClientInfo(_) = other {
            return true;
        }

        false
    }
}

/*
 * PartialEq<T> implemented for ClientRemove
 */
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

impl PartialEq<Request> for ClientRemove {
    fn eq(&self, other: &Request) -> bool {
        false
    }
}

impl PartialEq<Info> for ClientRemove {
    fn eq(&self, other: &Info) -> bool {
        false
    }
}

impl PartialEq<HeartBeat> for ClientRemove {
    fn eq(&self, other: &HeartBeat) -> bool {
        false
    }
}

impl PartialEq<Connect> for ClientRemove {
    fn eq(&self, other: &Connect) -> bool {
        false
    }
}

impl PartialEq<Disconnect> for ClientRemove {
    fn eq(&self, other: &Disconnect) -> bool {
        false
    }
}

impl PartialEq<ClientUpdate> for ClientRemove {
    fn eq(&self, other: &ClientUpdate) -> bool {
        false
    }
}

impl PartialEq<ClientInfo> for ClientRemove {
    fn eq(&self, other: &ClientInfo) -> bool {
        false
    }
}

impl PartialEq<Client> for ClientRemove {
    fn eq(&self, other: &Client) -> bool {
        false
    }
}

impl PartialEq<Success> for ClientRemove {
    fn eq(&self, other: &Success) -> bool {
        false
    }
}

impl PartialEq<Error> for ClientRemove {
    fn eq(&self, other: &Error) -> bool {
        false
    }
}

impl PartialEq<Commands> for ClientRemove {
    fn eq(&self, other: &Commands) -> bool {
        if let Commands::ClientRemove(_) = other {
            return true;
        }

        false
    }
}

/*
 * PartialEq<T> implemented for Client
 */
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

impl PartialEq<Request> for Client {
    fn eq(&self, other: &Request) -> bool {
        false
    }
}

impl PartialEq<Info> for Client {
    fn eq(&self, other: &Info) -> bool {
        false
    }
}

impl PartialEq<HeartBeat> for Client {
    fn eq(&self, other: &HeartBeat) -> bool {
        false
    }
}

impl PartialEq<Connect> for Client {
    fn eq(&self, other: &Connect) -> bool {
        false
    }
}

impl PartialEq<Disconnect> for Client {
    fn eq(&self, other: &Disconnect) -> bool {
        false
    }
}

impl PartialEq<ClientUpdate> for Client {
    fn eq(&self, other: &ClientUpdate) -> bool {
        false
    }
}

impl PartialEq<ClientInfo> for Client {
    fn eq(&self, other: &ClientInfo) -> bool {
        false
    }
}

impl PartialEq<ClientRemove> for Client {
    fn eq(&self, other: &ClientRemove) -> bool {
        false
    }
}

impl PartialEq<Success> for Client {
    fn eq(&self, other: &Success) -> bool {
        false
    }
}

impl PartialEq<Error> for Client {
    fn eq(&self, other: &Error) -> bool {
        false
    }
}

impl PartialEq<Commands> for Client {
    fn eq(&self, other: &Commands) -> bool {
        if let Commands::Client(_) = other {
            return true;
        }

        false
    }
}

/*
 * PartialEq<T> implemented for Success
 */
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

impl PartialEq<Request> for Success {
    fn eq(&self, other: &Request) -> bool {
        false
    }
}

impl PartialEq<Info> for Success {
    fn eq(&self, other: &Info) -> bool {
        false
    }
}

impl PartialEq<HeartBeat> for Success {
    fn eq(&self, other: &HeartBeat) -> bool {
        false
    }
}

impl PartialEq<Connect> for Success {
    fn eq(&self, other: &Connect) -> bool {
        false
    }
}

impl PartialEq<Disconnect> for Success {
    fn eq(&self, other: &Disconnect) -> bool {
        false
    }
}

impl PartialEq<ClientUpdate> for Success {
    fn eq(&self, other: &ClientUpdate) -> bool {
        false
    }
}

impl PartialEq<ClientInfo> for Success {
    fn eq(&self, other: &ClientInfo) -> bool {
        false
    }
}

impl PartialEq<ClientRemove> for Success {
    fn eq(&self, other: &ClientRemove) -> bool {
        false
    }
}

impl PartialEq<Client> for Success {
    fn eq(&self, other: &Client) -> bool {
        false
    }
}

impl PartialEq<Error> for Success {
    fn eq(&self, other: &Error) -> bool {
        false
    }
}

impl PartialEq<Commands> for Success {
    fn eq(&self, other: &Commands) -> bool {
        if let Commands::Success(_) = other {
            return true;
        }

        false
    }
}

/*
 * PartialEq<T> implemented for Error
 */
impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl PartialEq<Request> for Error {
    fn eq(&self, other: &Request) -> bool {
        false
    }
}

impl PartialEq<Info> for Error {
    fn eq(&self, other: &Info) -> bool {
        false
    }
}

impl PartialEq<HeartBeat> for Error {
    fn eq(&self, other: &HeartBeat) -> bool {
        false
    }
}

impl PartialEq<Connect> for Error {
    fn eq(&self, other: &Connect) -> bool {
        false
    }
}

impl PartialEq<Disconnect> for Error {
    fn eq(&self, other: &Disconnect) -> bool {
        false
    }
}

impl PartialEq<ClientUpdate> for Error {
    fn eq(&self, other: &ClientUpdate) -> bool {
        false
    }
}

impl PartialEq<ClientInfo> for Error {
    fn eq(&self, other: &ClientInfo) -> bool {
        false
    }
}

impl PartialEq<ClientRemove> for Error {
    fn eq(&self, other: &ClientRemove) -> bool {
        false
    }
}

impl PartialEq<Client> for Error {
    fn eq(&self, other: &Client) -> bool {
        false
    }
}

impl PartialEq<Success> for Error {
    fn eq(&self, other: &Success) -> bool {
        false
    }
}

impl PartialEq<Commands> for Error {
    fn eq(&self, other: &Commands) -> bool {
        if let Commands::Error = other {
            return true;
        }

        false
    }
}

/*
 * ToString implemented for all command types
 */
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

        if self.params.is_some() {
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

        if self.params.is_some() {
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

        if self.params.is_some() {
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

        if self.params.is_some() {
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

        if self.params.is_some() {
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

        if self.params.is_some() {
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
        String::from("!error:")
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
