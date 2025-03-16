use std::{env, thread::sleep, time::Duration};
use base64::{Engine as _, engine::general_purpose};
use chrono::{Datelike, NaiveDate};

use crate::models::{errors::InternalError, teamwork::TimeEntry};

pub async fn get_time_entries(last_month: bool) -> Result<Vec<TimeEntry>, InternalError> {
    dotenv::dotenv().ok();

    let max_page_offset = 90;
    let max_api_calls = 140;
    let mut num_api_calls = 0;

    let mut parsed_entries: Vec<TimeEntry> = vec![];
    let now = chrono::Local::now();
    let month_start = NaiveDate::from_ymd_opt(now.year(), now.month(), 1).ok_or_else(|| InternalError::DateParsingError("Error Parsing Month Start".to_string()))?;
    let mut start_date = if last_month {
        month_start - chrono::Duration::days(7) // 7 is an arbitrary number, we just need it to be the previous month
    } else {
        month_start
    };
    start_date = NaiveDate::from_ymd_opt(start_date.year(), start_date.month(), 1).ok_or_else(|| InternalError::DateParsingError("Error Parsing Month Start".to_string()))?;
    let mut start_date_str = start_date.format("%Y%m%d").to_string();
    
    let mut current_page = 1;
    let mut current_entries = get_time_entries_helper(start_date_str.clone(), 0).await?;

    parsed_entries.extend(current_entries.clone());
    let mut num_entries = current_entries.len();

    while num_entries == 500 {
        // update the current page offset, and ensure we don't go over the max
        current_page += 1;
        if current_page > max_page_offset {
            current_page = 1;
            start_date = parsed_entries.last().unwrap().date; // we know we can unwrap here because we just added entries and didn't error
            start_date_str = start_date.format("%Y%m%d").to_string();
        }


        // Sleep for 60 seconds if we've hit the API call limit
        num_api_calls += 1;
        if num_api_calls >= max_api_calls {
            sleep(Duration::from_secs(60));
            num_api_calls = 0;
        }
        current_entries = get_time_entries_helper(start_date_str.clone(), current_page).await?;
        parsed_entries.extend(current_entries.clone());
        num_entries = current_entries.len();
    }

    Ok(parsed_entries)
}

async fn get_time_entries_helper(start_date_str: String, offset: i32) -> Result<Vec<TimeEntry>, InternalError> {
    let api_key: String = env::var("TEAMWORK_API_KEY").expect("TEAMWORK_API_KEY must be set");

    let client = reqwest::Client::new();
    let auth_key = general_purpose::STANDARD.encode(
        format!("{}:f", api_key)
    );
    let req_url = format!("https://codewonderland.teamwork.com/time_entries.json?pageSize=500&page={}&fromdate={}", offset, start_date_str);
    let res = client.get(req_url)
        .header("Authorization", format!("Basic {}", auth_key))
        .send()
        .await
        .map_err(|e| InternalError::NetworkRequestError(e.to_string()))?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| InternalError::JsonParsingError(e.to_string()))?;

    let mut parsed_entries: Vec<TimeEntry> = Vec::new(); 

    res.get("time-entries").map(|time_entries| {
        for time_entry in time_entries.as_array().unwrap() {
            match TryInto::<TimeEntry>::try_into(time_entry) {
                Ok(entry) => parsed_entries.push(entry),
                Err(e) => eprintln!("Error parsing time entry: {:#?}", e),
            }
        }
    });

    Ok(parsed_entries)
}


pub fn prune_entries(entries: Vec<TimeEntry>, last_month: bool) -> Result<Vec<TimeEntry>, InternalError> {
    let now = chrono::Local::now();
    let month_start = NaiveDate::from_ymd_opt(now.year(), now.month(), 1).ok_or_else(|| InternalError::DateParsingError("Error Parsing Month Start".to_string()))?;
    let start_date = if last_month {
        month_start - chrono::Duration::days(7) // 7 is an arbitrary number, we just need it to be the previous month
    } else {
        month_start
    };

    Ok(entries.into_iter().filter(|entry: &TimeEntry | entry.date.month() == start_date.month()).collect())
}