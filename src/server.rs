use std::collections::HashMap;
use client::WebSocketClient;
use mio::tcp::{TcpListener,TcpStream};
use mio::{Ready, Poll, PollOpt, Token,Events};


pub struct WebSocketServer
{
    server_socket : TcpListener,
    clients : HashMap<Token, WebSocketClient>,
    token_counter : usize,
}

impl WebSocketServer 
{
    pub fn new(socket : TcpListener) -> WebSocketServer 
    {
        WebSocketServer 
        {
            server_socket : socket,
            clients : HashMap::new(),
            token_counter : 1,
        }
    }

    pub fn get_ref_socket(&self) -> &TcpListener 
    {
        &self.server_socket
    }

    pub fn get_new_token(&mut self) -> Token
    {
        let token = Token(self.token_counter);
        self.token_counter += 1;
        token
    }

   pub fn insert_client_socket(&mut self, token : Token, client_socket : WebSocketClient) 
   {
       self.clients.insert(token,client_socket);
   }

   pub fn get_client_ref(&self, token : &Token) -> Option<&WebSocketClient> 
   {
       self.clients.get(&token)
   }

    pub fn get_client_mut_ref(&mut self, token : &Token) -> Option<&mut WebSocketClient> 
   {
       self.clients.get_mut(&token)
   }


   pub fn remove_client(&mut self, token : &Token) -> Option<WebSocketClient> 
   {
        self.clients.remove(&token)
   }

  
}