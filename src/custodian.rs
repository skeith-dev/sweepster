use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::{self, DirEntry, File};
use std::io::{BufReader, Read};
use chrono::{NaiveDate, Utc, DateTime};


/* ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** */
//"GET FROM" FUNCTIONS
/* ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** */

fn file_path_from_direntry(dir_entry: &DirEntry) -> String {

    let path_buff: std::path::PathBuf = dir_entry.path();

    let path_option: Option<&str> = path_buff.to_str();
    match path_option {

        Some(file_path) => {
            return String::from(file_path);
        },

        None => {
            println!("Could not parse DirEntry into file path String");
            return String::from("");
        },

    }

}

pub fn file_name_including_extension_from_direntry(dir_entry: &DirEntry) -> String {

    let name_os_string: std::ffi::OsString = dir_entry.file_name();

    let name_option: Option<&str> = name_os_string.to_str();
    match name_option {

        Some(file_name) => {
            return String::from(file_name);
        },

        None => {
            println!("Could not parse DirEntry into file name String");
            return String::from("");
        },

    }

}

pub fn file_name_excluding_extension_from_direntry(dir_entry: &DirEntry) -> String {

    let path_buff: std::path::PathBuf = dir_entry.path();

    let path_option: Option<&OsStr> = path_buff.file_stem();
    match path_option {

        Some(name_os_string) => {

            let name_option: Option<&str> = name_os_string.to_str();
            match name_option {

                Some(file_name) => {
                    return String::from(file_name);
                },

                None => {
                    println!("Could not parse DirEntry into file name String");
                    return String::from("");
                },

            }

        },

        None => {
            println!("Could not get file_stem from PathBuf");
            return String::from("");
        },

    }

}

fn file_extension_from_direntry(dir_entry: &DirEntry) -> String {

    let path_buff: std::path::PathBuf = dir_entry.path();

    let extension_os_str_option: Option<&OsStr> = path_buff.extension();
    match extension_os_str_option {

        Some(extension_os_str) => {

            let extension_option: Option<&str> = extension_os_str.to_str();
            match extension_option {
                Some(extension) => String::from(extension),
                None => String::from(""),
            }

        },

        None => {
            return String::from("");
        },

    }

}

fn file_size_from_direntry(dir_entry: &DirEntry) -> u64 {

    let meta_data_result: Result<fs::Metadata, std::io::Error> = dir_entry.metadata();
    match meta_data_result {

        Ok(meta_data) => {
            return meta_data.len();
        },

        Err(_) => {
            println!("Could not get metadata of DirEntry");
            return 0;
        },

    }

}

fn file_last_modified_from_direntry(dir_entry: &DirEntry) -> NaiveDate {

    let meta_data_result: Result<fs::Metadata, std::io::Error> = dir_entry.metadata();
    match meta_data_result {
        
        Ok(meta_data) => {

            let last_modified_result: Result<std::time::SystemTime, std::io::Error> = meta_data.modified();
            match last_modified_result {

                Ok(last_modified) => {
                    let last_modified_date: DateTime<Utc> = last_modified.into();
                    return last_modified_date.date_naive();
                },

                Err(_) => {
                    println!("Could not get last modified SystemTime of DirEntry");
                    return NaiveDate::MIN;
                },

            }

        },

        Err(_) => {
            println!("Could not get metadata of DirEntry");
            return NaiveDate::MIN;
        },

    }

}

/* ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** */
// COUNTING AND ORGANIZING FUNCTIONS
/* ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** */

