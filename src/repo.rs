use crate::model;

pub trait Todo {
    fn get(&self) -> Vec<model::Todo>;
    fn save(&self, todo: &mut model::Todo) -> model::Todo;
    fn delete(&self, id: u32) -> model::Todo;
}
