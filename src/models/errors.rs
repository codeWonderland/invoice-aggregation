#![allow(dead_code)]
#[derive(Debug)]
pub(crate) enum InternalError {
    TimeEntryParsingError(String),
    DateParsingError(String),
    NetworkRequestError(String),
    JsonParsingError(String),
    FileError(String),
    ClientNotFoundError(String),
}