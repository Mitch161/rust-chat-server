use crate::server::utility;

struct Request {}

struct Info {}

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

trait Runnables<T> {
    fn execute(&self, stream: &TcpStream, input: T);
}

trait ParameterControl {
    fn get_params(&self) -> Option<HashMap<String, String>> {
        self.params
    }
}


impl Runnables<&Client> for Info {
    fn execute(&self, stream: &TcpStream, _input: &Client) {
        println!("Server: Invalid command sent");
        utility::transmit_data(stream, Commands::Error(None).to_string().as_str());
    }
}

impl Runnables<&Server> for Info {
    fn execute(&self, stream: &TcpStream, input: &Server) {
        println!("Server: info requested");

        let params: HashMap<String, String> = [(String::from("name"), input.get_name()), (String::from("owner"), input.get_author())].iter().cloned().collect();
        let command = Commands::Info(Some(params));

        utility::transmit_data(stream, self.command.to_string().as_str());
    }
}

impl Runnables<&mut [u8; 1024]> for Info {
    fn execute(&self, stream: &TcpStream, _input: &mut [u8; 1024]) {
        println!("Server: Invalid command sent");
        utility::transmit_data(stream, Commands::Error(None).to_string().as_str());
    }
}


impl Runnables<&Client> for Connect {
    fn execute(&self, stream: &TcpStream, _input: &Client) {
        println!("Server: Invalid command sent");
        utility::transmit_data(stream, Commands::Error(None).to_string().as_str());
    }
}

impl Runnables<&Server> for Connect {
    fn execute(&self, stream: &TcpStream, input: &Server) {
        match self.params {
            Some(map) => {
                let uuid = map.get("uuid").unwrap();
                let username = map.get("name").unwrap();
                let address = map.get("host").unwrap();

                println!("{}", format!("Server: new Client connection: _addr = {}", address ));
                
                let client = Client::new(stream, input.get_sender().clone(), &uuid, &username, &address);

                let connected_clients = input.get_connected_clients();
                connected_clients.lock().unwrap().insert(uuid.to_string(), client);

                let params: HashMap<String, String> = [(String::from("name"), username.clone()), (String::from("host"), address.clone()), (String::from("uuid"), uuid.clone())].iter().cloned().collect();
                let new_client = Commands::Client(Some(params));

                let _ = connected_clients.lock().unwrap().iter().map(|(_k, v)| v.sender.send(new_client.clone()));
            },
            None => {
                println!("Server: Invalid command sent");
                utility::transmit_data(stream, Commands::Error(None).to_string().as_str());
            },
        }
    }
}

impl Runnables<&mut [u8; 1024]> for Connect {
    fn execute(&self, stream: &TcpStream, _input: &mut [u8; 1024]) {
        println!("Server: Invalid command sent");
        utility::transmit_data(stream, Commands::Error(None).to_string().as_str());
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
        utility::transmit_data(stream, Commands::Error(None).to_string().as_str());
    }
}

impl Runnables<&mut [u8; 1024]> for Disconnect {
    fn execute(&self, stream: &TcpStream, _input: &mut [u8; 1024]) {
        println!("Server: Invalid command sent");
        utility::transmit_data(stream, Commands::Error(None).to_string().as_str());
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
        utility::transmit_data(stream, Commands::Error(None).to_string().as_str());
    }
}

impl Runnables<&mut [u8; 1024]> for ClientUpdate {
    fn run(&self, stream: &TcpStream, _input: &mut [u8; 1024]) {
        println!("Server: Invalid command sent");
        utility::transmit_data(stream, Commands::Error(None).to_string().as_str());
    }
}


impl Runnables<&Client> for ClientInfo {
    fn execute(&self, stream: &TcpStream, input: &Client) {
        let uuid =
        let _ = input.get_server_sender().send(ServerMessages::RequestInfo(uuid.clone(), input.get_stream_arc().clone()));
    }
}

impl Runnables<&Server> for ClientInfo {
    fn execute(&self, stream: &TcpStream, _input: &Server) {
        println!("Server: Invalid command sent");
        utility::transmit_data(stream, Commands::Error(None).to_string().as_str());
    }
}

impl Runnables<&mut [u8; 1024]> for ClientInfo {
    fn run(&self, stream: &TcpStream, _input: &mut [u8; 1024]) {
        println!("Server: Invalid command sent");
        utility::transmit_data(stream, Commands::Error(None).to_string().as_str());
    }
}






impl Runnables<&Client> for ClientRemove {
    fn execute(&self, stream: &TcpStream, _input: &Client) {
        println!("Server: Invalid command sent");
        utility::transmit_data(stream, Commands::Error(None).to_string().as_str());
    }
}

impl Runnables<&Server> for ClientRemove {
    fn execute(&self, stream: &TcpStream, _input: &Server) {
        println!("Server: Invalid command sent");
        utility::transmit_data(stream, Commands::Error(None).to_string().as_str());
    }
}

impl Runnables<&mut [u8; 1024]> for ClientRemove {
    fn execute(&self, stream: &TcpStream, input: &mut [u8; 1024]) {
        let mut retry: u8 = 3;
        'retry_loop: loop {
            if retry < 1 {
                utility::transmit_data(stream, self.error.to_string().as_str());
                break 'retry_loop;
            } else {                    
                utility::transmit_data(stream, self.command.to_string().as_str());

                if utility::read_data(input).unwrap_or(Commands::Error(None)) == Commands::Success(None) {
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
        utility::transmit_data(stream, Commands::Error(None).to_string().as_str());
    }
}

impl Runnables<&Server> for Client {
    fn execute(&self, stream: &TcpStream, _input: &Server) {
        println!("Server: Invalid command sent");
        utility::transmit_data(stream, Commands::Error(None).to_string().as_str());
    }
}

impl Runnables<&mut [u8; 1024]> for Client {
    fn execute(&self, stream: &TcpStream, input: &mut [u8; 1024]) {
        let mut retry: u8 = 3;
        'retry_loop: loop {
            if retry < 1 {
                utility::transmit_data(stream, self.error.to_string().as_str());
                break 'retry_loop;
            } else {
                utility::transmit_data(stream, self.command.to_string().as_str());
                
                if utility::read_data(input).unwrap_or(Commands::Error(None)) == Commands::Success(None) {
                    break 'retry_loop;
                } else {
                    retry -= 1;
                }
            }
        }
    }
}

/*impl Runnables for Success {
    fn execute() {
    }
}

impl Runnables for Error {
    fn execute() {
    }
}*/


impl ParameterControl for HeartBeat {}
impl ParameterControl for Connect {}
impl ParameterControl for ClientInfo {}
impl ParameterControl for ClientRemove {}
impl ParameterControl for Client {}
impl ParameterControl for Success {}
