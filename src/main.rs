use std::env;
mod app;
use app::{
    handler, lib,
    model::{error, model, request},
    repo,
    repo_impl::repo_impl,
    service,
};
// mod error;
// mod handler;
// mod model;
// mod repo;
// mod repo_impl;
// mod request;
// mod service;
// mod util;

fn main() {
    let args: Vec<String> = env::args().collect();
    let r = repo_impl::TodoImpl::new("data/data.json");
    let s = service::Todo::new(&r);
    let h = handler::Todo::new(&s);

    if args.len() < 2 {
        h.display_todo();
        return;
    }

    match args[1].as_str() {
        "create" => {
            h.create_todo(&args);
        }
        "toggle" => {
            h.toggle_todo(&args);
        }
        "delete" => {
            h.delete_todo(&args);
        }
        _ => h.unhandled(),
    }
}
