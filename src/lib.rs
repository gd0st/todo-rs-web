use actix_web::post;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
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
    use actix_web::{
        body::MessageBody,
        get, post,
        web::{self, JsonBody},
        HttpResponse, Responder, Result,
    };
    use serde::Deserialize;
    use std::sync::Mutex;
    use todo_rs::{Todo, TodoList};

    #[derive(Deserialize)]
    struct NewTodo {
        subject: String,
    }
    #[get("/health_check")]
    pub async fn health_check() -> Result<impl Responder> {
        Ok(HttpResponse::Ok())
    }

    #[get("/todos")]
    pub async fn get_todos(list: web::Data<Mutex<TodoList>>) -> Result<impl Responder> {
        let list = list.lock().unwrap();

        let todos: Vec<&Todo> = list.iter().collect();
        let text = serde_json::to_string(&todos).unwrap();
        Ok(HttpResponse::Ok().body(text))
    }

    #[get("/test_todo")]
    pub async fn test_todo() -> Result<impl Responder> {
        let todo = Todo::new("foobar");
        let text = serde_json::to_string(&todo)?;
        println!("get {}", text);
        Ok(HttpResponse::Ok().body(serde_json::to_string(&todo)?))
    }

    // TODO need to fix main todo library to receive from Todo struct instead of the current setup.
    #[post("/add_todo")]
    pub async fn add_todo(
        data: web::Data<Mutex<TodoList>>,
        //item: web::Json<Todo>,
        item: String,
    ) -> Result<impl Responder> {
        let todo: Todo = serde_json::from_str(&item)?;
        let subject = todo.subject();
        let mut list = data.lock().unwrap();
        list.add(todo);
        println!("Added todo {} to {}", subject, list.name());
        Ok(HttpResponse::Ok())
    }
}
