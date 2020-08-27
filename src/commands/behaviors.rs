use crate::server::utility;

struct Request {
    command: Commands,
}

struct Info {
    command: Commands,
}

struct Connect {
    command: Commands,
    uuid: String,
    username: String,
    address: String,

}

struct Disconnect {
}

struct ClientUpdate {
    command: Commands,
}

struct ClientInfo {
    uuid: String,
}

struct ClientRemove {
    command: Commands,
    error: Commands,
}

struct Client {
    command: Commands,
    error: Commands,
}

struct Success {
    command: Commands,
}

struct Error {
    command: Commands,
}

trait Runnables<T> {
    fn execute(&self, stream: &TcpStream, input: T);
}




impl Runnables<&Client> for Info {
    fn execute(&self, stream: &TcpStream, _input: &Client) {
        println!("Server: Invalid command sent");
        utility::transmit_data(stream, Commands::Error(None).to_string().as_str());
    }
}

impl Runnables<&Server> for Info {
    fn execute(&self, stream: &TcpStream, _input: &Server) {
        println!("Server: info requested");

        utility::transmit_data(stream, self.command.to_string().as_str());
    }
}

impl Runnables<&mut [u8; 1024]> for Info {
    fn run(&self, stream: &TcpStream, _input: &mut [u8; 1024]) {
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
    fn run(&self, stream: &TcpStream, input: &Server) {
        println!("{}", format!("Server: new Client connection: _addr = {}", self.address ));
        
        let client = Client::new(stream, input.get_sender().clone(), &self.uuid, &self.username, &self.address);

        let connected_clients = input.get_connected_clients();
        connected_clients.lock().unwrap().insert(uuid.to_string(), client);
        
        let _ = connected_clients.lock().unwrap().iter().map(|(_k, v)| v.sender.send(self.command.clone()));
    }
}

impl Runnables<&mut [u8; 1024]> for Connect {
    fn run(&self, stream: &TcpStream, _input: &mut [u8; 1024]) {
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
    fn run(&self, stream: &TcpStream, _input: &mut [u8; 1024]) {
        println!("Server: Invalid command sent");
        utility::transmit_data(stream, Commands::Error(None).to_string().as_str());
    }
}


impl Runnables<&Client> for ClientUpdate {
    fn execute(&self, stream: &TcpStream, input: &Client) {
        utility::transmit_data(stream, self.command.to_string().as_str());
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
        let _ = input.get_server_sender().send(ServerMessages::RequestInfo(self.uuid.clone(), input.get_stream_arc().clone()));
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
