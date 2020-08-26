use crate::server::utility;

struct Request {
    command: Commands,
}

struct Info {
    command: Commands,
}

struct Connect {
    command: Commands,
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

trait ServerRunnables {
    fn execute(&self, stream: &TcpStream, server: &Server);
}

trait ClientRunnables {
    fn execute(&self, stream: &TcpStream, client: &Client);
    
    fn execute(&self, stream: &TcpStream, _server: &Server) {
        println!("Server: Invalid command sent");
        utility::transmit_data(stream, Commands::Error(None).to_string().as_str());
    }
}

impl ServerRunnables for Info {
    fn run(&self, stream: &TcpStream, _server: &Server) {
        println!("Server: info requested");

        utility::transmit_data(stream, self.command.to_string().as_str());
    }
}

impl ServerRunnables for Connect {
    fn run(&self, stream: &TcpStream, server: &Server) {
        println!("{}", format!("Server: new Client connection: _addr = {}", address ));
        
        let client = Client::new(stream, server.get_sender().clone(), &uuid, &username, &address);

        let connected_clients = server.get_connected_clients();
        connected_clients.lock().unwrap().insert(uuid.to_string(), client);
        
        let _ = connected_clients.lock().unwrap().iter().map(|(_k, v)| v.sender.send(self.command.clone()));
    }
}






impl ClientRunnables for Disconnect {
    fn execute(&self, _stream: &TcpStream, client: &Client) {
        client.get_server_sender().send(ServerMessages::Disconnect(client.get_uuid())).expect("sending message to server failed");
        client.get_stream_arc().lock().unwrap().shutdown(Shutdown::Both).expect("shutdown call failed");
    }
}

impl ClientRunnables for ClientUpdate {
    fn execute(&self, stream: &TcpStream, client: &Client) {
        utility::transmit_data(stream, self.command.to_string().as_str());
        let _ = client.get_server_sender().send(ServerMessages::RequestUpdate(client.get_stream_arc().clone()));
    }
}

impl ClientRunnables for ClientInfo {
    fn execute(&self, stream: &TcpStream, client: &Client) {
        let _ = client.get_server_sender().send(ServerMessages::RequestInfo(self.uuid.clone(), client.get_stream_arc().clone()));
    }
}

impl ClientRunnables for ClientRemove {
    fn execute(&self, stream: &TcpStream, buffer: &mut [u8; 1024]) {
        let mut retry: u8 = 3;
        'retry_loop: loop {
            if retry < 1 {
                utility::transmit_data(self.error.to_string().as_str());
                break 'retry_loop;
            } else {                    
                utility::transmit_data(self.command.to_string().as_str());

                if utility::read_data(buffer).unwrap_or(Commands::Error(None)) == Commands::Success(None) {
                    break 'retry_loop;
                } else {
                    retry -= 1;
                }
            }
        }
    }
}

impl ClientRunnables for Client {
    fn execute(&self, stream: &TcpStream, buffer: &mut [u8; 1024]) {
        let mut retry: u8 = 3;
        'retry_loop: loop {
            if retry < 1 {
                utility::transmit_data(self.error.to_string().as_str());
                break 'retry_loop;
            } else {
                utility::transmit_data(self.command.to_string().as_str());
                
                if utility::read_data(buffer).unwrap_or(Commands::Error(None)) == Commands::Success(None) {
                    break 'retry_loop;
                } else {
                    retry -= 1;
                }
            }
        }
    }
}

impl ClientRunnables for Success {
    fn run() {
    }
}

impl ClientRunnables for Error {
    fn run() {
    }
}
