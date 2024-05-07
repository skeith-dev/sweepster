use std::collections::HashMap;
use std::fs::DirEntry;
use std::{fs, io};
use std::time::Instant;
use chrono::NaiveDate;
use terminal_fonts::to_block_string;

use crate::prompts::parse_prompt;
use crate::{custodian, prompts};


pub fn run() {
    
    println!("\n{}", to_block_string("SWEEPSTER"));
    println!("\nYour very own command-line file custodian\n");

    loop {

        let action: u8 = prompts::parse_prompt::<u8>("1. Search\n2. Sweep\n3. Store");
        let criteria: u8;
        let sub_criteria: u8;
        let dir_path: String = prompts::string_prompt("Enter the path of the target directory:");
        
        match action {

            //Search or Sweep
            1..=2 => {

                criteria = prompts::parse_prompt::<u8>("1. For duplicates\n2. By criteria");

                match criteria {

                    1 => {

                        sub_criteria = parse_prompt::<u8>("1. By name\n2. By contents");
                        let mut duplicate_files: Vec<(DirEntry, String)> = vec![];

                        match sub_criteria {

                            1 => {

                                let now: Instant = Instant::now();

                                let mut extension_counts: HashMap<String, u32> = HashMap::new();
                                custodian::count_files_by_type(&dir_path, &mut extension_counts);
                                println!("\n{:?}", extension_counts);

                                let mut file_cabinet: HashMap<String, Vec<DirEntry>> = HashMap::with_capacity(extension_counts.len());
                                for (key, value) in extension_counts.into_iter() {
                                    file_cabinet.insert(key, Vec::with_capacity(value as usize));
                                }

                                custodian::organize_files_by_type(&dir_path, &mut file_cabinet);

                                for value in file_cabinet.values_mut() {
                                    custodian::find_duplicates_by_name(value, &mut duplicate_files);
                                }

                                let elapsed: std::time::Duration = now.elapsed();
                                println!("\nCompleted in {:.2?}", elapsed);

                            },

                            2 => {

                                let print_flag: bool = prompts::parse_prompt::<bool>("Enable print flag (enter \"true\" or \"false\")? Enabling will print each file comparison as it occurs.");

                                let now: Instant = Instant::now();

                                let mut extension_counts: HashMap<String, u32> = HashMap::new();
                                custodian::count_files_by_type(&dir_path, &mut extension_counts);
                                println!("\n{:?}", extension_counts);

                                let mut file_cabinet: HashMap<String, Vec<DirEntry>> = HashMap::with_capacity(extension_counts.len());
                                for (key, value) in extension_counts.into_iter() {
                                    file_cabinet.insert(key, Vec::with_capacity(value as usize));
                                }

                                custodian::organize_files_by_type(&dir_path, &mut file_cabinet);

                                for value in file_cabinet.values_mut() {
                                    custodian::find_duplicates_by_contents(value, &mut duplicate_files, print_flag);
                                }

                                let elapsed: std::time::Duration = now.elapsed();
                                println!("\nCompleted in {:.2?}", elapsed);

                            },

                            
                            _ => { },

                        }

                        let duplicate_files_bundle: Vec<[String; 5]> = custodian::bundle_duplicate_files(duplicate_files);
                        let csv_path: String = prompts::string_prompt("Enter the path of the CSV file to export search results to:");
                        custodian::export_duplicate_files_to_csv(csv_path.as_str(), duplicate_files_bundle);

                    },

                    //By criteria
                    2 => {

                        sub_criteria = prompts::parse_prompt::<u8>("1. By name\n2. By type\n3. By last modified\n4. Empty directories");
                        let mut files_of_criteria: Vec<DirEntry> = vec![];

                        match sub_criteria {

                            //By name
                            1 => {

                                let file_names: Vec<String> = prompts::strings_prompt("Enter the file names to search for, INCLUDING the file extension, separated by a single space:");

                                let now: Instant = Instant::now();

                                custodian::find_files_of_given_names(&dir_path, &file_names, &mut files_of_criteria);

                                let elapsed: std::time::Duration = now.elapsed();
                                println!("\nCompleted in {:.2?}", elapsed);

                            },

                            //By type
                            2 => {

                                let file_types: Vec<String> = prompts::strings_prompt("Enter the file types to search for, NOT INCLUDING the \".\", separated by a single space:");

                                let now: Instant = Instant::now();

                                custodian::find_files_of_given_types(&dir_path, &file_types, &mut files_of_criteria);

                                let elapsed: std::time::Duration = now.elapsed();
                                println!("\nCompleted in {:.2?}", elapsed);

                            },

                            //By last modified
                            3 => {

                                let cutoff_date: NaiveDate = prompts::date_prompt("Enter the cutoff date, formatted as YYYY-mm-dd (ex. 2020-01-01)");

                                let now: Instant = Instant::now();

                                custodian::find_files_last_modifed_before(&dir_path, &cutoff_date, &mut files_of_criteria);

                                let elapsed: std::time::Duration = now.elapsed();
                                println!("\nCompleted in {:.2?}", elapsed);

                            },

                            //Empty directories
                            4 => {

                                let now: Instant = Instant::now();

                                custodian::find_empty_directories(&dir_path, &mut files_of_criteria);

                                let elapsed: std::time::Duration = now.elapsed();
                                println!("\nCompleted in {:.2?}", elapsed);

                            },

                            _ => { },

                        }

                        let found_files_bundle: Vec<[String; 4]> = custodian::bundle_found_files(files_of_criteria);
                        let csv_path: String = prompts::string_prompt("Enter the path of the CSV file to export search results to:");
                        custodian::export_found_files_to_csv(csv_path.as_str(), found_files_bundle);

                    },

                    _ => { },

                }

            },

            //Store
            3 => {

                let storage_path: String = prompts::string_prompt("Enter the path of the storage directory to generate:");
                let create_storage_result: Result<(), io::Error> = fs::create_dir(&storage_path);
                match create_storage_result {

                    Ok(_) => {

                        let file_separator: String = prompts::string_prompt("Enter the file separator character of your OS (\"/\" for Unix, \"\\\" for Windows):");
                        let cutoff_date: NaiveDate = prompts::date_prompt("Enter the cutoff date, formatted as YYYY-mm-dd (ex. 2020-01-01)");

                        let now: Instant = Instant::now();

                        custodian::generate_storage(&dir_path, &storage_path, &file_separator, &cutoff_date);

                        let elapsed: std::time::Duration = now.elapsed();
                        println!("\nCompleted in {:.2?}", elapsed);

                    },

                    Err(err) => {
                        println!("Could not create directory at path: {}", storage_path);
                        println!("{}", err);
                    },

                }

            },

            _ => { },

        }

    }

}
