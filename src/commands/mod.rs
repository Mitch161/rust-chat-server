use std::string::ToString;
use std::collections::HashMap;
use std::str::FromStr;

use std::borrow::Borrow;
use regex::Regex;
use std::ops::Index;
use log::info;
use zeroize::Zeroize;
//use dashmap::DashMap;

#[derive(Clone, Debug)]
pub enum Commands<T: Runnables<&Client> + Runnables<&Server> + Runnables<&mut [u8; 1024]>> {
    Type(T),
    /*Request(Request),
    Info(Info),

    HeartBeat(HeartBeat),

    Connect(Connect),
    Disconnect(Disconnect),

    ClientUpdate(ClientUpdate),
    ClientInfo(ClientInfo),
    ClientRemove(ClientRemove),
    Client(Client),

    Success(Success),
    Error(Error),*/
}

#[derive(Debug)]
pub enum CommandParseError {
    UnknownCommand,
    NoString,
}


impl<T: Runnables<&Client> + Runnables<&Server> + Runnables<&mut [u8; 1024]>> PartialEq for Commands<T> {
    fn eq(&self, other: &Self) -> bool {
        let Commands::Type(ref arguments_a) = self;
        let Commands::Type(ref arguments_b) = other;

        arguments_a == arguments_b
    }
}


impl<T: Runnables<&Client> + Runnables<&Server> + Runnables<&mut [u8; 1024]>> ToString for Commands<T> {

    fn to_string(&self) -> std::string::String {
        let mut out_string = String::new();

        let Commands::Type(ref arguments) = self;
        arguments.to_string()
    }
}

impl<T: Runnables<&Client> + Runnables<&Server> + Runnables<&mut [u8; 1024]>> FromStr for Commands<T> {
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

        Ok(Commands::Type(match command {
            "!request:" if params.is_none() => Request {},
            "!info:" if params.is_none() => Info {},

            "!heartbeat:" => {
                Heartbeat {
                    params: params,
                }
            },

            "!connect:" if params.is_some() => {
                Connect {
                    params: params,
                }
            },
            "!disconnect:" if params.is_none() => Disconnect {},

            "!clientUpdate:" if params.is_none() => ClientUpdate {},
            "!clientInfo:" if params.is_some() => {
                ClientInfo {
                    params: params,
                }
            },
            "!client:" if params.is_some() => {
                Client {
                    params: params,
                }
            },
            "!clientRemove:" if params.is_some() => {
                ClientRemove {
                    params: params,
                }
            },
            
            "!success:" => {
                Success {
                    params: params,
                    //command: Commands::Success(params),
                }
            },
            "!error:" if params.is_none() => Error {},
            
            _ => Error {},
        }))
    }
}

impl<T: Runnables<&Client> + Runnables<&Server> + Runnables<&mut [u8; 1024]>> From<String> for Commands<T> {
    fn from(data: String) -> Self {
        if let Ok(data) = data.as_str().parse() {
            data
        } else {
            info!("Command: failed to parse with");
            Commands::Type(Error {})
        }
    }
}

impl<T: Runnables<&Client> + Runnables<&Server> + Runnables<&mut [u8; 1024]>> From<&mut [u8; 1024]> for Commands<T> {
    fn from(data: &mut [u8; 1024]) -> Self {
        let incoming_message = String::from(String::from_utf8_lossy(data));
        data.zeroize();
        Commands::from(incoming_message)
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
