use std::io;
use std::collections::HashMap;
use terminal_fonts::to_block_string;

mod custodian;


fn main() {
    
    //introductory prompt
    println!("\n{}", to_block_string("SWEEPSTER"));
    println!("\nYour very own command-line file custodian\n\n");

    loop {

        let selection: u8 = menu();

        match selection {

            //"Walk" a directory and list it's contents
            1 => {
                let dir_path: String = string_prompt("Enter the path of the directory to walk:");
                println!();
                custodian::walk_dir(dir_path.as_str());
            },

            //Search a directory for duplicate files BY NAME
            2 => {

                //create & assign dir_path variable as String from string_prompt()
                let dir_path: String = string_prompt("Enter the path of the directory to search:");

                //create & assign file_names variable as HashMap<String, String> from HashMap::new()
                let mut file_names: HashMap<String, String> = HashMap::new();
                //create & assign duplicate_files variable as Vec<(String, String)> from vec!{}
                let mut duplicate_files: Vec<(String, String)> = vec!{};

                println!();
                //find NAME duplicates in directory at file path dir_path.as_str()
                //place discovered files in file_names HashMap<key, value>
                //place duplicate files in duplicate_files Vec<(file name, file path)>
                custodian::find_duplicates_by_name(dir_path.as_str(), &mut file_names, &mut duplicate_files);

                println!();
                println!("HashMap currently has {} entries", file_names.len());

                //sort the duplicate files alphabetically prior to CSV export
                duplicate_files.sort();

                //create & assign csv_path variable as String from string_prompt()
                let csv_path: String = string_prompt("Enter the path of the CSV file to export search results to:");
                //export duplicate files in duplicate_files found from search to CSV file at path csv_path.as_str()
                custodian::export_duplicates_to_csv(csv_path.as_str(), duplicate_files);

            },

            3 => {

                //create & assign dir_path variable as String from string_prompt()
                let dir_path: String = string_prompt("Enter the path of the directory to search:");

                //create & assign file_types variable as Vec<Vec<String>> from vec!{}
                let mut file_types: Vec<Vec<String>> = vec!{};
                //create & assign duplicate_files variable as Vec<(String, String)> from vec!{}
                let mut duplicate_files: Vec<(String, String)> = vec!{};

                println!();
                custodian::organize_files_by_type(&dir_path, &mut file_types);
                custodian::find_duplicate_files_by_size(&mut file_types, & mut duplicate_files);
                
                //sort the duplicate files alphabetically prior to CSV export
                duplicate_files.sort();

                //create & assign csv_path variable as String from string_prompt()
                let csv_path: String = string_prompt("Enter the path of the CSV file to export search results to:");
                //export duplicate files in duplicate_files found from search to CSV file at path csv_path.as_str()
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
    println!("1. \"Walk\" a directory and list it's contents");
    println!("2. Search a directory for duplicate files BY NAME");
    println!("3. Search a directory for duplicate files BY FILE SIZE");
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
            String::from(selection.trim())
        },

        //if Err...
        Err(_e1) => {
            println!("Invalid user input!");
            //recursively call function
            string_prompt(prompt)
        },

    }

}