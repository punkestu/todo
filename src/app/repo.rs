use crate::{error, todo};

pub trait Todo {
    fn get(&self) -> error::Result<Vec<todo::Todo>>;
    fn save(&self, todo: &mut todo::Todo) -> error::Result<todo::Todo>;
    fn delete(&self, id: u32) -> error::Result<todo::Todo>;
}
