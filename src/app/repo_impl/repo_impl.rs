use crate::{
    error,
    lib::{read_from_json, save_to_json},
    model, repo,
};
use std::path::Path;

pub struct TodoImpl {
    path: &'static Path,
}

impl TodoImpl {
    pub fn new(path: &'static str) -> Self {
        TodoImpl {
            path: Path::new(path),
        }
    }
}

impl repo::Todo for TodoImpl {
    fn get(&self) -> error::Result<Vec<model::Todo>> {
        read_from_json(self.path)
    }
    fn save(&self, todo: &mut model::Todo) -> error::Result<model::Todo> {
        let mut users: Vec<model::Todo> = read_from_json(self.path)?;
        match todo.id {
            None => {
                match users.last() {
                    Some(last_user) => {
                        todo.id = Some(last_user.id.unwrap() + 1);
                    }
                    None => {
                        todo.id = Some(1);
                    }
                }
                users.push(todo.clone());
            }
            Some(id) => {
                if let Some(user) = users.iter_mut().find(|user| user.id.unwrap() == id) {
                    user.label = todo.label.to_owned();
                    user.state = todo.state;
                }
            }
        }

        save_to_json(self.path, &users)?;
        Ok(todo.to_owned())
    }
    fn delete(&self, id: u32) -> error::Result<model::Todo> {
        let mut users: Vec<model::Todo> = read_from_json(self.path)?;
        let mut deleted_user = model::Todo {
            ..Default::default()
        };
        if let Some(deleted_index) = users.iter().position(|user| user.id.unwrap() == id) {
            deleted_user = users[deleted_index].clone();
            users.remove(deleted_index);
        }
        save_to_json(self.path, &users)?;
        Ok(deleted_user)
    }
}
