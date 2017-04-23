extern crate mio;
extern crate http_muncher;
extern crate sha1;
extern crate rustc_serialize;

 pub mod server;
 pub mod client;
 pub mod http_parser;

//use std::io::Read;
use server::WebSocketServer;
use client::WebSocketClient;
use mio::tcp::{TcpListener,TcpStream};
use mio::{Ready, Poll, PollOpt, Token,Events};
use std::net::{SocketAddr};



const SERVER : Token = Token(0);


fn main() {
    // Bind a server socket to connect to.
    let sock_address : SocketAddr = "127.0.0.1:25543".parse().unwrap();
    let server_socket = TcpListener::bind(&sock_address).expect("Error while binding to socket!");
    let mut web_socket_server = WebSocketServer::new(server_socket);

    // Setup the server socket
    let pool = Poll::new().unwrap();

    
    // Create storage for events
    let mut events = Events::with_capacity(1024);


    // Start listening for incoming connections
    pool.register(web_socket_server.get_ref_socket(),SERVER,Ready::readable(),PollOpt::edge()).unwrap();

    // Setup the client socket
   // let client_socket = TcpStream::connect(&sock_address).expect("Client: Error while connecting to socket!");

    //pool.register(&client_socket,CLIENT,Ready::readable(),PollOpt::edge()).unwrap();

    loop
    {
        pool.poll(&mut events, None).unwrap();

        for event in events.iter()
        {
             //  Dealing with the read event
            if event.readiness().is_readable()
            {
                // The listening socket generate readable events when a new client arrives
                match event.token() 
                {
                    // make sure that the event has sourced from the listening socket 
                    SERVER => 
                    {
                            let (stream, _client_address) = match web_socket_server.get_ref_socket().accept() 
                            {
                                Ok((stream, client_address)) => (stream, client_address),
                                Err(e) =>   
                                {
                                    println!("Accept error: {}", e); 
                                    continue;    
                                }
                            };

                            let token = web_socket_server.get_new_token();
                            pool.register(&stream,token,Ready::readable(),PollOpt::edge() | PollOpt::oneshot()).unwrap();
                            web_socket_server.insert_client_socket(token,WebSocketClient::new(stream));

                    },
                    token => 
                    {
                        let mut client = web_socket_server.get_client_mut_ref(&token).unwrap();
                        client.read();
                        pool.reregister(client.get_socket_ref(),token,client.get_readinees(),PollOpt::edge() | PollOpt::oneshot()).unwrap();
                    },
                }
            }
            if event.readiness().is_writable() 
            {
                match event.token() {
                   SERVER => panic!("SERVER WRITABLE!"),
                   token => 
                   {
                        let mut client = web_socket_server.get_client_mut_ref(&token).unwrap();
                        client.write();
                        pool.reregister(client.get_socket_ref(), token, client.get_readinees(),PollOpt::edge() | PollOpt::oneshot()).unwrap();
                   },
                }
            }
        }
    }


    


    //let  mut s = String::new();
   // std::io::stdin().read_line(&mut s);


}
