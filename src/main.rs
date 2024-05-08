use std::{fs::{self, DirEntry}, io, time::Instant};

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

        "search" => 'search: {

            let target: String;
            match cli.target {
                Some(t) => {
                    target = t;
                },
                None => {
                    println!("Provide a filepath to the target directory!");
                    break 'search;
                },
            }

            let option: String;
            match cli.option {
                Some(o) => {
                    option = o;
                },
                None => {
                    println!("Provide an option to search by!");
                    break 'search;
                }
            }

            match option.as_str() {

                "duplicates" => {



                },

                "by_criteria" => {

                    let mut files_of_criteria: Vec<DirEntry> = vec![];

                    let criteria: String;
                    match cli.criteria {
                        Some(c) => {
                            criteria = c;
                        },
                        None => {
                            println!("Provide a subcriteria to search by!");
                            break 'search;
                        }
                    }

                    match criteria.as_str() {

                        "by_name" => {

                            let file_names: Vec<String>;
                            match cli.file_names {
                                Some(fln) => {
                                    file_names = fln;
                                },
                                None => {
                                    println!("Provide file names to search for!");
                                    break 'search;
                                },
                            }

                            let now: Instant = Instant::now();

                            custodian::find_files_of_given_names(&target, &file_names, &mut files_of_criteria);

                            let elapsed: std::time::Duration = now.elapsed();
                            println!("\nCompleted in {:.2?}", elapsed);

                        },

                        "by_type" => {

                            let file_types: Vec<String>;
                            match cli.file_types {
                                Some(ft) => {
                                    file_types = ft;
                                },
                                None => {
                                    println!("Provide file types to search for!");
                                    break 'search;
                                },
                            }

                            let now: Instant = Instant::now();

                            custodian::find_files_of_given_types(&target, &file_types, &mut files_of_criteria);

                            let elapsed: std::time::Duration = now.elapsed();
                            println!("\nCompleted in {:.2?}", elapsed);

                        },

                        "by_last_modified" => {

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
                                            break 'search;
                                        },
                                    }
                                },
                                None => {
                                    println!("Provide a cutoff date!");
                                    break 'search;
                                },
                            }

                            let now: Instant = Instant::now();

                            custodian::find_files_last_modifed_before(&target, &cutoff_date, &mut files_of_criteria);

                            let elapsed: std::time::Duration = now.elapsed();
                            println!("\nCompleted in {:.2?}", elapsed);

                        },

                        "empty_directories" => {

                            let now: Instant = Instant::now();

                            custodian::find_empty_directories(&target, &mut files_of_criteria);

                            let elapsed: std::time::Duration = now.elapsed();
                            println!("\nCompleted in {:.2?}", elapsed);

                        },

                        _ => {
                            println!("Provide a valid subcriteria!")
                        },

                    }

                },

                _ => {
                    println!("Provide a valid option!");
                    break 'search;
                },

            }

        },

        "sweep" => {

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
