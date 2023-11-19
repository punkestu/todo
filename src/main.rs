use std::env;
mod app;
use app::{
    handler, lib,
    model::{error, request, todo},
    repo,
    repo_impl::mysql_impl,
    service,
};
use dotenv::dotenv;
use tokio::runtime::Runtime;

fn main() {
    let args: Vec<String> = env::args().collect();
    dotenv().ok();

    let rt = Runtime::new().unwrap();
    let pool = lib::db_conn::gen_pool(&rt).unwrap();

    // let r = todo_impl::TodoImpl::new("data/data.json");
    let r: mysql_impl::TodoImpl = mysql_impl::TodoImpl::new(&pool, &rt);
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