pub fn count_files_by_type(dir_path: &str, extension_counts: &mut HashMap<String, u32>) {

    let directory_result: Result<fs::ReadDir, std::io::Error> = fs::read_dir(dir_path);
    match directory_result {

        Ok(directory) => {

            for path in directory {

                match path {

                    Ok(entry) => {

                        let file_path: String = file_path_from_direntry(&entry);

                        if entry.path().is_dir() {
                            count_files_by_type(&file_path, extension_counts);
                        } else {

                            let file_extension: String = file_extension_from_direntry(&entry);
                            if extension_counts.contains_key(&file_extension) {

                                let extension_count_option: Option<&mut u32> = extension_counts.get_mut(&file_extension);
                                match extension_count_option {
                                    Some(extension_count) => {
                                        *extension_count += 1;
                                    },
                                    None => {
                                        println!("Could not get count of extension {}", file_extension);
                                    },
                                }

                            } else {
                                extension_counts.insert(file_extension, 1);
                            }

                        }
                        
                    },

                    Err(_) => {
                        println!("Could not open directory or file in directory: {}", dir_path);
                    }

                }

            }

        },

        Err(_) => {
            println!("Could not open directory at path: {}", dir_path);
        },

    }

}

pub fn organize_files_by_type(dir_path: &str, file_cabinet: &mut HashMap<String, Vec<DirEntry>>) {

    let directory_result: Result<fs::ReadDir, std::io::Error> = fs::read_dir(dir_path);
    match directory_result {

        Ok(directory) => {

            for path in directory {

                match path {

                    Ok(entry) => {

                        if entry.path().is_dir() {

                            let directory_path: String = file_path_from_direntry(&entry);
                            organize_files_by_type(directory_path.as_str(), file_cabinet);

                        } else {

                            let file_extension: String = file_extension_from_direntry(&entry);

                            if file_cabinet.contains_key(&file_extension) {

                                let extension_vec_option: Option<&mut Vec<DirEntry>> = file_cabinet.get_mut(file_extension.as_str());
                                match extension_vec_option {

                                    Some(extension_vec) => {
                                        extension_vec.push(entry);
                                    },

                                    None => {
                                        println!("Could not get HashMap key of extension: {}", file_extension);
                                    },

                                }

                            } else {
                                file_cabinet.insert(file_extension, vec!{entry});
                            }

                        }
                        
                    },

                    Err(_) => {
                        println!("Could not open directory or file in directory: {}", dir_path);
                    }

                }

            }

        },

        Err(_) => {
            println!("Could not open directory at path: {}", dir_path);
        },

    }

}

/* ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** */
//CLEANING FUNCTIONS
/* ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** */

pub fn find_duplicates_by_name_including_ext(drawer: &mut Vec<DirEntry>, duplicate_files: &mut Vec<(DirEntry, String)>) {

    let mut files_by_name: HashMap<String, DirEntry> = HashMap::new();

    while !drawer.is_empty() {

        let file_option: Option<DirEntry> = drawer.pop();
        match file_option {

            Some(file) => {

                let file_name: String = file_name_including_extension_from_direntry(&file);
                let file_path: String = file_path_from_direntry(&file);

                let duplicate_file_result: Option<DirEntry> = files_by_name.insert(file_name, file);
                match duplicate_file_result {

                    Some(duplicate_file) => {
                        println!("{} == {}", duplicate_file.path().display(), file_path);
                        duplicate_files.push( (duplicate_file, file_path) );
                    },

                    None => {},

                }

            },

            None => println!("Could not pop file from file cabinet drawer"),

        }

    }

}

pub fn find_duplicates_by_name_excluding_ext(dir_path: &str, file_names: &mut HashMap<String, DirEntry>, duplicate_files: &mut Vec<(DirEntry, String)>) {
    
    let directory_result: Result<fs::ReadDir, std::io::Error> = fs::read_dir(dir_path);
    match directory_result {

        Ok(directory) => {

            for path in directory {

                match path {

                    Ok(entry) => {

                        if entry.path().is_dir() {

                            let directory_path: String = file_path_from_direntry(&entry);
                            find_duplicates_by_name_excluding_ext(directory_path.as_str(), file_names, duplicate_files);

                        } else {

                            let file_name: String = file_name_excluding_extension_from_direntry(&entry);
                            let original_file_path: String = file_path_from_direntry(&entry);

                            match file_names.insert(file_name, entry) {

                                Some(duplicate_file) => {
                                    println!("{} == {}", duplicate_file.path().display(), original_file_path);
                                    duplicate_files.push( (duplicate_file, original_file_path) );
                                },

                                None => { /* entry was successfully inserted into file_names */ },

                            }

                        }
                        
                    },

                    Err(_) => {
                        println!("Could not open directory or file in directory: {}", dir_path);
                    }

                }

            }

        },

        Err(_) => {
            println!("Could not open directory at path: {}", dir_path);
        },

    }

}

