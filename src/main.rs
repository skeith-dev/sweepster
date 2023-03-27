use std::collections::HashMap;
use std::fs::DirEntry;
use std::io;
use std::time::Instant;
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
