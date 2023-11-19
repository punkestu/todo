pub const LOAD_FILE_FAILED: &str = "failed when loading file";
pub const WRITE_FILE_FAILED: &str = "failed when write file";
pub const PARSE_DATA_FAILED: &str = "failed when parse data";
pub const TODO_NOT_FOUND: &str = "todo not found";

#[derive(Debug)]
pub enum Error {
    TodoNotFound,
    LoadFileFailed,
    ParseFileFailed,
    WriteFileFailed,
}
pub type Result<T> = core::result::Result<T, Error>;

pub fn map_and_print_error(err: Error) {
    match err {
        Error::TodoNotFound => {
            println!("{}", TODO_NOT_FOUND);
        }
        Error::ParseFileFailed => {
            println!("{}", PARSE_DATA_FAILED);
        }
        Error::LoadFileFailed => {
            println!("{}", LOAD_FILE_FAILED);
        }
        Error::WriteFileFailed => {
            println!("{}", WRITE_FILE_FAILED);
        }
    }
}
