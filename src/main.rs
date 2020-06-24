
use ws::{ Factory, Sender, WebSocket, Handler, Error as WebSocketError };
use url::Url;

#[derive(Clone)]
struct Client {}

impl Factory for Client {
    type Handler = Self;

    fn connection_made(&mut self, _: Sender) -> Self::Handler {
        println!("Connection created");
        self.clone()
    }

    fn connection_lost(&mut self, _: Self::Handler) {
        println!("Connection lost");
    }
}

impl Handler for Client {
    fn on_error(&mut self, err: WebSocketError) {
        println!("Error in WebSocket communication: {:?}", err);
    }
}

fn main() {
     // Setup logging
    env_logger::init();

    let client = Client {};
    let mut ws = WebSocket::new(client).unwrap();
    let url = Url::parse("wss://yeticgi.casualos.com/socket.io/?EIO=3&transport=websocket").unwrap();

    if let Err(e) = ws.connect(url) {
        println!("Failed to establish a WebSocket connection: {:?}", e);
        return;
    };

    if let Err(e) = ws.run() {
        println!("Failed to run WebSocket: {:?}", e);
    };
}
