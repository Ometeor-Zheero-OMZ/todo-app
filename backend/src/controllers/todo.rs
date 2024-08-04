use crate::models::todo::{TodoItem, CreateTodoItem, UpdateTodoItem};
use actix_web::{get, post, put, web, HttpResponse, Responder, delete};
use sqlx::PgPool;
use uuid::Uuid;

#[get("")]
pub async fn get_todos(db: web::Data<PgPool>) -> impl Responder {
    let todos = sqlx::query_as!(TodoItem, 
        r#"
        SELECT id, title, completed, created_at 
        FROM todos
        "#
    )
    .fetch_all(db.get_ref())
    .await;

    match todos {
        Ok(todo_list) => HttpResponse::Ok().json(todo_list),
        Err(_) => HttpResponse::NotFound().body("タスクが見つかりませんでした。"),
    }
}

#[post("")]
pub async fn add_todo(item: web::Json<CreateTodoItem>, db: web::Data<PgPool>) -> impl Responder {
    let new_todo = TodoItem::create_todo(db.get_ref(), item.title.clone(), item.completed).await;

    match new_todo {
        Ok(todo) => HttpResponse::Created().json(todo),
        Err(_) => HttpResponse::InternalServerError().body("タスクの追加に失敗しました。"),
    }
}

#[put("/{id}")]
pub async fn update_todo(
    path: web::Path<Uuid>,
    item: web::Json<UpdateTodoItem>,
    db: web::Data<PgPool>
) -> impl Responder {
    let result = TodoItem::update_todo(db.get_ref(), path.into_inner(), item.title.clone(), item.completed.clone()).await;

    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::NotFound().body("更新可能なタスクがありません。"),
    }
}

#[delete("/{id}")]
pub async fn delete_todo(path: web::Path<Uuid>, db: web::Data<PgPool>) -> impl Responder {
    let result = TodoItem::delete_todo(db.get_ref(), path.into_inner()).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("タスクが削除されました。"),
        Err(_) => HttpResponse::NotFound().body("削除可能なタスクがありません。"),
    }
}
