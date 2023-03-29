use std::collections::HashMap;
use std::fs::{DirEntry, self};
use std::io;
use std::time::Instant;
use chrono::NaiveDate;
use terminal_fonts::to_block_string;

mod custodian;


fn main() {
    
    println!("\n{}", to_block_string("SWEEPSTER"));
    println!("\nYour very own command-line file custodian\n");

    loop {

        let selection: u8 = menu();

        match selection {

            //Search a directory for duplicate files BY NAME
            1 => {

                let dir_path: String = string_prompt("Enter the path of the directory to search BY NAME:");

                let now: Instant = Instant::now();

                let mut extension_counts: HashMap<String, u32> = HashMap::new();
                custodian::count_files_by_type(&dir_path, &mut extension_counts);
                println!("\n{:?}", extension_counts);

                let mut file_cabinet: HashMap<String, Vec<DirEntry>> = HashMap::with_capacity(extension_counts.len());
                for (key, value) in extension_counts.into_iter() {
                    file_cabinet.insert(key, Vec::with_capacity(value as usize));
                }

                custodian::organize_files_by_type(&dir_path, &mut file_cabinet);

                let mut duplicate_files: Vec<DirEntry> = vec![];
                for value in file_cabinet.values_mut() {
                    custodian::find_duplicates_by_name(value, &mut duplicate_files);
                }

                let elapsed: std::time::Duration = now.elapsed();
                println!("\nCompleted in {:.2?}", elapsed);

                let duplicate_files_bundle: Vec<[String; 4]> = custodian::bundle_found_files(duplicate_files);
                let csv_path: String = string_prompt("Enter the path of the CSV file to export search results to:");
                custodian::export_found_files_to_csv(csv_path.as_str(), duplicate_files_bundle);

            },

            //Search a directory for duplicate files BY SIZE
            2 => {

                let dir_path: String = string_prompt("Enter the path of the directory to search BY CONTENTS:");

                let now: Instant = Instant::now();

                let mut extension_counts: HashMap<String, u32> = HashMap::new();
                custodian::count_files_by_type(&dir_path, &mut extension_counts);
                println!("\n{:?}", extension_counts);

                let mut file_cabinet: HashMap<String, Vec<DirEntry>> = HashMap::with_capacity(extension_counts.len());
                for (key, value) in extension_counts.into_iter() {
                    file_cabinet.insert(key, Vec::with_capacity(value as usize));
                }

                custodian::organize_files_by_type(&dir_path, &mut file_cabinet);

                let mut duplicate_files: Vec<DirEntry> = vec![];
                for value in file_cabinet.values_mut() {
                    custodian::find_duplicates_by_contents(value, &mut duplicate_files);
                }

                let elapsed: std::time::Duration = now.elapsed();
                println!("\nCompleted in {:.2?}", elapsed);

                let duplicate_files_bundle: Vec<[String; 4]> = custodian::bundle_found_files(duplicate_files);
                let csv_path: String = string_prompt("Enter the path of the CSV file to export search results to:");
                custodian::export_found_files_to_csv(csv_path.as_str(), duplicate_files_bundle);

            },

            //Search a directory for files of a GIVEN NAME
            3 => {

                let dir_path: String = string_prompt("Enter the path of the directory to search:");
                let file_names: Vec<String> = strings_prompt("Enter the file names to search for, NOT INCLUDING the file extension, separated by a single space:");

                let now: Instant = Instant::now();

                let mut files_of_names: Vec<DirEntry> = vec![];
                custodian::find_files_of_given_names(&dir_path, &file_names, &mut files_of_names);

                let elapsed: std::time::Duration = now.elapsed();
                println!("\nCompleted in {:.2?}", elapsed);

                let found_files_bundle: Vec<[String; 4]> = custodian::bundle_found_files(files_of_names);
                let csv_path: String = string_prompt("Enter the path of the CSV file to export search results to:");
                custodian::export_found_files_to_csv(csv_path.as_str(), found_files_bundle);

            },

            //Search a directory for files of a GIVEN TYPE
            4 => {

                let dir_path: String = string_prompt("Enter the path of the directory to search:");
                let file_types: Vec<String> = strings_prompt("Enter the file types to search for, NOT INCLUDING the \".\", separated by a single space:");

                let now: Instant = Instant::now();

                let mut files_of_types: Vec<DirEntry> = vec![];
                custodian::find_files_of_given_types(&dir_path, &file_types, &mut files_of_types);

                let elapsed: std::time::Duration = now.elapsed();
                println!("\nCompleted in {:.2?}", elapsed);

                let found_files_bundle: Vec<[String; 4]> = custodian::bundle_found_files(files_of_types);
                let csv_path: String = string_prompt("Enter the path of the CSV file to export search results to:");
                custodian::export_found_files_to_csv(csv_path.as_str(), found_files_bundle);

            },

            //Search a directory for files last modified before a GIVEN CUTOFF DATE
            5 => {

                let dir_path: String = string_prompt("Enter the path of the directory to search:");
                let cutoff_date: NaiveDate = date_prompt("Enter the cutoff date, formatted as YYYY-mm-dd (ex. 2020-01-01)");

                let now: Instant = Instant::now();

                let mut files_last_modified_before: Vec<DirEntry> = vec![];
                custodian::find_files_last_modifed_before(&dir_path, &cutoff_date, &mut files_last_modified_before);

                let elapsed: std::time::Duration = now.elapsed();
                println!("\nCompleted in {:.2?}", elapsed);

                let found_files_bundle: Vec<[String; 4]> = custodian::bundle_found_files(files_last_modified_before);
                let csv_path: String = string_prompt("Enter the path of the CSV file to export search results to:");
                custodian::export_found_files_to_csv(csv_path.as_str(), found_files_bundle);

            },

            //Search a directory for empty directories (folders)
            6 => {

                let dir_path: String = string_prompt("Enter the path of the directory to search:");

                let now: Instant = Instant::now();

                let mut empty_directories: Vec<DirEntry> = vec![];
                custodian::find_empty_directories(&dir_path, &mut empty_directories);

                let elapsed: std::time::Duration = now.elapsed();
                println!("\nCompleted in {:.2?}", elapsed);

                let found_files_bundle: Vec<[String; 4]> = custodian::bundle_found_files(empty_directories);
                let csv_path: String = string_prompt("Enter the path of the CSV file to export search results to:");
                custodian::export_found_files_to_csv(csv_path.as_str(), found_files_bundle);

            },

            //Generate an archive of a directory
            7 => {

                let dir_path: String = string_prompt("Enter the path of the directory to archive:");
                let archive_path: String = string_prompt("Enter the path of the archive directory to generate:");

                let create_archive_result: Result<(), io::Error> = fs::create_dir(&archive_path);
                match create_archive_result {

                    Ok(_) => {

                        let file_separator: String = string_prompt("Enter the file separator character of your OS (\"/\" for Unix, \"\\\" for Windows):");
                        let cutoff_date: NaiveDate = date_prompt("Enter the cutoff date, formatted as YYYY-mm-dd (ex. 2020-01-01)");

                        let now: Instant = Instant::now();

                        custodian::generate_archive(&dir_path, &archive_path, &file_separator, &cutoff_date);

                        let elapsed: std::time::Duration = now.elapsed();
                        println!("\nCompleted in {:.2?}", elapsed);

                    },

                    Err(err) => {
                        println!("Could not create directory at path: {}", archive_path);
                        println!("{}", err);
                    },

                }

            },

            //Quit
            0 => {
                break;
            },

            //user did not select a valid option
            _ => {/*DO NOTHING*/},

        }

    }

}

