use http_muncher::{Parser,ParserHandler};
use mio::Ready;
use mio::tcp::TcpStream;
use std::io::Read;
use std::io::Write;
use http_parser::HttpParser;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use rustc_serialize::base64::{ToBase64,STANDARD};
use sha1;


 #[derive(PartialEq)]
pub enum ClientState
{
    AwaitingHandshake,
    HandshakeResponse,
    Connected
}


pub struct WebSocketClient {
    client_socket : TcpStream,
    http_parser : Parser,
    //  the headers declaration to the WebSocketClient struct
    headers : Rc<RefCell<HashMap<String,String>>>,
    interest: Ready,
    state : ClientState,
}

impl WebSocketClient 
{


    pub fn gen_key(key: &String) -> String 
   {
       let mut m = sha1::Sha1::new();
       let mut buffer = [0u8; 20];
       m.update(key.as_bytes());
       m.update(b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
       m.output(&mut buffer);

       buffer.to_base64(STANDARD)
    }


    pub fn new(socket : TcpStream) -> WebSocketClient 
    {
        let headers = Rc::new(RefCell::new(HashMap::new()));

        WebSocketClient 
        {
            client_socket : socket,
            http_parser : Parser::request(),
            headers : headers.clone(),
            interest : Ready::readable(),
            state :   ClientState::AwaitingHandshake,  
        }
    }

    pub fn get_socket_ref(&self) -> &TcpStream 
    {
        &self.client_socket
    }

    pub fn get_readinees(&self) -> Ready 
    {
        self.interest
    }

    pub fn read(&mut self) 
    {
        loop 
        {
            let mut buff = [0; 2048];
            match self.client_socket.read(&mut buff)
            {
                //error while reading socket
                Err(e) => 
                {
                    println!("Error while reading socket {:?}!",e);
                    return;
                },
                Ok(len) => 
                {
                    // len==0 indicates end of data
                    if len > 0
                    {
                        let mut parser = HttpParser::new(None,&self.headers);
                        self.http_parser.parse(&mut parser,&buff[0..len]);
                        if self.http_parser.is_upgrade() 
                        {
                              // Change the current state
                            self.state = ClientState::HandshakeResponse;
                            // Change current interest to `Writable`
                            self.interest.remove(Ready::readable());
                            self.interest.insert(Ready::writable());
                            break;
                        }
                    }
                }
            }

        }
    }

    pub fn write(&mut self) 
    {
         // Get the headers HashMap from the Rc<RefCell<...>> wrapper:
         let headers = self.headers.borrow();

         // Find the header that interests us, and generate the key from its value:
         let response_key = Self::gen_key(&headers.get("Sec-WebSocket-Key").unwrap());

         // We're using special function to format the string.
        // You can find analogies in many other languages, but in Rust it's performed
        // at the compile time with the power of macros. We'll discuss it in the next
        // part sometime.
         let response = fmt::format(format_args!("HTTP/1.1 101 Switching Protocols\r\n\
                                                 Connection: Upgrade\r\n\
                                                 Sec-WebSocket-Accept: {}\r\n\
                                                 Upgrade: websocket\r\n\r\n", response_key));

        // Write the response to the socket:
        self.client_socket.write(response.as_bytes()).unwrap();

         // Change the state:
        self.state = ClientState::Connected;

        // And change the interest back to `readable()`:
        self.interest.remove(Ready::writable());
        self.interest.insert(Ready::readable());

    }
}