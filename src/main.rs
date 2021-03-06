#![allow(dead_code)]
#![allow(unused_variables)] 
#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate rustful;
extern crate serde;
extern crate uuid;
extern crate unicase;

use std::fmt;
use std::io::Read;
use std::error::Error;
use std::sync::{Arc, RwLock};

use rustful::{Server, Context, Response, Router, TreeRouter, Handler};
use rustful::Method::{Get, Head, Post, Delete, Options, Put, Patch};
use rustful::header::{AccessControlAllowOrigin, AccessControlAllowMethods,
                      AccessControlAllowHeaders};
use rustful::mime::{Mime, TopLevel, SubLevel, Attr, Value};

use serde::json;
use uuid::Uuid;
use unicase::UniCase;

fn new_id() -> String {
    Uuid::new_v4().to_string()
}


#[derive(Debug,Serialize, Deserialize)]
struct Item {
     id   :     Option<String>,
     title:     Option<String>,
     completed: Option<bool>,
     order  :   Option<u32>,
     text   :   Option<String>,
}

impl Item  {

    fn new() -> Item {
        let id = new_id();
        Item { id : Some(id), completed: Some(false), order: Some(0),  title:None, text: None}
    }

    /*fn url(&self) -> String {
        let host = "http://todo-backend-rust.herokuapp.com";
        let path = "todo";
        format!("{}/{}/{}", host, path, self.id.unwrap().to_owned())
    }
    */

}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", json::to_string(&self).unwrap())
    }
}

type RwItems = Arc<RwLock<Vec<Item>>>;

struct Todo {
       items: RwItems,
       handler_fn: fn(&mut Context, &RwItems) -> String
}


fn vec2str(vec_items: &Vec<Item>) -> String {
    let vec_str: Vec<String> = vec_items.iter().map(|x| { format!("{}", x) }).collect();
    format!("[{}]", vec_str.connect(","))
}

fn show_all(c: &mut Context, items: &RwItems) -> String {
    //TODO: fmt::Display for Vec<Item>
    vec2str(&(*(items.read().unwrap())))
}

    
fn add_item(c: &mut Context, items: &RwItems) -> String {
    let mut string = String::new();
    //let ref body_reader =  c.body_reader.by_ref();
    c.read_to_string(&mut string).unwrap();
    let mut item: Item = json::from_str(&string).unwrap();
    item.id = Some(new_id());
    let mut w = items.write().unwrap();
    (*w).push(item);
    "".to_string()
}

fn find_item(c: &mut Context, items: &RwItems) -> String {
    if let Some(id) = c.variables.get("id") {
    //TODO: return the right id 
    // items.inter().filter(|x| { (x.id) == id} )
    format!("{}", items.read().unwrap()[1])
    } else {
    format!("{}", "")
    }
}

fn status_ok(c: &mut Context, items: &RwItems) -> String {
    //TODO: fmt::Display for Vec<Item>
    format!("{}", "")
}

impl Handler for Todo {
    fn handle_request(&self, mut context: Context, mut response: Response) {

        let json = (self.handler_fn)(&mut context, &(self.items)) ;

        let allowed_headers = AccessControlAllowHeaders(vec![UniCase("accept".to_string()), UniCase("content-type".to_string())]);
        let allowed_methods = AccessControlAllowMethods(vec![Get,Head,Post,Delete,Options,Put,Patch]);
        response.set_header(allowed_headers);
        response.set_header(allowed_methods);
        response.set_header(AccessControlAllowOrigin::Any);
        if let Err(e) = response.into_writer().send(json)  {
               context.log.note(&format!("could not send hello: {}", e.description()));
        }

    }

}


fn main() {
    let mut items : Vec<Item> = Vec::new();
    let it0 = Item::new();
    let it1 = Item::new();
    items.push(it0);
    items.push(it1);
    let shared = Arc::new(RwLock::new(items));

    let mut router = TreeRouter::new();
    //router.insert(Options, "/todos", |_: Context, response: Response| {
    //           response.set_status(StatusCode::Ok);}) ;
    //router.insert(Options, "/todos", HandlerFn(say_hello));
    router.insert(Options, "/todos",Todo { handler_fn: status_ok, items: shared.clone() } );
    router.insert(Get, "/todos",Todo { handler_fn: show_all, items: shared.clone() }  );
    router.insert(Post, "/todos",Todo { handler_fn: add_item, items: shared.clone() }  );

    router.insert(Options, "/todos/:id",Todo { handler_fn: status_ok, items: shared.clone() } );
    router.insert(Get, "/todos/:id",Todo { handler_fn: find_item, items: shared.clone() } );

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
