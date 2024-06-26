use std::{collections::HashMap, env, fs::{self, DirEntry}, io, time::Instant};

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

        "search" | "sweep" => 'search_or_sweep: {

            let option: String;
            match cli.option {
                Some(o) => {
                    option = o;
                },
                None => {
                    println!("Provide an option to search by!");
                    break 'search_or_sweep;
                }
            }

            let criteria: String;
            match cli.criteria {
                Some(c) => {
                    criteria = c;
                },
                None => {
                    println!("Provide a criteria to search by!");
                    break 'search_or_sweep;
                }
            }

            match option.as_str() {

                "duplicates" => {

                    let mut duplicate_files: Vec<(DirEntry, String)> = vec![];

                    match criteria.as_str() {

                        "by_name" => {

                            match cli.include_extension {

                                true => {

                                    let now: Instant = Instant::now();

                                    let mut extension_counts: HashMap<String, u32> = HashMap::new();
                                    custodian::count_files_by_type(&cli.target, &mut extension_counts);
                                    println!("\n{:?}", extension_counts);

                                    let mut file_cabinet: HashMap<String, Vec<DirEntry>> = HashMap::with_capacity(extension_counts.len());
                                    for (key, value) in extension_counts.into_iter() {
                                        file_cabinet.insert(key, Vec::with_capacity(value as usize));
                                    }

                                    custodian::organize_files_by_type(&cli.target, &mut file_cabinet);

                                    for value in file_cabinet.values_mut() {
                                        custodian::find_duplicates_by_name_including_ext(value, &mut duplicate_files);
                                    }

                                    let elapsed: std::time::Duration = now.elapsed();
                                    println!("\nCompleted in {:.2?}", elapsed);

                                },

                                false => {

                                    let now: Instant = Instant::now();
                                    
                                    let mut file_names: HashMap<String, DirEntry> = HashMap::new();

                                    custodian::find_duplicates_by_name_excluding_ext(&cli.target, &mut file_names, &mut duplicate_files);

                                    let elapsed: std::time::Duration = now.elapsed();
                                    println!("\nCompleted in {:.2?}", elapsed);

                                },

                            }

                        },

                        "by_contents" => {

                            let now: Instant = Instant::now();

                            let mut extension_counts: HashMap<String, u32> = HashMap::new();
                            custodian::count_files_by_type(&cli.target, &mut extension_counts);
                            println!("\n{:?}", extension_counts);

                            let mut file_cabinet: HashMap<String, Vec<DirEntry>> = HashMap::with_capacity(extension_counts.len());
                            for (key, value) in extension_counts.into_iter() {
                                file_cabinet.insert(key, Vec::with_capacity(value as usize));
                            }

                            custodian::organize_files_by_type(&cli.target, &mut file_cabinet);

                            for value in file_cabinet.values_mut() {
                                custodian::find_duplicates_by_contents(value, &mut duplicate_files, cli.print);
                            }

                            let elapsed: std::time::Duration = now.elapsed();
                            println!("\nCompleted in {:.2?}", elapsed);

                        },

                        _ => {
                            println!("Provide a valid criteria!");
                            break 'search_or_sweep;
                        },

                    }

                    match cli.action.as_str() {

                        "search" => {

                            let csv_path: String;
                            match cli.csv_path {
                                Some(csv_p) => {
                                    csv_path = csv_p;
                                },
                                None => {
                                    println!("No CSV filepath provided");
                                    break 'search_or_sweep;
                                },
                            }

                            let duplicate_files_bundle: Vec<[String; 5]> = custodian::bundle_duplicate_files(duplicate_files);
                            custodian::export_duplicate_files_to_csv(csv_path.as_str(), duplicate_files_bundle);

                        },

                        "sweep" => {
                            custodian::delete_duplicate_files(&mut duplicate_files);
                        },

                        _ => {
                            println!("Provide a valid action!");
                            break 'search_or_sweep;
                        },

                    }

                },

                "by_criteria" => {

                    let mut files_of_criteria: Vec<DirEntry> = vec![];

                    match criteria.as_str() {

                        "by_name" => {

                            let file_names: Vec<String>;
                            match cli.file_names {
                                Some(fln) => {
                                    file_names = fln;
                                },
                                None => {
                                    println!("Provide file names to search for!");
                                    break 'search_or_sweep;
                                },
                            }

                            let now: Instant = Instant::now();

                            custodian::find_files_of_given_names(&cli.target, &file_names, &mut files_of_criteria, cli.include_extension);

                            let elapsed: std::time::Duration = now.elapsed();
                            println!("\nCompleted in {:.2?}", elapsed);

                        },

                        "by_type" => {

                            let file_extensions: Vec<String>;
                            match cli.file_extensions {
                                Some(fe) => {
                                    file_extensions = fe;
                                },
                                None => {
                                    println!("Provide file types to search for!");
                                    break 'search_or_sweep;
                                },
                            }

                            let now: Instant = Instant::now();

                            custodian::find_files_of_given_types(&cli.target, &file_extensions, &mut files_of_criteria);

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
                                            break 'search_or_sweep;
                                        },
                                    }
                                },
                                None => {
                                    println!("Provide a cutoff date!");
                                    break 'search_or_sweep;
                                },
                            }

                            let now: Instant = Instant::now();

                            custodian::find_files_last_modifed_before(&cli.target, &cutoff_date, &mut files_of_criteria);

                            let elapsed: std::time::Duration = now.elapsed();
                            println!("\nCompleted in {:.2?}", elapsed);

                        },

                        "empty_directories" => {

                            let now: Instant = Instant::now();

                            custodian::find_empty_directories(&cli.target, &mut files_of_criteria);

                            let elapsed: std::time::Duration = now.elapsed();
                            println!("\nCompleted in {:.2?}", elapsed);

                        },

                        _ => {
                            println!("Provide a valid criteria!");
                            break 'search_or_sweep;
                        },

                    }

                    match cli.action.as_str() {

                        "search" => {

                            let csv_path: String;
                            match cli.csv_path {
                                Some(csv_p) => {
                                    csv_path = csv_p;
                                },
                                None => {
                                    println!("No CSV filepath provided");
                                    break 'search_or_sweep;
                                },
                            }

                            let found_files_bundle: Vec<[String; 4]> = custodian::bundle_found_files(files_of_criteria);
                            custodian::export_found_files_to_csv(csv_path.as_str(), found_files_bundle);

                        },

                        "sweep" => {
                            custodian::delete_found_files(&mut files_of_criteria);
                        },

                        _ => {
                            println!("Provide a valid action!");
                            break 'search_or_sweep;
                        },

                    }

                },

                _ => {
                    println!("Provide a valid option!");
                    break 'search_or_sweep;
                },

            }

        },

        "store" => 'store: {

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

            let file_separator: String = match env::consts::OS.contains("windows") {
                true => String::from("\\"),
                false => String::from("/"),
            };

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

            let create_archive_result: Result<(), io::Error> = fs::create_dir(&storage_path);
            match create_archive_result {

                Ok(_) => {

                    let now: Instant = Instant::now();

                    custodian::generate_storage(&cli.target, &storage_path, &file_separator, &cutoff_date);

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

        _ => {
            println!("Provide a valid action!");
        },

    }
    
}


/* ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** */
//Testing
/* ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** */


#[cfg(test)]
mod tests {

    use assert_cmd::cargo::CommandCargoExt;
    use chrono::{Duration, Utc};
    use std::{env, fs::{self, DirEntry, File}, process::Command};

    use crate::custodian;

    const TEST_DIRECTORY_PATH: &str = "test";
    const JUSTICE_LEAGUE_TEST_DIRECTORY_PATH: &str = "justice_league_test";


    fn setup_justice_league(test_directory_path: &mut String, file_separator: &String) -> Result<(), Box<dyn std::error::Error>> {

        test_directory_path.push_str(&file_separator);
        test_directory_path.push_str("justice_league");
        fs::create_dir_all(test_directory_path.as_str())?;

        let mut superman_file_path: String = String::new();
        superman_file_path.push_str(&test_directory_path);
        superman_file_path.push_str(&file_separator);
        superman_file_path.push_str("superman.txt");
        File::create(superman_file_path.as_str())?;

        let mut wonder_woman_file_path: String = String::new();
        wonder_woman_file_path.push_str(&test_directory_path);
        wonder_woman_file_path.push_str(&file_separator);
        wonder_woman_file_path.push_str("wonder_woman.txt");
        File::create(wonder_woman_file_path.as_str())?;

        return Ok(());

    }
    
