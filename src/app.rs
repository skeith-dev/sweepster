use std::collections::HashMap;
use std::fs::DirEntry;
use std::{fs, io};
use std::time::Instant;
use chrono::NaiveDate;
use terminal_fonts::to_block_string;

use crate::{custodian, prompts};


pub fn run() {
    
    println!("\n{}", to_block_string("SWEEPSTER"));
    println!("\nYour very own command-line file custodian\n");

    loop {

        let action: u8 = prompts::parse_prompt::<u8>("1. Search\n2. Sweep\n3. Store");
        let criteria: u8;
        let sub_criteria: u8;
        let target: String = prompts::string_prompt("Enter the path of the target directory:");
        
        match action {

            //Search or Sweep
            1..=2 => {

                criteria = prompts::parse_prompt::<u8>("1. For duplicates\n2. By criteria");

                match criteria {

                    1 => {

                        sub_criteria = prompts::parse_prompt::<u8>("1. By name\n2. By contents");
                        let mut duplicate_files: Vec<(DirEntry, String)> = vec![];

                        match sub_criteria {

                            1 => {

                                let now: Instant = Instant::now();
                                let mut file_cabinet: HashMap<String, Vec<DirEntry>> = set_up_file_cabinet(&target);

                                for value in file_cabinet.values_mut() {
                                    custodian::find_duplicates_by_name(value, &mut duplicate_files);
                                }

                                let elapsed: std::time::Duration = now.elapsed();
                                println!("\nCompleted in {:.2?}", elapsed);

                            },

                            2 => {

                                let print_flag: bool = prompts::parse_prompt::<bool>("Enable print flag (enter \"true\" or \"false\")? Enabling will print each file comparison as it occurs.");

                                let now: Instant = Instant::now();
                                let mut file_cabinet: HashMap<String, Vec<DirEntry>> = set_up_file_cabinet(&target);

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

                        //FIXME
                        match sub_criteria {

                            //By name
                            1 => {

                                let file_names: Vec<String> = prompts::strings_prompt("Enter the file names to search for, INCLUDING the file extension, separated by a single space:");

                                let now: Instant = Instant::now();

                                custodian::find_files_of_given_names(&target, &file_names, &mut files_of_criteria);

                                let elapsed: std::time::Duration = now.elapsed();
                                println!("\nCompleted in {:.2?}", elapsed);

                            },

                            //By type
                            2 => {

                                let file_types: Vec<String> = prompts::strings_prompt("Enter the file types to search for, NOT INCLUDING the \".\", separated by a single space:");

                                let now: Instant = Instant::now();

                                custodian::find_files_of_given_types(&target, &file_types, &mut files_of_criteria);

                                let elapsed: std::time::Duration = now.elapsed();
                                println!("\nCompleted in {:.2?}", elapsed);

                            },

                            //By last modified
                            3 => {

                                let cutoff_date: NaiveDate = prompts::date_prompt("Enter the cutoff date, formatted as YYYY-mm-dd (ex. 2020-01-01)");

                                let now: Instant = Instant::now();

                                custodian::find_files_last_modifed_before(&target, &cutoff_date, &mut files_of_criteria);

                                let elapsed: std::time::Duration = now.elapsed();
                                println!("\nCompleted in {:.2?}", elapsed);

                            },

                            //Empty directories
                            4 => {

                                let now: Instant = Instant::now();

                                custodian::find_empty_directories(&target, &mut files_of_criteria);

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

                        custodian::generate_storage(&target, &storage_path, &file_separator, &cutoff_date);

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

fn set_up_file_cabinet(dir_path: &str) -> HashMap<String, Vec<DirEntry>> {

    let mut extension_counts: HashMap<String, u32> = HashMap::new();
    custodian::count_files_by_type(dir_path, &mut extension_counts);

    let mut file_cabinet: HashMap<String, Vec<DirEntry>> = HashMap::with_capacity(extension_counts.len());
    for (key, value) in extension_counts.into_iter() {
        file_cabinet.insert(key, Vec::with_capacity(value as usize));
    }

    custodian::organize_files_by_type(dir_path, &mut file_cabinet);

    return file_cabinet;

}


/* ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** */
//Testing
/* ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** */


#[cfg(test)]
mod tests {

    use std::collections::HashMap;
    use std::ffi::OsString;
    use std::fs::DirEntry;
    use std::path::Path;
    
    use clap::builder::OsStr;

    use crate::{app::{self, set_up_file_cabinet},custodian};

    const TEST_FOLDER_PATH: &str = "test";

    const BATMAN_FILES_COUNT: u32 = 1;
    const CATWOMAN_FILES_COUNT: u32 = 2;
    const CSV_FILE_COUNT: u32 = 1;
    const DUPLICATE_FILES_BY_NAME_COUNT: usize = 2;
    const EMPTY_DIRECTORIES_COUNT: usize = 1;
    const PNG_FILE_COUNT: u32 = 2;

    
    #[test]
    fn find_duplicates_by_name_test() {

        let mut duplicate_files: Vec<(DirEntry, String)> = vec![];
        let mut file_cabinet: HashMap<String, Vec<DirEntry>> = app::set_up_file_cabinet(TEST_FOLDER_PATH);

        for value in file_cabinet.values_mut() {
            custodian::find_duplicates_by_name(value, &mut duplicate_files);
        }

        assert_eq!(duplicate_files.len(), DUPLICATE_FILES_BY_NAME_COUNT);
        for (duplicate_file, original_file_path) in duplicate_files {

            let mut original_file_name: OsString = OsString::with_capacity(duplicate_file.file_name().len());

            match Path::new(&original_file_path).file_name() {
                Some(ofn) => {
                    original_file_name.push(ofn);
                },
                None => { },
            }

            assert_eq!(duplicate_file.file_name(), original_file_name);

        }

    }

    #[test]
    fn find_duplicates_by_contents_test() {

        let mut duplicate_files: Vec<(DirEntry, String)> = vec![];
        let mut file_cabinet: HashMap<String, Vec<DirEntry>> = set_up_file_cabinet(TEST_FOLDER_PATH);

        for value in file_cabinet.values_mut() {
            custodian::find_duplicates_by_contents(value, &mut duplicate_files, false);
        }

        assert_ne!(duplicate_files.len(), 0);
        for (duplicate_file, original_file_path) in duplicate_files {
            assert!( custodian::compare_two_files_by_contents_given_osstrs(duplicate_file.path().as_os_str(), &OsString::from(original_file_path)) );
        }

    }

    #[test]
    fn find_files_of_given_names_test() {

        let mut files_of_criteria: Vec<DirEntry> = vec![];
        let file_names: Vec<String> = vec![String::from("batman.txt"), String::from("catwoman.txt")];

        custodian::find_files_of_given_names(TEST_FOLDER_PATH, &file_names, &mut files_of_criteria);

        assert_eq!(files_of_criteria.len(), (BATMAN_FILES_COUNT + CATWOMAN_FILES_COUNT) as usize);
        for file_of_criteria in files_of_criteria {
            assert!(file_of_criteria.file_name().as_os_str() == OsStr::from("batman.txt") || file_of_criteria.file_name().as_os_str() == OsStr::from("catwoman.txt"));
        }

    }

    #[test]
    fn find_files_of_given_types_test() {

        let mut files_of_criteria: Vec<DirEntry> = vec![];
        let file_types: Vec<String> = vec![String::from("png"), String::from("csv")];

        custodian::find_files_of_given_types(TEST_FOLDER_PATH, &file_types, &mut files_of_criteria);

        assert_eq!(files_of_criteria.len(), (CSV_FILE_COUNT + PNG_FILE_COUNT) as usize);
        for file_of_criteria in files_of_criteria {
            assert!(file_of_criteria.file_name().as_os_str() == OsStr::from("batman.png") || file_of_criteria.file_name().as_os_str() == OsStr::from("robin.png") || file_of_criteria.file_name().as_os_str() == OsStr::from("vehicles.csv"));
        }

    }

    #[test]
    fn find_empty_directories_test() {

        let mut files_of_criteria: Vec<DirEntry> = vec![];

        custodian::find_empty_directories(TEST_FOLDER_PATH, &mut files_of_criteria);

        assert_eq!(files_of_criteria.len(), EMPTY_DIRECTORIES_COUNT);
        for file_of_criteria in files_of_criteria {
            assert!(file_of_criteria.file_name().as_os_str() == OsStr::from("better_superheroes"));
        }

    }

}
