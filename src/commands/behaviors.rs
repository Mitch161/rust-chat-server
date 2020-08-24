struct Request {
    executable: Commands,
}

struct Info {
    executable: Commands,
}

struct Connect {
    executable: Commands,
}

struct Disconnect {
    executable: Commands,
}

struct ClientUpdate {
    executable: Commands,
}

struct ClientInfo {
    executable: Commands,
}

struct ClientRemove {
    executable: Commands,
}

struct Client {
    executable: Commands,
}

struct Success {
    executable: Commands,
}

struct Error {
    executable: Commands,
}

trait Runnables {
    fn execute(&self);
}






impl ServerRunnables for Request {
    fn execute(server: &Server) {
    }
}

impl Runnables for Info {
    fn run(server: &Server, stream, &TcpStream) {
        println!("Server: info requested");
        let params: HashMap<String, String> = [(String::from("name"), self.name.to_string().clone()), (String::from("owner"), self.author.to_string().clone())].iter().cloned().collect();
        let command = Commands::Info(Some(params));

        server.transmit_data(stream, command.to_string().as_str());
    }
}

impl ServerRunnables for Connect {
    fn run(server: &Server, stream: &TcpStream) {

    }
}

impl Runnables for Disconnect {
    fn run() {
    }
}

impl ClientRunnables for ClientUpdate {
    fn client_execution(client: &Client) {
        let mut command = Commands::Success(None);
        client.transmit_data(command.to_string().as_str());

        let data: HashMap<String, String> = [(String::from("uuid"), client.get_uuid())].iter().cloned().collect();
        let command = Commands::ClientUpdate(Some(data));

        self.server.update_all_clients(self.uuid.as_str(), command);

    }
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
