#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate rustful;
extern crate serde;
extern crate uuid;

use serde::json;
use uuid::Uuid;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

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

impl fmt::Display for Item {
      fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "{}", json::to_string(&self).unwrap())
      }
}


struct Todo {
       items: Vec<Item>,
       handler_fn: fn(&Context, &Vec<Item>) -> String
}

fn show_all(c: &Context, items: &Vec<Item>) -> String {
    // fmt::Display for Vec<Item>
     format!("{}", items[0])
}

impl Handler for Todo {
    fn handle_request(&self, context: Context, mut response: Response) {

        let json = (self.handler_fn)(&context, &self.items) ;
       /* 
        let allow_origin =response.set_header(header!("access-control-allow-origin", "*"));
        let allowed_methods = AccessControlAllowMethods(pub Vec<Method>);
        let allowed_headers = AccessControlAllowHeaders(pub Vec<UniCase<String>>);

        response.set_header(allow_origin);   
        response.set_header(allowed_methods);   
        response.set_header(allowed_headers);
        */
        if let Err(e) = response.into_writer().send(json)  {
               context.log.note(&format!("could not send hello: {}", e.description()));
        }

    }

}


fn main() {
    let mut items : Vec<Item> = Vec::new();
    let it = Item::new();
    items.push(it);

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
