use actix_web::{web, App, HttpServer};
use serde::Deserialize;
use std::net::Ipv4Addr;
use std::sync::Mutex;
use todo_rs::TodoList;
use todo_rs_web::routes;

#[derive(Deserialize)]
struct Config {
    bind: Ipv4Addr,
    port: u32,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let config = Config {
        bind: Ipv4Addr::new(127, 0, 0, 1),
        port: 8080,
    };

    let list = TodoList::new("foobar");

    let mux_list = web::Data::new(Mutex::new(list));

    HttpServer::new(move || {
        App::new()
            .service(routes::health_check)
            .service(routes::test_todo)
            .service(routes::add_todo)
            .service(routes::get_todos)
            .app_data(mux_list.clone())
    })
    .bind((config.bind, 8080))?
    .run()
    .await
}