fn menu() -> u8 {

    println!();
    println!("1. Search a directory for duplicate files BY NAME");
    println!("2. Search a directory for duplicate files BY CONTENTS");
    println!("3. Search a directory for files of a GIVEN NAME");
    println!("4. Search a directory for files of a GIVEN TYPE");
    println!("5. Search a directory for files last modified before a GIVEN CUTOFF DATE");
    println!("6. Search a directory for empty directories (folders)");
    println!("7. Generate an archive of a directory");
    println!("0. Quit");
    println!();

    let mut selection: String = String::new();
    let result: Result<usize, io::Error> = io::stdin().read_line(&mut selection);
    match result {

        Ok(_) => {

            //WITHOUT trim() FUNCTION, SELECTION INCLUDES \n AND ERRORS EVERY TIME
            selection = String::from(selection.trim());
            match selection.parse::<u8>() {

                Ok(num_selection) => num_selection,

                Err(_) => {
                    println!("User input \"{}\" cannot be parsed into i8!", selection);
                    //recursively call function
                    menu()
                },

            }

        },

        Err(_) => {
            println!("Invalid user input!");
            //recursively call function
            menu()
        },

    }

}

fn string_prompt(prompt: &str) -> String {

    println!();
    println!("{}", prompt);

    let mut selection: String = String::new();
    let result: Result<usize, io::Error> = io::stdin().read_line(&mut selection);
    match result {

        Ok(_) => {
            //WITHOUT trim() FUNCTION, SELECTION INCLUDES \n AND ERRORS EVERY TIME
            return String::from(selection.trim());
        },

        Err(_) => {
            println!("Invalid user input!");
            //recursively call function
            return string_prompt(prompt);
        },

    }

}

fn strings_prompt(prompt: &str) -> Vec<String> {

    println!();
    println!("{}", prompt);

    let mut selection: String = String::new();
    let result: Result<usize, io::Error> = io::stdin().read_line(&mut selection);
    match result {

        Ok(_) => {

            let mut strings: Vec<String> = vec![];

            let response: String = String::from(selection.trim());
            for part in response.split(" ") {
                strings.push(String::from(part));
            }

            return strings;

        },

        Err(_) => {
            println!("Invalid user input!");
            //recursively call function
            return strings_prompt(prompt);
        },

    }

}

fn date_prompt(prompt: &str) -> NaiveDate {

    println!();
    println!("{}", prompt);

    let mut selection: String = String::new();
    let result: Result<usize, io::Error> = io::stdin().read_line(&mut selection);
    match result {

        Ok(_) => {

            //WITHOUT trim() FUNCTION, SELECTION INCLUDES \n AND ERRORS EVERY TIME
            selection = String::from(selection.trim());
            
            let date_result: Result<NaiveDate, chrono::ParseError> = NaiveDate::parse_from_str(selection.as_str(), "%Y-%m-%d");
            match date_result {

                Ok(date) => {
                    return date;
                },

                Err(_) => {
                    println!("Could not parse user input String into NaiveDate");
                    return date_prompt(prompt);
                },

            }
            
        },

        Err(_) => {
            println!("Invalid user input!");
            //recursively call function
            return date_prompt(prompt);
        },

    }

}