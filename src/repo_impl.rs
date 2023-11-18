use crate::{error, model, repo};
use std::{fs::File, path::Path};

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

fn save_to_json<T: serde::Serialize>(path: &'static Path, payload: &T) -> error::Result<()> {
    match File::create(path) {
        Ok(writer) => match serde_json::to_writer(writer, payload) {
            Ok(_) => Ok(()),
            Err(_) => Err(error::Error::WriteFileFailed),
        },
        Err(_) => Err(error::Error::LoadFileFailed),
    }
}

impl repo::Todo for TodoImpl {
    fn get(&self) -> error::Result<Vec<model::Todo>> {
        match File::open(self.path) {
            Ok(file) => match serde_json::from_reader(file) {
                Ok(users) => Ok(users),
                Err(_) => Err(error::Error::ParseFileFailed),
            },
            Err(_) => Err(error::Error::LoadFileFailed),
        }
    }
    fn save(&self, todo: &mut model::Todo) -> error::Result<model::Todo> {
        let reader = File::open(self.path).expect("error open file reader");
        let mut users: Vec<model::Todo> =
            serde_json::from_reader(reader).expect("error parse data");
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
        let reader = File::open(self.path).expect("error open file reader");
        let mut users: Vec<model::Todo> =
            serde_json::from_reader(reader).expect("error parse data");
        let mut deleted_user = model::Todo {
            id: None,
            label: String::from(""),
            state: false,
        };
        if let Some(deleted_index) = users.iter().position(|user| user.id.unwrap() == id) {
            deleted_user = users[deleted_index].clone();
            users.remove(deleted_index);
        }
        save_to_json(self.path, &users)?;
        Ok(deleted_user)
    }
}
