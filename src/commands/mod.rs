pub mod behaviors;

use crate::{
    commands::behaviors::{
        Runnables,
        Request,
        Info,
        HeartBeat,
        Connect,
        Disconnect,
        ClientUpdate,
        ClientInfo,
        ClientRemove,
        Client,
        Success,
        Error,
    },
    server::{
        server_profile::Server,
        client::client_profile::Client as ClientProfile,
    },
};

use std::string::ToString;
use std::collections::HashMap;
use std::str::FromStr;

use std::borrow::Borrow;
use regex::Regex;
use std::ops::Index;
use log::info;
use zeroize::Zeroize;
use std::io::Error as IoError;
use std::net::TcpStream;
//use dashmap::DashMap;

trait Conversion<T> {
    fn from_str(data: &str) -> std::result::Result<Box<dyn Runnables<T>>, CommandParseError>;
}

pub trait GenerateFrom<T, U> {
    fn generate_from(data: T) -> Box<dyn Runnables<U>>;
}


pub struct CommandsAPI;


#[derive(Clone, Debug)]
pub enum Commands {
    Request,
    Info,

    HeartBeat(Option<HashMap<String, String>>),

    Connect(Option<HashMap<String, String>>),
    Disconnect,

    ClientUpdate,
    ClientInfo(Option<HashMap<String, String>>),
    ClientRemove(Option<HashMap<String, String>>),
    Client(Option<HashMap<String, String>>),

    Success(Option<HashMap<String, String>>),
    Error,
}


#[derive(Debug)]
pub enum CommandParseError {
    UnknownCommand,
    NoString,
}