    #[test]
    fn invalid_action_failure() -> Result<(), Box<dyn std::error::Error>> {

        let mut cmd: Command = Command::cargo_bin("sweepster")?;
        cmd.arg("non_existent_action").arg(TEST_DIRECTORY_PATH).arg("-o").arg("by_criteria").arg("-c").arg("by_name").arg("-n").arg("file.txt");

        let output: std::process::Output = cmd.output()?;
        assert!(String::from_utf8_lossy(&output.stdout).contains("Provide a valid action!"));

        return Ok(());

    }
    
    #[test]
    fn invalid_target_failure() -> Result<(), Box<dyn std::error::Error>> {

        let mut cmd: Command =  Command::cargo_bin("sweepster")?;
        cmd.arg("search").arg("test/non_existent_directory").arg("-o").arg("by_criteria").arg("-c").arg("by_name").arg("-n").arg("file.txt");
        
        let output: std::process::Output =  cmd.output()?;
        assert!(String::from_utf8_lossy(&output.stdout).contains("Could not open directory at path"));

        return Ok(());

    }

    #[test]
    fn invalid_option_failure() -> Result<(), Box<dyn std::error::Error>> {

        let mut cmd: Command =  Command::cargo_bin("sweepster")?;
        cmd.arg("search").arg(TEST_DIRECTORY_PATH).arg("-o").arg("non_existent_option").arg("-c").arg("by_name").arg("-n").arg("file.txt");
        
        let output: std::process::Output =  cmd.output()?;
        assert!(String::from_utf8_lossy(&output.stdout).contains("Provide a valid option!"));

        return Ok(());

    }

    #[test]
    fn invalid_criteria_failure() -> Result<(), Box<dyn std::error::Error>> {

        let mut cmd: Command = Command::cargo_bin("sweepster")?;
        cmd.arg("search").arg(TEST_DIRECTORY_PATH).arg("-o").arg("by_criteria").arg("-c").arg("by_non_existent_criteria").arg("-n").arg("file.txt");

        let output: std::process::Output = cmd.output()?;
        assert!(String::from_utf8_lossy(&output.stdout).contains("Provide a valid criteria!"));

        return Ok(());

    }

    #[test]
    fn search_duplicates_by_name_including_extension_success() -> Result<(), Box<dyn std::error::Error>> {

        let mut cmd: Command = Command::cargo_bin("sweepster")?;
        cmd.arg("search").arg(TEST_DIRECTORY_PATH).arg("-o").arg("duplicates").arg("-c").arg("by_name").arg("-i");

        let output: std::process::Output = cmd.output()?;
        let std_output: String = String::from_utf8_lossy(&output.stdout).to_string();

        assert!(std_output.contains("catwoman.txt"));
        assert!(std_output.contains("red_hood.txt"));

        assert!(!std_output.contains("batman.txt"));
        assert!(!std_output.contains("batman.png"));
        assert!(!std_output.contains("robin.txt"));
        assert!(!std_output.contains("robin.png"));

        return Ok(());

    }

    #[test]
    fn search_duplicates_by_name_excluding_extension_success() -> Result<(), Box<dyn std::error::Error>> {

        let mut cmd: Command = Command::cargo_bin("sweepster")?;
        cmd.arg("search").arg(TEST_DIRECTORY_PATH).arg("-o").arg("duplicates").arg("-c").arg("by_name");

        let output: std::process::Output = cmd.output()?;
        let std_output: String = String::from_utf8_lossy(&output.stdout).to_string();

        assert!(std_output.contains("catwoman.txt"));
        assert!(std_output.contains("red_hood.txt"));
        assert!(std_output.contains("batman.txt"));
        assert!(std_output.contains("batman.png"));
        assert!(std_output.contains("robin.txt"));
        assert!(std_output.contains("robin.png"));

        return Ok(());

    }