pub fn find_duplicates_by_contents(drawer: &mut Vec<DirEntry>, duplicate_files: &mut Vec<(DirEntry, String)>, print_flag: bool) {

    let mut files_by_size: HashMap<u64, Vec<DirEntry>> = HashMap::new();

    while !drawer.is_empty() {

        let file_option: Option<DirEntry> = drawer.pop();
        match file_option {

            Some(file) => {

                let file_size: u64 = file_size_from_direntry(&file);

                if files_by_size.contains_key(&file_size) {

                    let files_of_size_option: Option<&mut Vec<DirEntry>> = files_by_size.get_mut(&file_size);
                    match files_of_size_option {
                        Some(files_of_size) => {
                            files_of_size.push(file);
                        },
                        None => {
                            println!("Could not get files of size {}", &file_size);
                        },
                    }

                    
                } else {
                    files_by_size.insert(file_size, vec!{file});
                }

            },

            None => println!("Could not pop file from file cabinet drawer"),

        }

    }

    println!("{:?}", files_by_size);

    for value in files_by_size.values_mut() {

        for i in 0..value.len() {

            if i >= value.len() {
                break;
            }

            let mut j: usize = i + 1;
            while j < value.len() {

                if compare_two_files_by_contents_given_direntries(&value[i], &value[j], print_flag) {

                    let duplicate_file: DirEntry = value.remove(j);
                    duplicate_files.push( (duplicate_file, file_path_from_direntry(&value[i])) );
                    
                } else {
                    j += 1;
                }

            }

        }

    }

}

fn compare_two_files_by_contents_given_direntries(dir_entry_1: &DirEntry, dir_entry_2: &DirEntry, print_flag: bool) -> bool {

    let file_1_path: String = file_path_from_direntry(&dir_entry_1);
    let file_2_path: String = file_path_from_direntry(&dir_entry_2);

    if print_flag {
        println!("{} <-> {}", file_1_path, file_2_path);
    }

    let file_1_result: Result<File, std::io::Error> = File::open(&file_1_path);
    let file_2_result: Result<File, std::io::Error> = File::open(&file_2_path);

    match file_1_result {

        Ok(file_1) => {

            match file_2_result {

                Ok(file_2) => {

                    let file_1_reader: BufReader<File> = BufReader::new(file_1);
                    let file_2_reader: BufReader<File> = BufReader::new(file_2);

                    for (file_1_byte_result, file_2_byte_result) in file_1_reader.bytes().zip(file_2_reader.bytes()) {

                        match file_1_byte_result {

                            Ok(file_1_byte) => {

                                match file_2_byte_result {

                                    Ok(file_2_byte) => {

                                        if file_1_byte != file_2_byte {
                                            return false;
                                        }

                                    },

                                    Err(_) => {
                                        println!("Could not read byte from file at path: {}", file_2_path);
                                        return false;
                                    },

                                }

                            },

                            Err(_) => {
                                println!("Could not read byte from file at path: {}", file_1_path);
                                return false;
                            },

                        }

                    }

                    println!("{} == {}", file_1_path, file_2_path);
                    return true;

                },

                Err(_) => {
                    println!("Could not open file at path: {}", file_2_path);
                    return false;
                },

            }

        },

        Err(_) => {
            println!("Could not open file at path: {}", file_1_path);
            return false;
        },

    }

}

