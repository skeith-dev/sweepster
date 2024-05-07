use std::{fs, io, time::Instant};

use chrono::NaiveDate;
use clap::Parser;

mod app;
mod cli;
mod custodian;
mod prompts;


fn main() {

    let cli: cli::Cli = cli::Cli::parse();

    match cli.action.as_str() {

        "Sweepster" => {
            app::run();
        },

        "store" => 'store: {

            let target: String;
            match cli.target {
                Some(t) => {
                    target = t;
                },
                None => {
                    println!("Provide a filepath to the target directory!");
                    break 'store;
                },
            }

            let storage_path: String;
            match cli.storage_path {
                Some(sp) => {
                    storage_path = sp;
                },
                None => {
                    println!("Provide a filepath to the storage directory!");
                    break 'store;
                }
            }

            let file_separator: String = String::from("/"); //FIXME

            let cutoff_date: NaiveDate;
            match cli.cutoff_date {
                Some(cutoff_date_string) => {
                    match NaiveDate::parse_from_str(cutoff_date_string.as_str(), "%Y-%m-%d") {
                        Ok(date) => {
                            cutoff_date = date;
                        },
                        Err(err) => {
                            println!("Could not parse &str to NaiveDate!");
                            println!("Ensure proper formatting (ex. 2020-01-01)");
                            println!("{}", err);
                            break 'store;
                        },
                    }
                },
                None => {
                    println!("Provide a cutoff date!");
                    break 'store;
                },
            }

            let create_storage_result: Result<(), io::Error> = fs::create_dir(&storage_path);
            match create_storage_result {

                Ok(_) => {

                    let now: Instant = Instant::now();

                    custodian::generate_storage(&target, &storage_path, &file_separator, &cutoff_date);

                    let elapsed: std::time::Duration = now.elapsed();
                    println!("\nCompleted in {:.2?}", elapsed);

                },

                Err(err) => {
                    println!("Could not create directory at path: {}", storage_path);
                    println!("{}", err);
                    break 'store;
                },

            }


        },

        _ => { },

    }
    
}
