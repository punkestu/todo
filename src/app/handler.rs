use crate::{error, repo, request, service};

pub struct Todo<'a, T: repo::Todo> {
    s: &'a service::Todo<'a, T>,
}

impl<'a, T: repo::Todo> Todo<'a, T> {
    pub fn new(s: &'a service::Todo<'a, T>) -> Self {
        Todo { s }
    }
}

impl<'a, T: repo::Todo> Todo<'a, T> {
    pub fn display_todo(&self) {
        match self.s.get_all() {
            Ok(users) => {
                for user in users {
                    println!("{}", user);
                }
            }
            Err(err) => error::map_and_print_error(err),
        }
    }
    pub fn create_todo(&self, args: &'a Vec<String>) {
        if args.len() < 3 {
            println!("create todo need label");
            return;
        }
        match self.s.create_one(request::CreateOne {
            label: args[2].to_owned(),
        }) {
            Ok(todo) => {
                println!("todo created with id: {}", todo.id.unwrap());
            }
            Err(err) => error::map_and_print_error(err),
        }
    }
    pub fn toggle_todo(&self, args: &'a Vec<String>) {
        if args.len() < 3 {
            println!("create todo need id");
            return;
        }
        match args[2].parse::<u32>() {
            Ok(id) => match self.s.toggle_state(request::ToggleState { id }) {
                Ok(todo) => {
                    println!("todo with id {} updated", todo.id.unwrap())
                }
                Err(err) => error::map_and_print_error(err),
            },
            Err(_) => println!("id not valid"),
        }
    }
    pub fn delete_todo(&self, args: &'a Vec<String>) {
        if args.len() < 3 {
            println!("create todo need id");
            return;
        }

        match args[2].parse::<u32>() {
            Ok(id) => match self.s.deleted(request::Delete { id }) {
                Ok(todo) => {
                    println!("todo with id {} deleted", todo.id.unwrap())
                }
                Err(err) => error::map_and_print_error(err),
            },
            Err(_) => println!("id not valid"),
        }
    }
    pub fn unhandled(&self) {
        println!("command cannot be handled");
    }
}
