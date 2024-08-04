use actix_web::web::{self, get, post, put, delete};

use crate::controllers::
    todo::{
        get_todos,
        add_todo,
        update_todo,
        delete_todo,
    };

pub fn config(conf: &mut web::ServiceConfig) {
    let todo_scope = web::scope("/api/todos")
        .service(get_todos)
        .service(add_todo)
        .service(update_todo)
        .service(delete_todo);

    conf.service(todo_scope);
}