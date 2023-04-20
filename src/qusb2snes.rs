#[allow(dead_code)]
pub mod usb2snes {
    use std::net::TcpStream;

    use serde::{Deserialize, Serialize};
    use serde_json;
    use strum_macros::Display;
    use tungstenite::{connect, stream::MaybeTlsStream, Message, WebSocket};

    #[derive(Display, Debug)]
    pub enum Command {
        AppVersion,
        Name,
        DeviceList,
        Attach,
        Info,
        Boot,
        Reset,
        Menu,

        List,
        PutFile,
        GetFile,
        Rename,
        Remove,

        GetAddress,
    }
    #[derive(Display, Debug)]
    #[allow(dead_code, clippy::upper_case_acronyms)]
    pub enum Space {
        None,
        SNES,
        CMD,
    }

    #[derive(Debug)]
    pub struct Infos {
        pub version: String,
        pub dev_type: String,
        pub game: String,
        pub flags: Vec<String>,
    }

    #[derive(Serialize)]
    #[allow(non_snake_case)]
    struct USB2SnesQuery {
        Opcode: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        Space: Option<String>,
        Flags: Vec<String>,
        Operands: Vec<String>,
    }
    #[derive(Deserialize)]
    #[allow(non_snake_case)]
    struct USB2SnesResult {
        Results: Vec<String>,
    }

    pub struct SyncClient {
        client: WebSocket<MaybeTlsStream<TcpStream>>,
    }

    impl SyncClient {
        pub fn connect() -> Self {
            let (client, response) = connect("ws://localhost:8080").expect("Failed to connect");
            println!("Connected to the server");
            println!("Response HTTP code: {}", response.status());
            println!("Response contains the following headers:");
            for (ref header, _value) in response.headers() {
                println!("* {}", header);
            }
            Self { client }
        }

        fn get_reply(&mut self) -> USB2SnesResult {
            let reply = self.client.read_message().expect("Error reading message");
            dbg!(&reply);
            let mut textreply: String = String::from("");
            match reply {
                Message::Text(value) => {
                    textreply = value;
                }
                _ => {
                    println!("Error getting a reply");
                }
            }
            println!("Received: {}", textreply);
            serde_json::from_str(&textreply).unwrap()
        }

        pub fn send_message(
            &mut self,
            opcode: Command,
            space: Option<Space>,
            operands: Vec<String>,
        ) {
            let nspace = space.map(|sp| sp.to_string());

            let query = USB2SnesQuery {
                Opcode: opcode.to_string(),
                Space: nspace,
                Flags: vec![],
                Operands: operands,
            };

            let json = serde_json::to_string(&query).unwrap();
            let message = Message::text(json);
            self.client.write_message(message).unwrap();
        }

        pub fn list_device(&mut self) -> Vec<String> {
            self.send_message(Command::DeviceList, Some(Space::SNES), vec![]);
            let reply = self.get_reply();
            reply.Results
        }

        pub fn attach(&mut self, device: &String) {
            self.send_message(Command::Attach, Some(Space::SNES), vec![device.to_string()]);
        }

        pub fn set_name(&mut self, name: String) {
            self.send_message(Command::Name, Some(Space::SNES), vec![name]);
        }

        pub fn app_version(&mut self) -> String {
            self.send_message(Command::AppVersion, Some(Space::SNES), vec![]);
            let usbreply = self.get_reply();
            usbreply.Results[0].to_string()
        }

        pub fn info(&mut self) -> Infos {
            self.send_message(Command::Info, Some(Space::SNES), vec![]);
            let usbreply = self.get_reply();
            let info: Vec<String> = usbreply.Results;
            Infos {
                version: info[0].clone(),
                dev_type: info[1].clone(),
                game: info[2].clone(),
                flags: (info[3..].to_vec()),
            }
        }

        pub fn get_address(&mut self, address: u32, size: usize) -> Vec<u8> {
            self.send_message(
                Command::GetAddress,
                Some(Space::SNES),
                vec![format!("{:x}", address), format!("{:x}", size)],
            );
            let mut data: Vec<u8> = vec![];
            data.reserve(size);
            loop {
                let reply = self.client.read_message().expect("Error reading message");
                match reply {
                    Message::Binary(msgdata) => {
                        data.extend(&msgdata);
                    }
                    _ => {
                        println!("Error getting a reply");
                    }
                }
                if data.len() == size {
                    break;
                }
            }
            data
        }
    }
}
