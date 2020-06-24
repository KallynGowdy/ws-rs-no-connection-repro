
use ws::{ Factory, Sender, WebSocket, Handler, Error as WebSocketError, Message, Result as WebSocketResult };
use url::Url;
use std::io;
use std::thread;
use std::io::Write;

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
    fn on_message(&mut self, message: Message) -> WebSocketResult<()> {
        let res = match message {
            Message::Text(message) => {
                println!("Server: {}", message);
                Ok(())
            },
            Message::Binary(_) => Ok(()),
        };

        res
    }

    fn on_error(&mut self, err: WebSocketError) {
        println!("Error in WebSocket communication: {:?}", err);
    }
}

fn main() {
     // Setup logging
    env_logger::init();

    let client = Client {};
    let mut ws = WebSocket::new(client).unwrap();
    
    let stdin = io::stdin();
    println!("Choose an option:");
    println!("0) ws://echo.websocket.org <-- This one works");
    println!("1) wss://echo.websocket.org <-- This one does not");
    print!("Type an option: ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    stdin.read_line(&mut choice).unwrap();
    let url = match choice.chars().next().unwrap() {
        // This choice works
        '0' => Url::parse("ws://echo.websocket.org").unwrap(),

        // This choice fails
        '1' => Url::parse("wss://echo.websocket.org").unwrap(),

        _ => panic!("You did not choose between 0 and 1"),
    };

    if let Err(e) = ws.connect(url) {
        println!("Failed to establish a WebSocket connection: {:?}", e);
        return;
    };

    let ws_sender = ws.broadcaster();
    println!("Got sender!");

    thread::spawn(move || {
        loop {
            let mut input = String::new();
            match stdin.read_line(&mut input) {
                Ok(_) => {
                    ws_sender.send(input).unwrap();
                },
                Err(error) => {
                    println!("Input error: {}", error);
                }
            }
        }
    });
    
    println!("run!");
    if let Err(e) = ws.run() {
        println!("Failed to run WebSocket: {:?}", e);
    };
}