#[allow(dead_code)]
pub fn compare_two_files_by_contents_given_osstrs(osstr_1: &OsStr, osstr_2: &OsStr) -> bool {

    let file_1_result: Result<File, std::io::Error> = File::open(osstr_1);
    let file_2_result: Result<File, std::io::Error> = File::open(osstr_2);

    match file_1_result {

        Ok(file_1) => {

            match file_2_result {

                Ok(file_2) => {

                    let file_1_reader: BufReader<File> = BufReader::new(file_1);
                    let file_2_reader: BufReader<File> = BufReader::new(file_2);

                    for (file_1_byte_result, file_2_byte_result) in file_1_reader.bytes().zip(file_2_reader.bytes()) {

                        match file_1_byte_result {

                            Ok(file_1_byte) => {

                                match file_2_byte_result {

                                    Ok(file_2_byte) => {

                                        if file_1_byte != file_2_byte {
                                            return false;
                                        }

                                    },

                                    Err(_) => {
                                        println!("Could not read byte from file at path: {:?}", osstr_2);
                                        return false;
                                    },

                                }

                            },

                            Err(_) => {
                                println!("Could not read byte from file at path: {:?}", osstr_1);
                                return false;
                            },

                        }

                    }

                    println!("{:?} == {:?}", osstr_1, osstr_2);
                    return true;

                },

                Err(_) => {
                    println!("Could not open file at path: {:?}", osstr_2);
                    return false;
                },

            }

        },

        Err(_) => {
            println!("Could not open file at path: {:?}", osstr_1);
            return false;
        },

    }

}

pub fn delete_duplicate_files(duplicate_files: &mut Vec<(DirEntry, String)>) {

    while !duplicate_files.is_empty() {

        match duplicate_files.pop() {

            Some((duplicate_file, _)) => {

                match fs::remove_file(duplicate_file.path()) {

                    Ok(_) => {
                        println!("✔ -> {}", duplicate_file.path().display());
                    },
    
                    Err(err) => {
                        println!("✘ -> {}", duplicate_file.path().display());
                        println!("{}", err);
                    },
    
                } 

            },

            None => {
                println!("Cannot pop from duplicate_files!");
            },

        }

        

    }

}

pub fn delete_found_files(files_of_criteria: &mut Vec<DirEntry>) {

    while !files_of_criteria.is_empty() {

        match files_of_criteria.pop() {

            Some(file_of_criteria) => {

                match fs::remove_file(file_of_criteria.path()) {

                    Ok(_) => {
                        println!("✔ -> {}", file_of_criteria.path().display());
                    },
    
                    Err(err) => {
                        println!("✘ -> {}", file_of_criteria.path().display());
                        println!("{}", err);
                    },
    
                } 

            },

            None => {
                println!("Cannot pop from duplicate_files!");
            },

        }

        

    }

}

pub fn find_files_of_given_names(dir_path: &str, file_names: &Vec<String>, files_of_names: &mut Vec<DirEntry>, include_extension: bool) {

    let directory_result: Result<fs::ReadDir, std::io::Error> = fs::read_dir(dir_path);
    match directory_result {

        Ok(directory) => {

            for path in directory {

                match path {

                    Ok(entry) => {

                        if entry.path().is_dir() {

                            let directory_path: String = file_path_from_direntry(&entry);
                            find_files_of_given_names(directory_path.as_str(), file_names, files_of_names, include_extension);

                        } else {

                            let file_name: String = match include_extension {
                                true => file_name_including_extension_from_direntry(&entry),
                                false => file_name_excluding_extension_from_direntry(&entry),
                            };

                            if file_names.iter().any(|e| file_name == *e) {

                                let file_path: String = file_path_from_direntry(&entry);
                                println!("{}", file_path);

                                files_of_names.push(entry);

                            }

                        }
                        
                    },

                    Err(_) => {
                        println!("Could not open directory or file in directory: {}", dir_path);
                    },

                }

            }

        },

        Err(_) => {
            println!("Could not open directory at path: {}", dir_path);
        },

    }

}

