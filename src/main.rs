mod to_do;
mod state;
mod processes;

use to_do::{to_do_factory, ItemTypes};
use std::env;
use to_do::structs::traits::get;
use crate::state::{read_file, write_to_file};
use serde_json::{Value, Map};
use crate::to_do::structs::traits::get::Get;
use crate::processes::process_input;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command: &String = &args[1];
    let title: &String = &args[2];

    let mut state: Map<String, Value> = read_file("./state.json");

    let status: String;

    match state.get(title) {
        Some(result) => {
            status = result.to_string().replace('\"', "")
        }
        None => {
            status = "pending".to_string();
        }
    }

    let item = to_do_factory(&status, title).expect(&status);

    process_input(item, command.to_string(), &state);
}
