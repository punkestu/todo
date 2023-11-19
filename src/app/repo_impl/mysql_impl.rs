use crate::app::{
    model::{error::Result, todo},
    repo,
};
use sqlx::{MySqlPool, Row};
use tokio::runtime::Runtime;

pub struct TodoImpl<'a> {
    pool: &'a MySqlPool,
    rt: &'a Runtime,
}

impl<'a> TodoImpl<'a> {
    pub fn new(pool: &'a MySqlPool, rt: &'a Runtime) -> Self {
        TodoImpl { pool, rt }
    }
}

impl<'a> repo::Todo for TodoImpl<'a> {
    fn get(&self) -> Result<Vec<todo::Todo>> {
        self.rt.block_on(async {
            let rows = sqlx::query("SELECT * FROM todo")
                .fetch_all(self.pool)
                .await
                .unwrap();
            let todos = rows
                .iter()
                .map(|row| todo::Todo {
                    id: Some(row.get::<u32, _>("id")),
                    label: row.get::<String, _>("label"),
                    state: row.get::<bool, _>("state"),
                })
                .collect::<Vec<todo::Todo>>();
            Ok(todos)
        })
    }
    fn save(&self, todo: &mut todo::Todo) -> Result<todo::Todo> {
        match todo.id {
            Some(id) => self.rt.block_on(async {
                sqlx::query("UPDATE todo SET label=?, state=? WHERE id=?")
                    .bind(&todo.label)
                    .bind(todo.state)
                    .bind(id)
                    .execute(self.pool)
                    .await
                    .unwrap();
                Ok(todo.to_owned())
            }),
            None => self.rt.block_on(async {
                let result = sqlx::query("INSERT INTO todo VALUES (DEFAULT, ?, DEFAULT)")
                    .bind(&todo.label)
                    .execute(self.pool)
                    .await
                    .unwrap();
                todo.id = Some(result.last_insert_id() as u32);
                Ok(todo.to_owned())
            }),
        }
    }
    fn delete(&self, id: u32) -> Result<todo::Todo> {
        self.rt.block_on(async {
            let deleted = sqlx::query("SELECT * FROM todo WHERE id=?")
                .bind(id)
                .fetch_one(self.pool)
                .await
                .unwrap();
            sqlx::query("DELETE FROM todo WHERE id=?")
                .bind(id)
                .execute(self.pool)
                .await
                .unwrap();
            Ok(todo::Todo {
                id: Some(deleted.get::<u32, _>("id")),
                label: deleted.get::<String, _>("label"),
                state: deleted.get::<bool, _>("state"),
            })
        })
    }
}
