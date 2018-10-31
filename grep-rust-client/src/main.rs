// A WebSocket client that sends one message then closes
extern crate ws;

// use ws::{connect, CloseCode};
// use ws::{connect, Handler, Sender, Handshake, Result, Message, CloseCode};
// use ws::{connect, CloseCode, Handler, Sender, Handshake, Result, Message};
use ws::{connect, Handler, Sender, Handshake, Result, Message, CloseCode};


struct Client {
    out: Sender,
}

// We implement the Handler trait for Client so that we can get more
// fine-grained control of the connection.
impl Handler for Client {

    // `on_open` will be called only after the WebSocket handshake is successful
    // so at this point we know that the connection is ready to send/receive messages.
    // We ignore the `Handshake` for now, but you could also use this method to setup
    // Handler state or reject the connection based on the details of the Request
    // or Response, such as by checking cookies or Auth headers.
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        // Now we don't need to call unwrap since `on_open` returns a `Result<()>`.
        // If this call fails, it will only result in this connection disconnecting.
        self.out.send("Hello WebSocket")
    }

    // `on_message` is roughly equivalent to the Handler closure. It takes a `Message`
    // and returns a `Result<()>`.
    fn on_message(&mut self, msg: Message) -> Result<()> {
        // Close the connection when we get a response from the server
        println!("Got message: {}", msg);
        self.out.close(CloseCode::Normal)
    }
    
}

fn main() {
  connect("ws://127.0.0.1:3012", |out| Client { out: out } ).unwrap()
} 