pub fn find_files_of_given_types(dir_path: &str, file_types: &Vec<String>, files_of_types: &mut Vec<DirEntry>) {

    let directory_result: Result<fs::ReadDir, std::io::Error> = fs::read_dir(dir_path);
    match directory_result {

        Ok(directory) => {

            for path in directory {

                match path {

                    Ok(entry) => {

                        if entry.path().is_dir() {

                            let directory_path: String = file_path_from_direntry(&entry);
                            find_files_of_given_types(directory_path.as_str(), file_types, files_of_types);

                        } else {

                            let file_type: String = file_extension_from_direntry(&entry);
                            if file_types.iter().any(|e| file_type == *e) {

                                let file_path: String = file_path_from_direntry(&entry);
                                println!("{}", file_path);

                                files_of_types.push(entry);

                            }

                        }
                        
                    },

                    Err(_) => {
                        println!("Could not open directory or file in directory: {}", dir_path);
                    },

                }

            }

        },

        Err(_) => {
            println!("Could not open directory at path: {}", dir_path);
        },

    }

}

pub fn find_files_last_modifed_before(dir_path: &str, cutoff_date: &NaiveDate, files_last_modified_before: &mut Vec<DirEntry>) {

    let directory_result: Result<fs::ReadDir, std::io::Error> = fs::read_dir(dir_path);
    match directory_result {

        Ok(directory) => {

            for path in directory {

                match path {

                    Ok(entry) => {

                        if entry.path().is_dir() {

                            let directory_path: String = file_path_from_direntry(&entry);
                            find_files_last_modifed_before(directory_path.as_str(), cutoff_date, files_last_modified_before);

                        } else {

                            let last_modified: NaiveDate = file_last_modified_from_direntry(&entry);
                            if last_modified < *cutoff_date {

                                let file_path: String = file_path_from_direntry(&entry);
                                println!("{}", file_path);

                                files_last_modified_before.push(entry);

                            }

                        }
                        
                    },

                    Err(_) => {
                        println!("Could not open directory or file in directory: {}", dir_path);
                    }

                }

            }

        },

        Err(_) => {
            println!("Could not open directory at path: {}", dir_path);
        },

    }

}

pub fn find_empty_directories(dir_path: &str, empty_directories: &mut Vec<DirEntry>) {

    let directory_result: Result<fs::ReadDir, std::io::Error> = fs::read_dir(dir_path);
    match directory_result {

        Ok(directory) => {

            for path in directory {

                match path {

                    Ok(entry) => {

                        if entry.path().is_dir() {

                            let entry_directory_path: String = file_path_from_direntry(&entry);

                            let entry_directory_result: Result<fs::ReadDir, std::io::Error> = entry.path().read_dir();
                            match entry_directory_result {

                                Ok(entry_directory) => {

                                    if entry_directory.count() == 0 {
                                        println!("{}", entry_directory_path);
                                        empty_directories.push(entry);
                                    } else {
                                        find_empty_directories(entry_directory_path.as_str(), empty_directories);
                                    }

                                },

                                Err(_) => {
                                    println!("Could not open directory at path: {}", entry_directory_path);
                                }

                            }

                        }
                        
                    },

                    Err(_) => {
                        println!("Could not open directory or file in directory: {}", dir_path);
                    },

                }

            }

        },

        Err(_) => {
            println!("Could not open directory at path: {}", dir_path);
        },

    }

}

