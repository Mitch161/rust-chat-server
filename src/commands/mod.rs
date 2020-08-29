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
pub enum Commands {
    Request(Request),
    Info(Info),

    HeartBeat(HeartBeat),

    Connect(Connect),
    Disconnect(Disconnect),

    ClientUpdate(ClientUpdate),
    ClientInfo(ClientInfo),
    ClientRemove(ClientRemove),
    Client(Client),

    Success(Success),
    Error(Error),
}

#[derive(Debug)]
pub enum CommandParseError {
    UnknownCommand,
    NoString,
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

impl PartialEq for Commands {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Commands::Request(arguments), Commands::Request(other_arguments)) => self.compare_params(&arguments.get_params(), &other_arguments.get_params()),
            (Commands::Info(arguments), Commands::Info(other_arguments)) => self.compare_params(&arguments.get_params(), &other_arguments.get_params()),
            (Commands::Connect(arguments), Commands::Connect(other_arguments)) => self.compare_params(&arguments.get_params(), &other_arguments.get_params()),
            (Commands::Disconnect(arguments), Commands::Disconnect(other_arguments)) => self.compare_params(&arguments.get_params(), &other_arguments.get_params()),
            (Commands::ClientUpdate(arguments), Commands::ClientUpdate(other_arguments)) => self.compare_params(&arguments.get_params(), &other_arguments.get_params()),
            (Commands::ClientInfo(arguments), Commands::ClientInfo(other_arguments)) => self.compare_params(&arguments.get_params(), &other_arguments.get_params()),
            (Commands::ClientRemove(arguments), Commands::ClientRemove(other_arguments)) => self.compare_params(&arguments.get_params(), &other_arguments.get_params()),
            (Commands::Client(arguments), Commands::Client(other_arguments)) => self.compare_params(&arguments.get_params(), &other_arguments.get_params()),
            (Commands::Success(arguments), Commands::Success(other_arguments)) => self.compare_params(&arguments.get_params(), &other_arguments.get_params()),
            (Commands::Error(arguments), Commands::Error(other_arguments)) => self.compare_params(&arguments.get_params(), &other_arguments.get_params()),
            _ => false,
        }
    }
}


impl ToString for Commands {

    fn to_string(&self) -> std::string::String {
        let mut out_string = String::new();

        let (command, parameters) = match self {
            Commands::Request(arguments) => { ("!request:", arguments.get_params()) },
            Commands::Info(arguments) => { ("!info:", arguments.get_params()) },
            Commands::HeartBeat(arguments) => {("!heartbeat:", arguments.get_params())},
            Commands::Connect(arguments) => { ("!connect:", arguments.get_params()) },
            Commands::Disconnect(arguments) => { ("!disconnect:", arguments.get_params()) },
            Commands::ClientUpdate(arguments) => { ("!clientUpdate:", arguments.get_params()) },
            Commands::ClientInfo(arguments) => { ("!clientInfo:", arguments.get_params()) },
            Commands::ClientRemove(arguments) => { ("!clientRemove", arguments.get_params()) },
            Commands::Client(arguments) => { ("!client:", arguments.get_params()) },
            Commands::Success(arguments) => { ("!success:", arguments.get_params()) },
            Commands::Error(arguments) => { ("!error:", arguments.get_params()) },
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
            "!request:" => {
                Commands::Request(Request {
                    params: params,
                    //command: Commands::Request(params),
                })
            },
            "!info:" => {
                Commands::Info(Info {
                    params: params,
                    //command: Commands::Info(params),
                })
            },
            "!heartbeat:" => {
                Commands::Heartbeat(Heartbeat {
                    params: params,
                    //command: Commands::HeartBeat(params),
                })
            },

            "!connect:" => {
                Commands::Connect(Connect {
                    params: params,
                    //command: Commands::Connect(params),
                    //uuid: map.get("uuid").unwrap(),
                    //username: map.get("name").unwrap(),
                    //address: map.get("host").unwrap(),
                })
            },
            "!disconnect:" => {
                Commands::Disconnect(Disconnect {
                    params: params,
                })
            },

            "!clientUpdate:" => {
                Commands::ClientUpdate(ClientUpdate {
                    params: params,
                    //command: Commands::ClientUpdate(params),
                })
            },
            "!clientInfo:" => {
                Commands::ClientInfo(ClientInfo {
                    params: params,
                    //uuid: map.get("uuid").unwrap(),
                })
            },
            "!client:" => {
                Commands::Client(Client {
                    params: params,
                    //command: Commands::Client(params),
                    //error: Commands::Error(None),
                })
            },
            "!clientRemove:" => {
                Commands::ClientRemove(ClientRemove {
                    params: params,
                    //command: Commands::ClientRemove(params),
                    //error: Commands::Error(None),
                })
            },
            
            "!success:" => {
                Commands::Success(Success {
                    params: params,
                    //command: Commands::Success(params),
                })
            },
            "!error:" => {
                Commands::Error(Error {
                    params: params,
                    //command: Commands::Error(params),
                })
            },
            
            _ => {
                Commands::Error(Error {
                    params: params,
                    //command: Commands::Error(None),
                })
            },
        })
    }
}

impl From<String> for Commands {
    fn from(data: String) -> Self {
        if let Ok(data) = data.as_str().parse() {
            data
        } else {
            info!("Command: failed to parse with");
            Commands::Error(Error {})
        }
    }
}

impl From<&mut [u8; 1024]> for Commands {
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
