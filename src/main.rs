pub(crate) mod models;
pub(crate) mod systems;

use clap::Parser;
use models::{
    args::Args,
    errors::InternalError,
    teamwork::{TimeEntriesByList, TimeEntry},
};
use systems::{
    builder::build_invoices,
    clients_parser::{associate_entries_with_clients, read_clients},
    time_api::{get_time_entries, prune_entries},
    util::remove_duplicates,
};

#[tokio::main]
async fn main() -> Result<(), InternalError> {
    // Parse User Arguments
    let args: Args = dbg!(Args::parse());

    let mut entries = get_time_entries(args.last_month).await?;
    entries = prune_entries(entries, args.last_month)?;
    entries = remove_duplicates(entries);

    let mut clients = read_clients()?;

    associate_entries_with_clients(&mut clients, entries)?;

    build_invoices(clients, args.last_month)?;

    Ok(())
}
