use crate::error;
use std::{fs::File, path::Path};

pub mod db_conn;

pub fn read_from_json<T: serde::de::DeserializeOwned>(
    path: &'static Path,
) -> error::Result<Vec<T>> {
    match File::open(path) {
        Ok(reader) => match serde_json::from_reader(reader) {
            Err(_) => Err(error::Error::ParseFileFailed),
            Ok(users) => Ok(users),
        },
        Err(_) => Err(error::Error::LoadFileFailed),
    }
}

pub fn save_to_json<T: serde::Serialize>(path: &'static Path, payload: &T) -> error::Result<()> {
    match File::create(path) {
        Ok(writer) => match serde_json::to_writer(writer, payload) {
            Ok(_) => Ok(()),
            Err(_) => Err(error::Error::WriteFileFailed),
        },
        Err(_) => Err(error::Error::LoadFileFailed),
    }
}
