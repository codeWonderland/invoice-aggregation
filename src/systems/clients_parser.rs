use std::{fs::File, result::Result};

use serde_json::{from_reader, Value};

use crate::{models::{
    errors::InternalError, 
    teamwork::Client
}, TimeEntriesByList, TimeEntry};


pub fn read_clients() -> Result<Vec<Client>, InternalError> {
    let file = File::open("clients.json")
        .map_err(|e| InternalError::FileError(e.to_string()))?;
    let json: Value = from_reader(file)
        .map_err(|e| InternalError::JsonParsingError(e.to_string()))?;
    let clients = json.as_array().ok_or(InternalError::JsonParsingError("clients".to_string()))?
        .iter()
        .map(|client| Client::try_from(client))
        .collect::<Result<Vec<Client>, InternalError>>()?;

    Ok(clients)
}

pub fn associate_entries_with_clients(
    clients: &mut Vec<Client>, 
    entries: Vec<TimeEntry>
) -> Result<(), InternalError> {
    for entry in entries {
        let mut list = entry.task_list.clone();
        let client = clients.iter_mut().find(|client| client.lists.iter().find(|client_list| *client_list.id == list.id).is_some())
            .ok_or(InternalError::ClientNotFoundError(format!("No client found for list: {:#?}", list)))?;

        // Get the list from the client so we have the public facing name
        list = client.lists.iter().find(|client_list| client_list.id == list.id).unwrap().clone();
        
        match client.entries_by_list.iter_mut().find(|e| e.list.id == list.id) {
            Some(list) => {
                list.entries.push(entry.clone());
            },
            None => {
                client.entries_by_list.push(TimeEntriesByList {
                    list: list.clone(),
                    entries: vec![entry.clone()]
                });
            }
        }
    }

    Ok(())
}