pub fn generate_storage(target: &str, storage_path: &str, file_separator: &str, cutoff_date: &NaiveDate) {

    let directory_result: Result<fs::ReadDir, std::io::Error> = fs::read_dir(target);
    match directory_result {

        Ok(directory) => {

            for path in directory {

                match path {

                    Ok(entry) => {

                        if entry.path().is_dir() {

                            let entry_dir_name: String = file_name_including_extension_from_direntry(&entry);
                            let entry_dir_path: String = file_path_from_direntry(&entry);

                            let mut new_dir_path: String = String::from(storage_path);
                            new_dir_path.push_str(file_separator);
                            new_dir_path.push_str(&entry_dir_name);

                            let create_dir_result: Result<(), std::io::Error> = fs::create_dir(&new_dir_path);
                            match create_dir_result {

                                Ok(_) => {
                                    generate_storage(&entry_dir_path, &new_dir_path, file_separator, cutoff_date);
                                },

                                Err(err) => {
                                    println!("Could not create directory at path: {}", new_dir_path);
                                    println!("{}", err);
                                },

                            }

                        } else {

                            let last_modified: NaiveDate = file_last_modified_from_direntry(&entry);
                            if last_modified < *cutoff_date {
                                
                                let entry_file_name: String = file_name_including_extension_from_direntry(&entry);
                                let entry_file_path: String = file_path_from_direntry(&entry);

                                let mut new_file_path: String = String::from(storage_path);
                                new_file_path.push_str(file_separator);
                                new_file_path.push_str(&entry_file_name);

                                let rename_result: Result<(), std::io::Error> = fs::rename(&entry_file_path, &new_file_path);
                                match rename_result {

                                    Ok(_) => {},

                                    Err(err) => {
                                        println!("Could not rename (move) file at path \"{}\" to file at path \"{}\"", entry_file_path, new_file_path);
                                        println!("{}", err);
                                    },

                                }

                            }

                        }
                        
                    },

                    Err(_) => {
                        println!("Could not open directory or file in directory: {}", target);
                    },

                }

            }

        },

        Err(_) => {
            println!("Could not open directory at path: {}", target);
        },

    }

}

/* ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** */
//EXPORTING FUNCTIONS
/* ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** */

pub fn bundle_found_files(found_files: Vec<DirEntry>) -> Vec<[String; 4]> {

    //FOR WRITER; writer can write "results" in &str format
    let mut duplicate_files_bundle: Vec<[String; 4]> = Vec::with_capacity(found_files.len());

    found_files.into_iter().for_each(|file| {

        let file_name: String = file_name_including_extension_from_direntry(&file);
        let file_path: String = file_path_from_direntry(&file);
        let file_extension: String = file_extension_from_direntry(&file);
        let file_size: String = file_size_from_direntry(&file).to_string();
        
        duplicate_files_bundle.push( [file_name, file_path, file_extension, file_size] );

    });

    duplicate_files_bundle.sort();
    duplicate_files_bundle.insert(0, [String::from("FILE NAME"), String::from("FILE PATH"), String::from("FILE TYPE"), String::from("FILE SIZE")]);

    return duplicate_files_bundle;

}

pub fn bundle_duplicate_files(found_files: Vec<(DirEntry, String)>) -> Vec<[String; 5]> {

    //FOR WRITER; writer can write "results" in &str format
    let mut duplicate_files_bundle: Vec<[String; 5]> = Vec::with_capacity(found_files.len());

    found_files.into_iter().for_each( | (duplicate_file, original_file_path) | {

        let file_name: String = file_name_including_extension_from_direntry(&duplicate_file);
        let file_path: String = file_path_from_direntry(&duplicate_file);
        let file_extension: String = file_extension_from_direntry(&duplicate_file);
        let file_size: String = file_size_from_direntry(&duplicate_file).to_string();
        
        duplicate_files_bundle.push( [file_name, file_path, original_file_path, file_extension, file_size] );

    });

    duplicate_files_bundle.sort();
    duplicate_files_bundle.insert(0, [String::from("FILE NAME"), String::from("FILE PATH"), String::from("ORIGINAL FILE PATH"), String::from("FILE TYPE"), String::from("FILE SIZE")]);

    return duplicate_files_bundle;

}