    #[test]
    fn search_duplicates_by_contents_success() -> Result<(), Box<dyn std::error::Error>> {

        let mut cmd: Command = Command::cargo_bin("sweepster")?;
        cmd.arg("search").arg(TEST_DIRECTORY_PATH).arg("-o").arg("duplicates").arg("-c").arg("by_contents");

        let output: std::process::Output = cmd.output()?;
        let std_output: String = String::from_utf8_lossy(&output.stdout).to_string();

        assert!(std_output.contains("nightwing.txt"));
        assert!(std_output.contains("dick_grayson.txt"));
        assert!(std_output.contains("red_hood.txt"));
        assert!(std_output.contains("jason_todd.txt"));
        assert!(std_output.contains("catwoman.txt"));
        assert!(std_output.contains("selina_kyle.txt"));
        assert!(std_output.contains("robin.txt"));
        assert!(std_output.contains("tim_drake.txt"));

        return Ok(());

    }

    #[test]
    fn search_by_criteria_by_name_success() -> Result<(), Box<dyn std::error::Error>> {

        let mut cmd: Command = Command::cargo_bin("sweepster")?;
        cmd.arg("search").arg(TEST_DIRECTORY_PATH).arg("-o").arg("by_criteria").arg("-c").arg("by_name").arg("-n").arg("catwoman red_hood");

        let output: std::process::Output = cmd.output()?;
        let std_output: String = String::from_utf8_lossy(&output.stdout).to_string();

        assert!(std_output.contains("catwoman.txt"));
        assert!(std_output.contains("red_hood.txt"));
        assert!(std_output.contains("catwoman.txt"));
        assert!(std_output.contains("red_hood.txt"));

        return Ok(());

    }

    #[test]
    fn search_by_criteria_by_type_success() -> Result<(), Box<dyn std::error::Error>> {

        let mut cmd: Command = Command::cargo_bin("sweepster")?;
        cmd.arg("search").arg(TEST_DIRECTORY_PATH).arg("-o").arg("by_criteria").arg("-c").arg("by_type").arg("-e").arg("png csv");

        let output: std::process::Output = cmd.output()?;
        let std_output: String = String::from_utf8_lossy(&output.stdout).to_string();

        assert!(std_output.contains("vehicles.csv"));
        assert!(std_output.contains("batman.png"));
        assert!(std_output.contains("robin.png"));

        return Ok(());

    }

    #[test]
    fn search_by_criteria_empty_directories_success() -> Result<(), Box<dyn std::error::Error>> {

        let mut cmd: Command = Command::cargo_bin("sweepster")?;
        cmd.arg("search").arg(TEST_DIRECTORY_PATH).arg("-o").arg("by_criteria").arg("-c").arg("empty_directories");

        let output: std::process::Output = cmd.output()?;
        let std_output: String = String::from_utf8_lossy(&output.stdout).to_string();

        assert!(std_output.contains("better_superheroes"));

        return Ok(());

    }

    #[test]
    fn store_success() -> Result<(), Box<dyn std::error::Error>> {

        let file_separator: String = match env::consts::OS.contains("windows") {
            true => String::from("\\"),
            false => String::from("/"),
        };
        let tomorrow_date: chrono::prelude::DateTime<Utc> = Utc::now() + Duration::days(1);

        let mut test_directory_path: String = String::from(TEST_DIRECTORY_PATH);

        setup_justice_league(&mut test_directory_path, &file_separator)?;

        let mut cmd: Command = Command::cargo_bin("sweepster")?;
        cmd.arg("store").arg(test_directory_path.as_str()).arg("-s").arg(JUSTICE_LEAGUE_TEST_DIRECTORY_PATH).arg("-d").arg(tomorrow_date.format("%Y-%m-%d").to_string().as_str());
        cmd.output()?;

        let mut files_of_criteria: Vec<DirEntry> = vec![];
        custodian::find_files_of_given_names(JUSTICE_LEAGUE_TEST_DIRECTORY_PATH, &(vec![String::from("superman.txt"), String::from("wonder_woman.txt")]), &mut files_of_criteria, true);

        for file in files_of_criteria {
            assert!( custodian::file_name_including_extension_from_direntry(&file) == String::from("superman.txt") || custodian::file_name_including_extension_from_direntry(&file) == String::from("wonder_woman.txt") )
        }

        fs::remove_dir_all(test_directory_path)?;
        fs::remove_dir_all(JUSTICE_LEAGUE_TEST_DIRECTORY_PATH)?;

        return Ok(());

    }

}
