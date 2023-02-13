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
                let dir_path: String = file_path_prompt();
                println!();
                custodian::walk_dir(dir_path.as_str());
            },

            //Search a directory for duplicate files BY NAME
            2 => {

                let dir_path: String = file_path_prompt();

                let mut file_names: HashMap<String, String> = HashMap::new();
                let mut duplicate_files: Vec<(String, String)> = vec!{};

                println!();
                custodian::find_duplicates_by_name(dir_path.as_str(), &mut file_names, &mut duplicate_files);

                let csv_path: String = file_path_prompt();
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

fn file_path_prompt() -> String {

    println!();
    println!("Enter the file path:");

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
            file_path_prompt()
        },

    }

}