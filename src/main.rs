use std::io;
use std::collections::HashMap;
use terminal_fonts::to_block_string;

mod custodian;


fn main() {
    
    //introductory prompt
    println!("\n{}", to_block_string("SWEEPSTER"));
    println!("\nYour very own command-line file custodian\n");

    loop {

        let selection: u8 = menu();

        match selection {

            //Search a directory for duplicate files BY NAME
            1 => {

                let dir_path: String = string_prompt("Enter the path of the directory to search BY NAME:");

                let mut file_types: HashMap<String, Vec<String>> = HashMap::new();
                custodian::organize_files_by_type(dir_path.as_str(), &mut file_types);

                let mut duplicate_files: Vec<(String, String)> = vec!{};
                for (_key, value) in file_types {
                    custodian::find_duplicates_by_name(value, &mut duplicate_files);
                }

                duplicate_files.sort();
                
                let csv_path: String = string_prompt("Enter the path of the CSV file to export search results to:");
                custodian::export_duplicates_to_csv(csv_path.as_str(), duplicate_files);
                
            },

            //Search a directory for duplicate files BY SIZE
            2 => {

                let dir_path: String = string_prompt("Enter the path of the directory to search BY NAME:");

                let mut file_types: HashMap<String, Vec<String>> = HashMap::new();
                custodian::organize_files_by_type(dir_path.as_str(), &mut file_types);

                let mut duplicate_files: Vec<(String, String)> = vec!{};
                for (_key, value) in file_types {
                    custodian::find_duplicates_by_size(value, &mut duplicate_files);
                }

                duplicate_files.sort();

                let csv_path: String = string_prompt("Enter the path of the CSV file to export search results to:");
                custodian::export_duplicates_to_csv(csv_path.as_str(), duplicate_files);

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
    println!("2. Search a directory for duplicate files BY SIZE");
    println!("0. Quit");
    println!();

    //create & assign selection variable to new String
    let mut selection: String = String::new();
    //create & assign result variable to Result<usize> from read_line() function
    let result = io::stdin().read_line(&mut selection);

    //match Result<usize> result
    match result {

        //if Ok...
        Ok(_r1) => {

            //re-assign selection variable to String from selection.trim()
            //re-assignment of selection frees memory of old selection
            //WITHOUT trim() FUNCTION, SELECTION INCLUDES \n AND ERRORS EVERY TIME
            selection = String::from(selection.trim());

            //match Result<usize> from parse::<i8>() function
            match selection.parse::<u8>() {

                //if Ok...
                //return parsed i8
                Ok(r2) => r2,

                //if Err...
                Err(_e2) => {
                    println!("User input \"{}\" cannot be parsed into i8!", selection);
                    //recursively call function
                    menu()
                },

            }

        },

        //if Err...
        Err(_e1) => {
            println!("Invalid user input!");
            //recursively call function
            menu()
        },

    }

}

fn string_prompt(prompt: &str) -> String {

    println!();
    println!("{}", prompt);

    //create & assign selection variable to new String
    let mut selection: String = String::new();
    //create & assign result variable to Result<usize> from read_line() function
    let result: Result<usize, io::Error> = io::stdin().read_line(&mut selection);

    //match Result<usize> result
    match result {

        //if Ok...
        Ok(_r1) => {
            //re-assign selection variable to String from selection.trim()
            //re-assignment of selection frees memory of old selection
            //WITHOUT trim() FUNCTION, SELECTION INCLUDES \n AND ERRORS EVERY TIME
            return String::from(selection.trim());
        },

        //if Err...
        Err(_e1) => {
            println!("Invalid user input!");
            //recursively call function
            return string_prompt(prompt);
        },

    }

}