use chrono::NaiveDate;

use crate::{models::errors::InternalError, systems::util::parse_date};

#[derive(Debug)]
pub(crate) struct Client {
    pub(crate) name: String,
    pub(crate) lists: Vec<TaskList>,
    pub(crate) entries_by_list: Vec<TimeEntriesByList>,
}

impl TryFrom<&serde_json::Value> for Client {
    type Error = InternalError;

    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        let name = value
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| InternalError::JsonParsing("name".to_string()))?
            .to_owned();
        let lists = value
            .get("lists")
            .and_then(|v| v.as_array())
            .ok_or_else(|| InternalError::JsonParsing("lists".to_string()))?
            .iter()
            .map(|list| -> Result<TaskList, InternalError> {
                let id = list
                    .get("id")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| InternalError::JsonParsing("id".to_string()))?
                    .to_owned();
                let name = list
                    .get("name")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| InternalError::JsonParsing("name".to_string()))?
                    .to_owned();

                Ok(TaskList { id, name })
            })
            .collect::<Result<Vec<TaskList>, InternalError>>()?;

        Ok(Client {
            name,
            lists,
            entries_by_list: vec![],
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct TaskList {
    pub(crate) id: String,   // tasklistId
    pub(crate) name: String, // todo-list-name
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct TimeEntry {
    pub(crate) hours_spent: i32,   // hours
    pub(crate) minutes_spent: i32, // minutes
    pub(crate) task_list: TaskList,
    pub(crate) task_name: String,   // todo-item-name
    pub(crate) description: String, // description
    pub(crate) date: NaiveDate,     // dateUserPerspective
}

impl TryFrom<serde_json::Value> for TimeEntry {
    type Error = InternalError;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        let hours_spent = value
            .get("hours")
            .and_then(|v| v.as_str())
            .ok_or_else(|| InternalError::TimeEntryParsing("hours".to_string()))?
            .parse::<i32>()
            .map_err(|e| InternalError::TimeEntryParsing(e.to_string()))?;
        let minutes_spent = value
            .get("minutes")
            .and_then(|v| v.as_str())
            .ok_or_else(|| InternalError::TimeEntryParsing("minutes".to_string()))?
            .parse::<i32>()
            .map_err(|e| InternalError::TimeEntryParsing(e.to_string()))?;
        let task_list_id = value
            .get("tasklistId")
            .and_then(|v| v.as_str())
            .ok_or_else(|| InternalError::TimeEntryParsing("tasklistId".to_string()))?
            .to_owned();
        let task_list = value
            .get("todo-list-name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| InternalError::TimeEntryParsing("todo-list-name".to_string()))?
            .to_owned();
        let task_name = value
            .get("todo-item-name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| InternalError::TimeEntryParsing("todo-list-name".to_string()))?
            .to_owned();
        let description = value
            .get("description")
            .and_then(|v| v.as_str())
            .ok_or_else(|| InternalError::TimeEntryParsing("todo-list-name".to_string()))?
            .to_owned();
        let timestamp_string = value
            .get("dateUserPerspective")
            .and_then(|v| v.as_str())
            .ok_or_else(|| InternalError::TimeEntryParsing("todo-list-name".to_string()))?;
        let date = parse_date(timestamp_string)?;

        Ok(TimeEntry {
            hours_spent,
            minutes_spent,
            task_list: TaskList {
                id: task_list_id,
                name: task_list,
            },
            task_name,
            description,
            date,
        })
    }
}

#[derive(Debug)]
pub(crate) struct TimeEntriesByList {
    pub(crate) list: TaskList,
    pub(crate) entries: Vec<TimeEntry>,
}