impl Conversion<Server> for CommandsAPI {
    fn from_str(data: &str) -> std::result::Result<Box<dyn Runnables<Server>>, CommandParseError> {
        let regex = Regex::new(r###"(\?|!)([a-zA-z0-9]*):|([a-zA-z]*):([a-zA-Z0-9@\-\+\[\]{}_=/.]+|("(.*?)")+)"###).unwrap();
        let mut iter = regex.find_iter(data);
        let command_opt = iter.next();

        if command_opt.is_none() {
            return Err(CommandParseError::NoString);
        }
        let command = command_opt.unwrap().as_str();


        println!("command parsed to: {:?}", command);

        let mut map: HashMap<String, String> = HashMap::new();

        for i in iter {
            let parameter = i.as_str().to_string();
            let parts:Vec<&str> = parameter.split(":").collect();

            map.insert(parts.index(0).to_string(), parts.index(1).to_string());
        }

        let params = if map.capacity() > 0 {Some(map)} else { None };

        Ok(match command {
            "!info:" if params.is_none() => Box::new(Info),

            "!connect:" if params.is_some() => Box::new(Connect::new(&params)), //{params: params.clone()}),
            
            "!success:" => Box::new(Success::new(&params)), //{params: params.clone()}),

            "!error:" if params.is_none() => Box::new(Error),
 
            _ => Box::new(Error),
        })
    }
}

impl Conversion<ClientProfile> for CommandsAPI {
    fn from_str(data: &str) -> std::result::Result<Box<dyn Runnables<ClientProfile>>, CommandParseError> {
        let regex = Regex::new(r###"(\?|!)([a-zA-z0-9]*):|([a-zA-z]*):([a-zA-Z0-9@\-\+\[\]{}_=/.]+|("(.*?)")+)"###).unwrap();
        let mut iter = regex.find_iter(data);
        let command_opt = iter.next();

        if command_opt.is_none() {
            return Err(CommandParseError::NoString);
        }
        let command = command_opt.unwrap().as_str();


        println!("command parsed to: {:?}", command);

        let mut map: HashMap<String, String> = HashMap::new();

        for i in iter {
            let parameter = i.as_str().to_string();
            let parts:Vec<&str> = parameter.split(":").collect();

            map.insert(parts.index(0).to_string(), parts.index(1).to_string());
        }

        let params = if map.capacity() > 0 {Some(map)} else { None };

        Ok(match command {
            "!heartbeat:" => Box::new(HeartBeat::new(&params)), //{params: params.clone()}),

            "!disconnect:" if params.is_none() => Box::new(Disconnect),

            "!clientUpdate:" if params.is_none() => Box::new(ClientUpdate),

            "!clientInfo:" if params.is_some() => Box::new(ClientInfo::new(&params)), //{params: params.clone()}),

            "!success:" => Box::new(Success::new(&params)), //{params: params.clone()}),

            "!error:" if params.is_none() => Box::new(Error),
            
            _ => Box::new(Error),
        })
    }
}

impl GenerateFrom<String, Server> for CommandsAPI {
    fn generate_from(data: String) -> Box<dyn Runnables<Server>> {
        if let Ok(data) = <CommandsAPI as Conversion<Server>>::from_str(data.as_str()) {
            return data;
        }

        info!("Command: failed to parse with");
        Box::new(Error)
    }
}

impl GenerateFrom<&mut [u8; 1024], Server> for CommandsAPI {
    fn generate_from(data: &mut [u8; 1024]) -> Box<dyn Runnables<Server>> {
        let incoming_message = String::from(String::from_utf8_lossy(data));
        data.zeroize();
        <CommandsAPI as GenerateFrom<String, Server>>::generate_from(incoming_message)
    }
}



impl GenerateFrom<String, ClientProfile> for CommandsAPI {
    fn generate_from(data: String) -> Box<dyn Runnables<ClientProfile>> {
        if let Ok(data) = <CommandsAPI as Conversion<ClientProfile>>::from_str(data.as_str()) {
            return data;
        }
        
        info!("Command: failed to parse with");
        Box::new(Error)
    }
}

impl GenerateFrom<&mut [u8; 1024], ClientProfile> for CommandsAPI {
    fn generate_from(data: &mut [u8; 1024]) -> Box<dyn Runnables<ClientProfile>> {
        let incoming_message = String::from(String::from_utf8_lossy(data));
        data.zeroize();
        <CommandsAPI as GenerateFrom<String, ClientProfile>>::generate_from(incoming_message)
    }
}







impl Commands {
    fn compare_params(&self, params: &Option<HashMap<String, String>>, other_params: &Option<HashMap<String, String>>) -> bool {
        match (params, other_params) {
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

impl ToString for Commands {

    fn to_string(&self) -> std::string::String {
        let mut out_string = String::new();

        let (command, parameters): (&str, Option<HashMap<String, String>>) = match self {
            Commands::Request => ("!request:", None),
            Commands::Info => ("!info:", None),
            Commands::HeartBeat(arguments) => ("!heartbeat:", arguments.clone()),
            Commands::Connect(arguments) => ("!connect:", arguments.clone()),
            Commands::Disconnect => ("!disconnect:", None),
            Commands::ClientUpdate => ("!clientUpdate:", None),
            Commands::ClientInfo(arguments) => ("!clientInfo:", arguments.clone()),
            Commands::ClientRemove(arguments) => ("!clientRemove", arguments.clone()),
            Commands::Client(arguments) => ("!client:", arguments.clone()),
            Commands::Success(arguments) => ("!success:", arguments.clone()),
            Commands::Error => ("!error:", None),
        };

        out_string.push_str(command);

        if parameters.is_some() {
            let hash_map = parameters.borrow().as_ref().unwrap();
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

impl FromStr for Commands {
    type Err = CommandParseError;

    fn from_str(data: &str) -> std::result::Result<Self, Self::Err> {
        let regex = Regex::new(r###"(\?|!)([a-zA-z0-9]*):|([a-zA-z]*):([a-zA-Z0-9@\-\+\[\]{}_=/.]+|("(.*?)")+)"###).unwrap();
        let mut iter = regex.find_iter(data);
        let command_opt = iter.next();

        if command_opt.is_none() {
            return Err(CommandParseError::NoString);
        }
        let command = command_opt.unwrap().as_str();


        println!("command parsed to: {:?}", command);

        let mut map: HashMap<String, String> = HashMap::new();

        for i in iter {
            let parameter = i.as_str().to_string();
            let parts:Vec<&str> = parameter.split(":").collect();

            map.insert(parts.index(0).to_string(), parts.index(1).to_string());
        }

        let params = if map.capacity() > 0 {Some(map)} else { None };

        Ok(match command {
            "!request:" if params.is_none() => Commands::Request,
            "!info:" if params.is_none() => Commands::Info,

            "!heartbeat:" => Commands::HeartBeat(params),

            "!connect:" if params.is_some() => Commands::Connect(params),
            "!disconnect:" if params.is_none() => Commands::Disconnect,

            "!clientUpdate:" if params.is_none() => Commands::ClientUpdate,
            "!clientInfo:" if params.is_some() => Commands::ClientInfo(params),
            "!client:" if params.is_some() => Commands::Client(params),
            "!clientRemove:" if params.is_some() => Commands::ClientRemove(params),
            
            "!success:" => Commands::Success(params),
            "!error:" if params.is_none() => Commands::Error,
            
            _ => Commands::Error,
        })
    }
}

impl From<String> for Commands {
    fn from(data: String) -> Self {
        if let Ok(data) = data.as_str().parse() {
            return data;
        }

        info!("Command: failed to parse with");
        Commands::Error
    }
}

impl From<&mut [u8; 1024]> for Commands {
    fn from(data: &mut [u8; 1024]) -> Self {
        let incoming_message = String::from(String::from_utf8_lossy(data));
        data.zeroize();
        Commands::from(incoming_message)
    }
}

impl PartialEq for Commands {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Commands::Request, Commands::Request) => true,
            (Commands::Info, Commands::Info) => true,
            (Commands::HeartBeat(params), Commands::HeartBeat(other_params)) => self.compare_params(&params, &other_params),
            (Commands::Connect(params), Commands::Connect(other_params)) => self.compare_params(&params, &other_params),
            (Commands::Disconnect, Commands::Disconnect) => true,
            (Commands::ClientUpdate, Commands::ClientUpdate) => true,
            (Commands::ClientInfo(params), Commands::ClientInfo(other_params)) => self.compare_params(&params, &other_params),
            (Commands::ClientRemove(params), Commands::ClientRemove(other_params)) => self.compare_params(&params, &other_params),
            (Commands::Client(params), Commands::Client(other_params)) => self.compare_params(&params, &other_params),
            (Commands::Success(params), Commands::Success(other_params)) => self.compare_params(&params, &other_params),
            (Commands::Error, Commands::Error) => true,
            _ => false,
        }
    }
}

impl PartialEq<Request> for Commands {
    fn eq(&self, _other: &Request) -> bool {
        if let Commands::Request = self {
            return true;
        }

        false
    }
}

impl PartialEq<Info> for Commands {
    fn eq(&self, _other: &Info) -> bool {
        if let Commands::Info = self {
            return true;
        }

        false
    }
}

impl PartialEq<HeartBeat> for Commands {
    fn eq(&self, _other: &HeartBeat) -> bool {
        if let Commands::HeartBeat(_) = self {
            return true;
        }

        false
    }
}

impl PartialEq<Connect> for Commands {
    fn eq(&self, _other: &Connect) -> bool {
        if let Commands::Connect(_) = self {
            return true;
        }

        false
    }
}

impl PartialEq<Disconnect> for Commands {
    fn eq(&self, _other: &Disconnect) -> bool {
        if let Commands::Disconnect = self {
            return true;
        }

        false
    }
}

impl PartialEq<ClientUpdate> for Commands {
    fn eq(&self, _other: &ClientUpdate) -> bool {
        if let Commands::ClientUpdate = self {
            return true;
        }

        false
    }
}

impl PartialEq<ClientInfo> for Commands {
    fn eq(&self, _other: &ClientInfo) -> bool {
        if let Commands::ClientInfo(_) = self {
            return true;
        }

        false
    }
}

impl PartialEq<ClientRemove> for Commands {
    fn eq(&self, _other: &ClientRemove) -> bool {
        if let Commands::ClientRemove(_) = self {
            return true;
        }

        false
    }
}

impl PartialEq<Client> for Commands {
    fn eq(&self, _other: &Client) -> bool {
        if let Commands::Client(_) = self {
            return true;
        }

        false
    }
}

impl PartialEq<Success> for Commands {
    fn eq(&self, _other: &Success) -> bool {
        if let Commands::Success(_) = self {
            return true;
        }

        false
    }
}

impl PartialEq<Error> for Commands {
    fn eq(&self, _other: &Error) -> bool {
        if let Commands::Error = self {
            return true;
        }

        false
    }
}


// TODO: check if unit tests still work
/*#[cfg(test)]
mod test_commands_v2 {
    #![feature(test)]
    use super::Commands;
    use std::collections::HashMap;
    use std::str::FromStr;
    use super::CommandParseError;

    #[test]
    fn test_creation_from_string() {
        let command_result = Commands::from_str("!connect: name:bop host:127.0.0.1 uuid:123456-1234-1234-123456");
    }

    #[test]
    fn test_to_string() {

        let mut a: HashMap<String, String> = HashMap::new();
        a.insert("name".to_string(), "michael".to_string());
        a.insert("host".to_string(), "127.0.0.1".to_string());
        a.insert("uuid".to_string(), "123456-1234-1234-123456".to_string());

        let command = Commands::Connect(Some(a));

        println!("{:?}", command.to_string())
    }
}*/
