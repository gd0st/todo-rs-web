use actix_web::{App, HttpServer, web};
use todo_rs::TodoList;
use serde::Deserialize;
use std::sync::Mutex;
use std::net::Ipv4Addr;
use todo_backend::routes;

#[derive(Deserialize)]
struct Config {
    bind: Ipv4Addr,
    port: u32,
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    println!("Hello, world!");
    let config = Config {
        bind: Ipv4Addr::new(127, 0, 0, 1),
        port: 8080,
    };

    let list = TodoList::new("foobar");

    let mux_list = web::Data::new(
        Mutex::new(list)
    );

    HttpServer::new(move || {
        App::new().service(routes::test_todo).service(routes::add_todo).app_data(mux_list.clone())
    }).bind((config.bind, 8080))?.run().await


}
