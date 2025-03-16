use std::collections::HashSet;
use std::hash::Hash;

use chrono::NaiveDate;

use crate::models::errors::InternalError;

pub(crate) fn parse_date(timestamp_str: &str) -> Result<NaiveDate, InternalError> {
    let datestamp = timestamp_str.split("T").next().ok_or(InternalError::DateParsingError("No date found".to_string()))?;
    let date = chrono::NaiveDate::parse_from_str(datestamp, "%Y-%m-%d").map_err(|e| InternalError::DateParsingError(e.to_string()))?;

    Ok(date)
}

pub(crate) fn remove_duplicates<T: Clone + Eq + Hash>(items: Vec<T>) -> Vec<T> {
    let unique_items: HashSet<T> = items.into_iter().collect();
    unique_items.into_iter().collect()
}