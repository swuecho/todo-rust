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


fn say_hello(context: Context, response: Response) {
    //Get the value of the path variable `:person`, from below.
    let person = match context.variables.get("person") {
        Some(name) => &name[..],
        None => "stranger"
    };

    //Use the value of the path variable to say hello.
    if let Err(e) = response.into_writer().send(format!("Hello, {}!", person))  {
        //There is not much we can do now
        context.log.note(&format!("could not send hello: {}", e.description()));
    }
}

type TODO<'a> = HashMap<String,&'a Item>;

fn todo_all(context: Context, response: Response) {

    //let mut todo = TODO::new();
    //let mut it = Item::new();

    //todo.insert(it.id.to_owned(), &it);
    //Get the value of the path variable `:person`, from below.
    let json_string = "[1,2,3]"; //json::to_string(&todo).unwrap().to_owned();
    //Use the value of the path variable to say hello.
    if let Err(e) = response.into_writer().send(format!("{}!", json_string ))  {
        //There is not much we can do now
        context.log.note(&format!("could not send hello: {}", e.description()));
    }
}


//Dodge an ICE, related to functions as handlers.
struct HandlerFn(fn(Context, Response));
impl Handler for HandlerFn {
    fn handle_request(&self, context: Context, response: Response) {
        self.0(context, response);
    }
}



fn main() {

    let mut router = TreeRouter::new();
    router.insert(Get, "/" , HandlerFn(say_hello));
    router.insert(Get, "/:person",  HandlerFn(say_hello));
    router.insert(Get, "/todos",HandlerFn(todo_all));


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
