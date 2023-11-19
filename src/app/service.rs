use crate::{error, model, repo, request};

pub struct Todo<'a, T: repo::Todo> {
    r: &'a T,
}

impl<'a, T: repo::Todo> Todo<'a, T> {
    pub fn new(r: &'a T) -> Self {
        Todo { r }
    }
}

impl<'a, T: repo::Todo> Todo<'a, T> {
    pub fn get_all(&self) -> error::Result<Vec<model::Todo>> {
        self.r.get()
    }
    pub fn get_by_id(&self, params: request::GetById) -> error::Result<model::Todo> {
        let users = self.r.get()?;
        match users.iter().find(|_todo| _todo.id.unwrap() == params.id) {
            Some(_todo) => Ok(_todo.clone()),
            None => Err(error::Error::TodoNotFound),
        }
    }
    pub fn create_one(&self, params: request::CreateOne) -> error::Result<model::Todo> {
        self.r.save(&mut model::Todo {
            label: params.label,
            ..Default::default()
        })
    }
    pub fn toggle_state(&self, params: request::ToggleState) -> error::Result<model::Todo> {
        let todo = self.get_by_id(request::GetById { id: params.id })?;
        self.r.save(&mut model::Todo {
            id: Some(params.id),
            label: todo.label,
            state: !todo.state,
        })
    }
    pub fn deleted(&self, params: request::Delete) -> error::Result<model::Todo> {
        self.get_by_id(request::GetById { id: params.id })?;
        self.r.delete(params.id)
    }
}
