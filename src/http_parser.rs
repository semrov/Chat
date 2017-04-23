extern crate http_muncher;
use http_muncher::{Parser, ParserHandler};
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std;

pub struct HttpParser 
{
    current_key : Option<String>,
    headers : Rc<RefCell<HashMap<String,String>>>,
}

impl HttpParser 
{
    pub fn new(current_key : Option<String>, headers : &Rc<RefCell<HashMap<String,String>>>) -> HttpParser
    {
        HttpParser 
        {
             current_key : current_key,
             headers : headers.clone(),
        }
    }
}

impl ParserHandler for HttpParser 
{
    fn on_header_field(&mut self, parser : &mut Parser, header : &[u8]) -> bool 
    {
        self.current_key = Some(std::str::from_utf8(header).unwrap().to_string() );
        true
    }

    fn on_header_value(&mut self, parser: &mut Parser, value: &[u8]) -> bool 
    {
        self.headers.borrow_mut()
            .insert(self.current_key.clone().unwrap(),
                std::str::from_utf8(value).unwrap().to_string()
                );
        true
    }

    fn on_headers_complete(&mut self, parser : &mut Parser) -> bool 
    {
        false
    }
}