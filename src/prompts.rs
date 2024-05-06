use std::io;
use chrono::NaiveDate;


pub fn string_prompt(prompt: &str) -> String {

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

pub fn strings_prompt(prompt: &str) -> Vec<String> {

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

pub fn parse_prompt<T: std::str::FromStr>(message: &str) -> T {

    println!();
    println!("{}", message);

    let mut selection: String = String::new();
    let result: Result<usize, io::Error> = io::stdin().read_line(&mut selection);
    match result {

        Ok(_) => {
            
            //WITHOUT trim() FUNCTION, SELECTION INCLUDES \n AND ERRORS EVERY TIME
            selection = String::from(selection.trim());
            match selection.parse::<T>() {

                Ok(num_selection) => num_selection,

                Err(_) => {
                    println!("User input \"{}\" cannot be parsed into bool!", selection);
                    //recursively call function
                    return parse_prompt::<T>(message);
                },

            }

        },

        Err(_) => {
            println!("Invalid user input!");
            //recursively call function
            return parse_prompt::<T>(message);
        },

    }

}

pub fn date_prompt(prompt: &str) -> NaiveDate {

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
