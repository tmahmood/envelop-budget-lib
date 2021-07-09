use std::env::args;

use clap::{App, load_yaml};

use budget_manager::budgeting::budget::Budget;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();
    // now we do things
    if let Some(ref matches) = matches.subcommand_matches("spent") {
        if matches.is_present("category") && matches.is_present("amount") {
            println!(
                "Transaction added: BDT{} @ {}",
                matches.value_of("amount").unwrap(),
                matches.value_of("category").unwrap());
        }
    }
}