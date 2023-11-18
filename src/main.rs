use std::env;
mod error;
mod model;
mod repo;
mod repo_impl;
mod service;

fn main() {
    let args: Vec<String> = env::args().collect();
    let r = repo_impl::TodoImpl::new("data/data.json");
    let s = service::Todo::new(&r);

    if args.len() < 2 {
        s.display_all();
        return;
    }

    match args[1].as_str() {
        "create" => {
            if args.len() < 3 {
                println!("create todo need label");
                return;
            }
            match s.create_one(service::CreateOne {
                label: args[2].to_owned(),
            }) {
                Ok(todo) => {
                    println!("todo created with id: {}", todo.id.unwrap());
                }
                Err(err) => error::map_and_print_error(err),
            }
        }
        "toggle" => {
            if args.len() < 3 {
                println!("create todo need id");
                return;
            }
            match s.toggle_state(service::ToggleState {
                id: args[2].parse::<u32>().unwrap(),
            }) {
                Ok(todo) => {
                    println!("todo with id {} updated", todo.id.unwrap())
                }
                Err(err) => error::map_and_print_error(err),
            }
        }
        "delete" => {
            if args.len() < 3 {
                println!("create todo need id");
                return;
            }
            match s.deleted(service::Delete {
                id: args[2].parse::<u32>().unwrap(),
            }) {
                Ok(todo) => {
                    println!("todo with id {} deleted", todo.id.unwrap())
                }
                Err(err) => error::map_and_print_error(err),
            }
        }
        _ => {}
    }
}
