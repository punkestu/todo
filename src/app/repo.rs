use crate::{error, model};

pub trait Todo {
    fn get(&self) -> error::Result<Vec<model::Todo>>;
    fn save(&self, todo: &mut model::Todo) -> error::Result<model::Todo>;
    fn delete(&self, id: u32) -> error::Result<model::Todo>;
}
