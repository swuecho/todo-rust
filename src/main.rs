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
        let id = self.id.to_owned();
        format!("{}/{}/{}", host, path, id)
    }

}


struct TODO {
       operation: String,
       items: Vec<Item>
}


impl Handler for TODO {
    fn handle_request(&self, context: Context, mut response: Response) {
        println!("{}", self.operation);
        /*
        let json = match self.operation {
            "all" => "all",
            "one" => "one",
             _  => "unknown",
        };
        */
        let json = "json_string";
        if let Err(e) = response.into_writer().send(format!("Hello, {}!", json))  {
        //There is not much we can do now
        context.log.note(&format!("could not send hello: {}", e.description()));
        }

    }

}


fn main() {
    let items : Vec<Item> = Vec::new();
    let it = Item::new();

    let mut router = TreeRouter::new();
    router.insert(Get, "/todos",TODO { operation: "all".to_string(), items: items }  );
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