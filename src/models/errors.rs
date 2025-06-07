#![allow(dead_code)]
#[derive(Debug)]
pub(crate) enum InternalError {
    TimeEntryParsing(String),
    DateParsing(String),
    NetworkRequest(String),
    JsonParsing(String),
    File(String),
    ClientNotFound(String),
}

