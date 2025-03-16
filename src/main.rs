pub(crate) mod models;
pub(crate) mod systems;

use clap::Parser;
use systems::{
    builder::build_invoices, 
    clients_parser::{
        associate_entries_with_clients, 
        read_clients
    }, 
    time_api::{
        get_time_entries, 
        prune_entries
    }, 
    util::remove_duplicates
};
use models::{args::Args, errors::InternalError, teamwork::{TimeEntriesByList, TimeEntry}};

#[tokio::main]
async fn main() -> Result<(), InternalError> {
    // Parse User Arguments
    let args: Args = dbg!(Args::parse());
    
    let mut entries = get_time_entries(args.last_month).await?;
    entries = prune_entries(entries, args.last_month)?;
    entries = remove_duplicates(entries);
    
    // Extra info for debug
    //let active_lists = entries.iter().map(|entry| entry.task_list.clone()).collect::<Vec<TaskList>>();
    //let unique_lists = remove_duplicates(active_lists);
    //println!("{:#?}", unique_lists);
    //println!("{:#?}", entries);
    
    let mut clients = read_clients()?;
    //println!("{:#?}", clients);

    associate_entries_with_clients(&mut clients, entries)?;
    //println!("{:#?}", clients);

    build_invoices(clients, args.last_month)?;
    
    Ok(())
}
