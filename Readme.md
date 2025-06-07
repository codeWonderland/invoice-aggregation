# Invoice Aggregation System for Teamwork
Compiles hours for the past month into a clean, organized, and itemized list that can be copy/pasted into invoicing systems such as Bill, Wave, or something similar.

## Setup
Rename `.example.env` to env, and insert your teamwork api key
Set up your clients and lists in `clients.json`, using `clients.example.json` as a reference. If you skip this step, the program will tell you which lists you are missing when you run it. This also serves the purpose of keeping your `clients.json` up to date over time by giving you information on what you need to add as your clients and lists expand

Please ensure that rust is installed first before attempting to run.

## Running the Program
From the project root, run `cargo run`. To run the report for the previous month, you can pass the `-l` parameter, like so:
`cargo run -- -l`

You can also build the project with `cargo build --release`, however you will still want to run it from a directory where you have a `.env` and `clients.json`. Alternatively, you don't strictly _need_ to have a `.env` if you've added your teamwork api key to your global environment variables, as this project just uses dotenv to load the values from `.env` into your environment variables.