pub fn export_found_files_to_csv(file_path: &str, found_files_bundle: Vec<[String; 4]>) {

    let writer_result: Result<csv::Writer<fs::File>, csv::Error> = csv::Writer::from_path(file_path);
    match writer_result {

        Ok(mut writer) => {

            for i in 0..found_files_bundle.len() {

                let record: [&str; 4] = [found_files_bundle[i][0].as_str(), found_files_bundle[i][1].as_str(), found_files_bundle[i][2].as_str(), found_files_bundle[i][3].as_str()];

                let write_record_result: Result<(), csv::Error> = writer.write_record(record);
                match write_record_result {

                    Ok(_) => {},

                    Err(_) => {
                        println!("Could not write duplicate file entry to CSV");
                    },

                }

            }
        },

        Err(_) => {
            println!("Could not create CSV writer for filepath: {}", file_path);
        },

    }

}

pub fn export_duplicate_files_to_csv(file_path: &str, duplicate_files_bundle: Vec<[String; 5]>) {

    let writer_result: Result<csv::Writer<fs::File>, csv::Error> = csv::Writer::from_path(file_path);
    match writer_result {

        Ok(mut writer) => {

            for i in 0..duplicate_files_bundle.len() {

                let record: [&str; 5] = [duplicate_files_bundle[i][0].as_str(), duplicate_files_bundle[i][1].as_str(), duplicate_files_bundle[i][2].as_str(), duplicate_files_bundle[i][3].as_str(), duplicate_files_bundle[i][4].as_str()];

                let write_record_result: Result<(), csv::Error> = writer.write_record(record);
                match write_record_result {

                    Ok(_) => {},

                    Err(_) => {
                        println!("Could not write duplicate file entry to CSV");
                    },

                }

            }
        },

        Err(_) => {
            println!("Could not create CSV writer for filepath: {}", file_path);
        },

    }

}


/* ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** */
//Testing
/* ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** */


#[cfg(test)]
mod tests {

    use super::*;
    
    const TEST_DIRECTORY_PATH: &str = "test";
    const NUMBER_OF_FILE_TYPES: u32 = 3;

    const CSV_FILES_COUNT: u32 = 1;
    const PNG_FILES_COUNT: u32 = 2;
    const TEXT_FILES_COUNT: u32 = 11;


    #[test]
    fn test_count_files_by_type() {

        let mut extension_counts: HashMap<String, u32> = HashMap::new();
        let mut csv_count: u32 = 0;
        let mut png_count: u32 = 0;
        let mut txt_count: u32 = 0;

        count_files_by_type(TEST_DIRECTORY_PATH, &mut extension_counts);

        print!("{:?}", extension_counts);

        match extension_counts.get("csv") {
            Some(cc) => {
                csv_count = *cc;
            },
            None => { },
        }

        match extension_counts.get("png") {
            Some(pc) => {
                png_count = *pc;
            },
            None => { },
        }
        
        match extension_counts.get("txt") {
            Some(tc) => {
                txt_count = *tc;
            },
            None => { },
        }

        assert_eq!(extension_counts.len(), NUMBER_OF_FILE_TYPES as usize);
        assert_eq!(csv_count, CSV_FILES_COUNT);
        assert_eq!(png_count, PNG_FILES_COUNT);
        assert_eq!(txt_count, TEXT_FILES_COUNT);

    }

    #[test]
    fn test_organize_files_by_type() {

        let mut extension_counts: HashMap<String, u32> = HashMap::new();
        count_files_by_type(TEST_DIRECTORY_PATH, &mut extension_counts);

        let mut file_cabinet: HashMap<String, Vec<DirEntry>> = HashMap::with_capacity(extension_counts.len());
        for (key, value) in extension_counts.into_iter() {
            file_cabinet.insert(key, Vec::with_capacity(value as usize));
        }

        organize_files_by_type(TEST_DIRECTORY_PATH, &mut file_cabinet);

        for (file_extension, files_of_type) in file_cabinet.iter() {
            for file in files_of_type {
                assert_eq!(&file_extension_from_direntry(file), file_extension);
            }
        }

    }

}
