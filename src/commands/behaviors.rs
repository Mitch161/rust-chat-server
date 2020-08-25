use crate::server::utility;

struct Request {
    command: Commands,
}

struct Info {
    command: Commands,
}

struct Connect {
    command: Commands,
    params: Option<HashMap<String, String>>,
}

struct Disconnect {
    command: Commands,
}

struct ClientUpdate {
    command: Commands,
}

struct ClientInfo {
    command: Commands,
}

struct ClientRemove {
    command: Commands,
}

struct Client {
    command: Commands,
}

struct Success {
    command: Commands,
}

struct Error {
    command: Commands,
}

trait ServerRunnables {
    fn execute(&self, stream: &TcpStream server: &Server);
}

trait ClientRunnables {
    fn execute(&self, stream: &TcpStream client: &Client);
    
    fn execute(&self, stream: &TcpStream _: &Server) {
        println!("Server: Invalid command sent");
        utility::transmit_data(stream, Commands::Error(None).to_string().as_str());
    }
}

impl ServerRunnables for Info {
    fn run(&self, stream: &TcpStream _: &Server) {
        println!("Server: info requested");

        utility::transmit_data(stream, self.command.to_string().as_str());
    }
}

impl ServerRunnables for Connect {
    fn run(&self, stream: &TcpStream, server: &Server) {
        let uuid = self.params.get("uuid").unwrap();
        let username = self.params.get("name").unwrap();
        let address = self.params.get("host").unwrap();
        
        println!("{}", format!("Server: new Client connection: _addr = {}", address ));
        
        let client = Client::new(stream, server.get_sender().clone(), &uuid, &username, &address);

        let connected_clients = server.get_connected_clients();
        connected_clients.lock().unwrap().insert(uuid.to_string(), client);
        
        let params: HashMap<String, String> = [(String::from("name"), username.clone()), (String::from("host"), address.clone()), (String::from("uuid"), uuid.clone())].iter().cloned().collect();
        let new_client = Commands::Client(Some(params));
        
        let _ = connected_clients.lock().unwrap().iter().map(|(_k, v)| v.sender.send(new_client.clone()));
    }
}






impl ClientRunnables for Disconnect {
    fn run() {
    }
}

impl ClientRunnables for ClientUpdate {
}

impl ClientRunnables for ClientInfo {
    fn run() {
    }
}

impl ClientRunnables for ClientRemove {
    fn run() {
    }
}

impl ClientRunnables for Client {
    fn run() {
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
