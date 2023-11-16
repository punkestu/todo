use crate::model;

pub trait Todo<'a> {
    fn get(&self) -> Vec<model::Todo>;
    fn save(&self, todo: &'a mut model::Todo) -> &'a mut model::Todo;
    fn delete(&self, id: u32) -> model::Todo;
}
