use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use actix_web::post;
#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Todo {
    subject: String,
}
#[derive(Deserialize, Debug, Serialize)]
pub struct TodoList {
    name: String,
    todos: Vec<Todo>,
}

impl TodoList {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            todos: vec![],
        }
    }

    async fn add(&mut self, todo: Todo) {
        self.todos.push(todo)
    }


}
pub mod routes {
    use todo_rs::{Todo, TodoList};
    use actix_web::{get, post, Responder, Result, HttpResponse, body::MessageBody, web};
    use std::sync::Mutex;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct NewTodo {
        subject: String,
    }
    #[get("/health_check")]
    pub async fn health_check() -> Result<impl Responder> {
        Ok(HttpResponse::Ok())
    }

    #[get("/test_todo")]
    pub async fn test_todo() -> Result<impl Responder> {
        let todo = Todo::new("foobar");
        let text = serde_json::to_string(&todo)?;
        println!("{}", text);
        Ok(HttpResponse::Ok().body(serde_json::to_string(&todo)?))
    }

    // TODO need to fix main todo library to receive from Todo struct instead of the current setup.
    #[post("/add_todo")]
    pub async fn add_todo(data: web::Data<Mutex<TodoList>>, item: web::Json<Todo>) -> Result<impl Responder> {
        let mut list = data.lock().unwrap();
        list.new_todo("new item");
        println!("new todo!");
        Ok(HttpResponse::Ok())
    }
}
