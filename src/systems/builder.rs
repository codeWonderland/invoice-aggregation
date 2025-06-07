use std::collections::HashSet;
use std::fs::{create_dir, exists, File};
use std::io::Write;

use chrono::{Datelike, NaiveDate};

use crate::TimeEntry;
use crate::{models::teamwork::Client, InternalError};



pub fn build_invoices(clients: Vec<Client>, last_month: bool) -> Result<(), InternalError> {
    // Check to see if output folder exists, if not create it
    if !exists("output").map_err(|e| InternalError::FileError(e.to_string()))? {
        create_dir("output").map_err(|open_err| InternalError::FileError(open_err.to_string()))?;
    }

    // Check to see if folder for the month exists, if not create it
    let now = chrono::Local::now();
    let month_start = NaiveDate::from_ymd_opt(now.year(), now.month(), 1).ok_or_else(|| InternalError::DateParsingError("Error Parsing Month Start".to_string()))?;
    let start_date = if last_month {
        month_start - chrono::Duration::days(7) // 7 is an arbitrary number, we just need it to be the previous month
    } else {
        month_start
    };
    let month_string = start_date.format("%Y-%B").to_string();

    if !exists(format!("output/{}", month_string)).map_err(|e| InternalError::FileError(e.to_string()))? {
        create_dir(format!("output/{}", month_string)).map_err(|open_err| InternalError::FileError(open_err.to_string()))?;
    }

    // For each client, create a file with the client's name containing the total time spent and tasks organized by list/date
    // Format:
    // Client Name
    // Total Time: x hours, y minutes
    // List Name (x hours, y minutes) | for list in lists
    // Date | for date in dates in list
    // Task Name (x hours, y minutes) - Description | for task in tasks in list on date
    for client in clients {
        let total_time = client.entries_by_list.iter()
            .map(|list| list.entries.iter()
                .map(|entry| entry.hours_spent * 60 + entry.minutes_spent)
                .sum::<i32>()
            )
            .sum::<i32>();

        if total_time == 0 {
            continue;
        }

        let mut client_file = File::create(format!("output/{}/{}.txt", month_string, client.name))
            .map_err(|e| InternalError::FileError(e.to_string()))?;

        writeln!(client_file, "{}", client.name).map_err(|e| InternalError::FileError(e.to_string()))?;
        writeln!(client_file, "Total Time: {} hours, {} minutes", total_time / 60, total_time % 60).map_err(|e| InternalError::FileError(e.to_string()))?;
        writeln!(client_file, "").map_err(|e| InternalError::FileError(e.to_string()))?;

        for list in client.entries_by_list {
            let list_time = list.entries.iter()
                .map(|entry| entry.hours_spent * 60 + entry.minutes_spent)
                .sum::<i32>();

            writeln!(client_file, "{} ({} hours, {} minutes)", list.list.name, list_time / 60, list_time % 60).map_err(|e| InternalError::FileError(e.to_string()))?;

            // Sort dates
            let mut dates = list.entries.iter().map(|entry| entry.date).collect::<HashSet<NaiveDate>>().into_iter().collect::<Vec<NaiveDate>>();
            dates.sort_by(|a, b| a.day0().cmp(&b.day0()));
            
            for date in dates {
                let date_entries = list.entries.iter().filter(|entry| entry.date == date).collect::<Vec<&TimeEntry>>();

                writeln!(client_file, "{}", date.format("%A, %B %-d")).map_err(|e| InternalError::FileError(e.to_string()))?;

                for entry in date_entries {
                    writeln!(client_file, "- {} ({} hours, {} minutes) - {}", entry.task_name, entry.hours_spent, entry.minutes_spent, entry.description).map_err(|e| InternalError::FileError(e.to_string()))?;
                }
            }

            writeln!(client_file, "").map_err(|e| InternalError::FileError(e.to_string()))?;
        }
    }

    Ok(())
}