use crate::{model, repo};
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
impl repo::Todo for TodoImpl {
    fn get(&self) -> Vec<model::Todo> {
        let file = File::open(self.path).expect("error open file");
        let users: Vec<model::Todo> = serde_json::from_reader(file).expect("error parse data");
        users
    }
    fn save(&self, todo: &mut model::Todo) -> model::Todo {
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

        let writer = File::create(self.path).expect("error open file writer");
        serde_json::to_writer(writer, &users).expect("error writing");
        todo.to_owned()
    }
    fn delete(&self, id: u32) -> model::Todo {
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
        let writer = File::create(self.path).expect("error open file writer");
        serde_json::to_writer(writer, &users).expect("error writing");
        deleted_user
    }
}
