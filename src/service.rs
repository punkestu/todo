use crate::{model, repo_impl};

pub struct Todo {
    _repo: repo_impl::TodoImpl,
}

pub struct GetById {
    pub id: u32,
}

pub struct CreateOne {
    pub label: String,
}

pub struct UpdateOne {
    pub id: u32,
    pub label: String,
    pub state: bool,
}

pub struct DeleteById {
    pub id: u32,
}

impl Todo {
    pub fn new(_repo: repo_impl::TodoImpl) -> Self {
        Todo { _repo }
    }
    pub fn display_all(&self) {
        println!("==TODO LIST==");
        println!("-------------");
        for user in self._repo.get() {
            println!("{}", user);
        }
    }
    pub fn get_by_id(&self, params: GetById) -> model::Todo {
        match self
            ._repo
            .get()
            .iter()
            .find(|_todo| _todo.id.unwrap() == params.id)
        {
            Some(_todo) => _todo.clone(),
            None => model::Todo {
                id: None,
                label: String::from(""),
                state: false,
            },
        }
    }
    pub fn create_one(&self, params: CreateOne) -> model::Todo {
        self._repo.save(&mut model::Todo {
            id: None,
            label: params.label,
            state: false,
        })
    }
    pub fn update_one(&self, params: UpdateOne) -> model::Todo {
        self._repo.save(&mut model::Todo {
            id: Some(params.id),
            label: params.label,
            state: params.state,
        })
    }
    pub fn deleted_by_id(&self, params: DeleteById) -> model::Todo {
        self._repo.delete(params.id)
    }
}
