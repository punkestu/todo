use crate::{error, model, repo};

pub struct Todo<'a> {
    _repo: &'a dyn repo::Todo,
}

pub struct GetById {
    pub id: u32,
}

pub struct CreateOne {
    pub label: String,
}

pub struct ToggleState {
    pub id: u32,
}

pub struct Delete {
    pub id: u32,
}

impl<'a> Todo<'a> {
    pub fn new(_repo: &'a impl repo::Todo) -> Self {
        Todo { _repo }
    }
    pub fn display_all(&self) {
        match self._repo.get() {
            Ok(users) => {
                for user in users {
                    println!("{}", user);
                }
            }
            Err(err) => error::map_and_print_error(err),
        }
    }
    pub fn get_by_id(&self, params: GetById) -> error::Result<model::Todo> {
        let users = self._repo.get()?;
        match users.iter().find(|_todo| _todo.id.unwrap() == params.id) {
            Some(_todo) => Ok(_todo.clone()),
            None => Err(error::Error::TodoNotFound),
        }
    }
    pub fn create_one(&self, params: CreateOne) -> error::Result<model::Todo> {
        self._repo.save(&mut model::Todo {
            label: params.label,
            ..Default::default()
        })
    }
    pub fn toggle_state(&self, params: ToggleState) -> error::Result<model::Todo> {
        let todo = self.get_by_id(GetById { id: params.id })?;
        self._repo.save(&mut model::Todo {
            id: Some(params.id),
            label: todo.label,
            state: !todo.state,
        })
    }
    pub fn deleted(&self, params: Delete) -> error::Result<model::Todo> {
        self.get_by_id(GetById { id: params.id })?;
        self._repo.delete(params.id)
    }
}
