use crate::integration::client;
use crate::server::Server;
use std::process::{Child, Command, Stdio};

pub const SENDER_ADDRESS: &'static str = "localhost:9000";
const SENDER_PATH: &str = "../../../target/release/import-http-sender";
const SENDER_CONFIG_PATH: &str = "../../../import/http/tests/config/sender.toml";

const RECEIVER_PATH: &str = "../../../target/release/import-http-receiver";
const RECEIVER_CONFIG_PATH: &str = "../../../import/http/tests/config/receiver.toml";

pub struct System {
    sender: Option<Child>,
    receiver: Option<Child>,
    pub server: Server,
}

impl System {
    pub fn start() -> Self {
        let sender = Some(Self::spawn_process(SENDER_PATH, SENDER_CONFIG_PATH));
        let receiver = Some(Self::spawn_process(RECEIVER_PATH, RECEIVER_CONFIG_PATH));
        let server = Server::spawn();

        Self::wait_for_ready();
        Self {
            sender,
            receiver,
            server,
        }
    }

    fn spawn_process(path: &str, config_path: &str) -> Child {
        Command::new(path)
            .env("CONFIG_PATH", config_path)
            .env("RUST_LOG", "debug")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap()
    }

    fn wait_for_ready() {
        let client = client();
        retry::retry(retry::delay::Fixed::from_millis(100).take(10), || {
            client
                .get(&format!("http://{SENDER_ADDRESS}/is_ready"))
                .send()?
                .error_for_status()
        })
        .unwrap();
    }
}

impl Drop for System {
    fn drop(&mut self) {
        if let Some(sender) = self.sender.take() {
            begone_child("sender", sender);
        }
        if let Some(receiver) = self.receiver.take() {
            begone_child("receiver", receiver);
        }
    }
}

fn begone_child(name: &'static str, mut child: Child) {
    child.kill().unwrap();
    let output = child.wait_with_output().unwrap();
    print!(
        "---- {} stderr ----\n{}",
        name,
        String::from_utf8_lossy(&output.stderr)
    );
}
