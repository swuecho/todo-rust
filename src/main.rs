#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate rustful;
extern crate serde;
extern crate uuid;

use serde::json;
use uuid::Uuid;
use std::collections::HashMap;
use std::error::Error;

use rustful::{Server, Context, Response, Router, TreeRouter, Handler};
use rustful::Method::{Get, Post};
use rustful::mime::{Mime, TopLevel, SubLevel, Attr, Value};

fn new_id() -> String {
    Uuid::new_v4().to_string()
}


#[derive(Debug,Serialize, Deserialize)]
struct Item {
     id   :     String,
     title:     Option<String>,
     completed : bool,
     order  :   u32,
     text   :   Option<String>,
}

impl Item  {

    fn new() -> Item {
        let id = new_id();
        Item { id : id, completed: false, order: 0,  title: None, text: None }
    }

    fn url(&self) -> String {
        let host = "http://todo-backend-rust.herokuapp.com";
        let path = "todo";
        format!("{}/{}/{}", host, path, self.id)
    }

}


struct Todo {
       items: Vec<Item>,
       handler_fn: fn(&Context, &Vec<Item>) -> String
}

fn show_all(c: &Context, items: &Vec<Item>) -> String {
    "abcdefg".to_string()
}

impl Handler for Todo {
    fn handle_request(&self, context: Context, mut response: Response) {
        let json = (self.handler_fn)(&context, &self.items) ;
        if let Err(e) = response.into_writer().send(format!("Hello, {}!", json))  {
        context.log.note(&format!("could not send hello: {}", e.description()));
        }

    }

}


fn main() {
    let items : Vec<Item> = Vec::new();
    let it = Item::new();

    let mut router = TreeRouter::new();
    router.insert(Get, "/todos",Todo { handler_fn: show_all, items: items }  );
    //router.insert(Get, "/todos/:id",TODO { operation: "find".to_string(), items: items }  );

    // server
    let server_result = Server {
        host: 8080.into(),
        handlers: router ,
        // TODO: too verbose
        content_type: Mime(
                             TopLevel::Application,
                             SubLevel::Json,
                             vec![(Attr::Charset, Value::Utf8)]
                                                                         ),
        ..Server::default()
    }.run();

    match server_result {
        Ok(_server) => {},
        Err(e) => println!("could not start server: {}", e.description())
    }
}
