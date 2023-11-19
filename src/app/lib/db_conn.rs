use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use tokio::runtime::Runtime;

pub fn gen_pool(rt: &Runtime) -> Result<Pool<MySql>, String> {
    rt.block_on(async {
        match std::env::var("DATABASE_URL") {
            Ok(url) => Ok(MySqlPoolOptions::new()
                .max_connections(1)
                .connect(&url)
                .await
                .unwrap()),
            Err(_) => Err(String::from("no url to connect")),
        }
    })
}
