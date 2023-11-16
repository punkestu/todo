use std::env;
mod model;
mod repo;
mod repo_impl;
mod service;

fn main() {
    let args: Vec<String> = env::args().collect();
    let r = repo_impl::TodoImpl::new("data/data.json");
    let s = service::Todo::new(r);

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
            s.create_one(service::CreateOne {
                label: args[2].to_owned(),
            });
        }
        "toggle" => {
            if args.len() < 3 {
                println!("create todo need id");
                return;
            }
            let _todo = s.get_by_id(service::GetById {
                id: args[2].parse::<u32>().unwrap(),
            });
            match _todo.id {
                Some(_) => {
                    s.update_one(service::UpdateOne {
                        id: _todo.id.unwrap(),
                        label: _todo.label,
                        state: !_todo.state,
                    });
                }
                None => {
                    println!("todo not found");
                }
            }
        }
        "delete" => {
            if args.len() < 3 {
                println!("create todo need id");
                return;
            }
            let _todo = s.get_by_id(service::GetById {
                id: args[2].parse::<u32>().unwrap(),
            });
            match _todo.id {
                Some(_) => {
                    s.deleted_by_id(service::DeleteById {
                        id: _todo.id.unwrap(),
                    });
                }
                None => {
                    println!("todo not found");
                }
            }
        }
        _ => {}
    }
}
