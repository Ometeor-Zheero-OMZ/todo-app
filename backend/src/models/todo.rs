use sqlx::{FromRow, PgPool, Error};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize};

#[derive(Debug, Clone, FromRow)]
pub struct TodoItem {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct CreateTodoItem {
    pub title: String,
    pub completed: bool,
}

#[derive(Deserialize)]
pub struct UpdateTodoItem {
    pub title: Option<String>,
    pub completed: Option<bool>,
}

impl TodoItem {
    pub async fn create_todo(
        db: &PgPool,
        title: String,
        completed: bool,
    ) -> Result<TodoItem, Error> {
        let id = Uuid::new_v4();
        let created_at = Utc::now();
        
        let row = sqlx::query!(
            r#"
            INSERT INTO todos (id, title, completed, created_at)
            VALUES ($1, $2, $3, $4)
            RETURNING id, title, completed, created_at
            "#,
            id,
            title,
            completed,
            created_at
        )
        .fetch_one(db)
        .await?;

        Ok(TodoItem {
            id: row.id,
            title: row.title,
            completed: row.completed,
            created_at: row.created_at,
        })
    }

    pub async fn update_todo(
        db: &PgPool,
        id: Uuid,
        title: Option<String>,
        completed: Option<bool>,
    ) -> Result<TodoItem, Error> {
        let todo: TodoItem = sqlx::query_as!(
            TodoItem,
            r#"
            SELECT id, title, completed, created_at
            FROM todos
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(db)
        .await?;

        let new_title = title.unwrap_or(todo.title);
        let new_completed = completed.unwrap_or(todo.completed);
        
        sqlx::query!(
            r#"
            UPDATE todos
            SET title = $1, completed = $2
            WHERE id = $3
            "#,
            new_title,
            new_completed,
            id
        )
        .execute(db)
        .await?;

        Ok(TodoItem {
            id: todo.id,
            title: new_title,
            completed: new_completed,
            created_at: todo.created_at,
        })
    }

    pub async fn delete_todo(
        db: &PgPool,
        id: Uuid,
    ) -> Result<(), Error> {
        sqlx::query!(
            r#"
            DELETE FROM todos
            WHERE id = $1
            "#,
            id
        )
        .execute(db)
        .await?;

        Ok(())
    }
}
