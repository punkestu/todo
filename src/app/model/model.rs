use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Todo {
    pub id: Option<u32>,
    pub label: String,
    pub state: bool,
}

impl Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.id {
            Some(id) => {
                write!(
                    f,
                    "{} | {} | {}",
                    id,
                    self.label,
                    if self.state { "X" } else { "_" }
                )
            }
            None => {
                write!(
                    f,
                    "_ | {} [{}]",
                    self.label,
                    if self.state { "X" } else { "_" }
                )
            }
        }
    }
}
