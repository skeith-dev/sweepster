use csv;
use std::collections::HashMap;
use std::fs::{self, DirEntry};
use std::ffi::OsStr;
use std::path::Path;


/* ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** */
//"GET FROM" FUNCTIONS
/* ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** */

fn file_path_from_dir_entry(dir_entry: &DirEntry) -> String {

    let dir_entry_buff: std::path::PathBuf = dir_entry.path();

    let file_path_option: Option<&str> = dir_entry_buff.to_str();
    match file_path_option {

        Some(file_path) => {
            return String::from(file_path);
        },

        None => {
            println!("Could not parse DirEntry into file path &str");
            return String::from("");
        },

    }

}

fn file_name_from_dir_entry(dir_entry: &DirEntry) -> String {

    let dir_entry_os_string: std::ffi::OsString = dir_entry.file_name();

    let file_name_option: Option<&str> = dir_entry_os_string.to_str();
    match file_name_option {

        Some(file_name) => {
            return String::from(file_name);
        },

        None => {
            println!("Could not parse DirEntry into file name &str");
            return String::from("");
        },

    }

}

fn file_name_from_file_path(file_path: &str) -> String {

    let name_option: Option<&str> = Path::new(file_path).file_name().and_then(OsStr::to_str);
    match name_option {
        Some(name) => String::from(name),
        None => String::from(""),
    }

}

fn file_extension_from_file_path(file_path: &str) -> String {

    let extension_option: Option<&str> = Path::new(file_path).extension().and_then(OsStr::to_str);
    match extension_option {
        Some(extension) => String::from(extension),
        None => String::from(""),
    }

}

fn file_size_from_file_path(file_path: &str) -> u64 {

    let metadata_result: Result<fs::Metadata, std::io::Error> = fs::metadata(file_path);
    match metadata_result {

        Ok(metadata) => {
            return metadata.len();
        },

        Err(_) => {
            println!("Could not get metadata for file: {}", file_path);
        },

    }

    return std::u64::MIN;

}

/* ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** */
// COUNTING AND ORGANIZING FUNCTIONS
/* ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** */

pub fn count_files_by_type(dir_path: &str, extension_counts: &mut HashMap<String, u32>) {

    let read_dir_result: Result<fs::ReadDir, std::io::Error> = fs::read_dir(dir_path);
    match read_dir_result {

        Ok(read_dir) => {

            let paths: fs::ReadDir = read_dir;
            for path in paths {

                match path {

                    Ok(dir_entry) => {

                        let file_path: String = file_path_from_dir_entry(&dir_entry);

                        if dir_entry.path().is_dir() {
                            count_files_by_type(file_path.as_str(), extension_counts);
                        } else {

                            let file_extension: String = file_extension_from_file_path(file_path.as_str());
                            if extension_counts.contains_key(&file_extension) {

                                let extension_count_option: Option<&mut u32> = extension_counts.get_mut(&file_extension);
                                match extension_count_option {
                                    Some(extension_count) => {
                                        *extension_count += 1;
                                    },
                                    None => {
                                        println!("Could not get the count of extension {}", file_extension);
                                    },
                                }

                            } else {
                                extension_counts.insert(file_extension, 1);
                            }

                        }
                        
                    },

                    Err(_) => {
                        println!("Could not open directory at path: {}", dir_path);
                    }

                }

            }

        },

        Err(_) => {
            println!("Could not open directory at path: {}", dir_path);
        },

    }

}

pub fn organize_files_by_type(dir_path: &str, file_types: &mut HashMap<String, Vec<String>>) {

    let read_dir_result: Result<fs::ReadDir, std::io::Error> = fs::read_dir(dir_path);
    match read_dir_result {

        Ok(read_dir) => {

            let paths: fs::ReadDir = read_dir;
            for path in paths {

                match path {

                    Ok(dir_entry) => {

                        if dir_entry.path().is_dir() {

                            let file_path: String = file_path_from_dir_entry(&dir_entry);
                            organize_files_by_type(file_path.as_str(), file_types);

                        } else {

                            let file_path: String = file_path_from_dir_entry(&dir_entry);
                            let file_extension: String = file_extension_from_file_path(file_path.as_str());

                            if file_types.contains_key(file_extension.as_str()) {

                                let extension_vec_option: Option<&mut Vec<String>> = file_types.get_mut(file_extension.as_str());
                                match extension_vec_option {
                                    Some(extension_vec) => {
                                        extension_vec.push(file_path);
                                    },
                                    None => {
                                        println!("Could not get HashMap key of extension: {}", file_extension);
                                    },
                                }

                            } else {
                                file_types.insert(file_extension, vec!{file_path});
                            }

                        }
                        
                    },

                    Err(_) => {
                        println!("Could not open directory at path: {}", dir_path);
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

pub fn find_duplicates_by_name(same_type_files: &mut Vec<String>, duplicate_files: &mut Vec<(String, String)>) {

    let mut files_by_name: HashMap<String, String> = HashMap::new();

    while !same_type_files.is_empty() {

        let file_name: String = file_name_from_file_path(same_type_files.last().unwrap().as_str());

        let original_file_path_result: Option<String> = files_by_name.insert(file_name.clone(), same_type_files.pop().unwrap());
        match original_file_path_result {

            Some(original_file_path) => {

                let duplicate_file_path_option: Option<String> = files_by_name.remove(&file_name);
                match duplicate_file_path_option {

                    Some(duplicate_file_path) => {
                        duplicate_files.push((original_file_path, duplicate_file_path));
                    },

                    None => {
                        println!("Could not get path of duplicate file!");
                    },

                }

            },

            None => {},

        }

    }

}

pub fn find_duplicates_by_size(same_type_files: Vec<String>, duplicate_files: &mut Vec<(String, String)>) {

    let mut files_by_name: HashMap<u64, &str> = HashMap::new();

    for i in 0..same_type_files.len() {

        let file_size: u64 = file_size_from_file_path(same_type_files[i].as_str());

        let original_file_result: Option<&str> = files_by_name.insert(file_size, same_type_files[i].as_str());
        match original_file_result {

            Some(original_file) => {
                println!("Found duplicate file: {}", same_type_files[i]);
                duplicate_files.push((String::from(original_file), same_type_files[i].clone()));
            },

            None => {},

        }

    }

}

/* ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** */
//EXPORTING FUNCTIONS
/* ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** */

pub fn export_duplicates_to_csv(file_path: &str, duplicate_files: Vec<(String, String)>) {

    //create & assign duplicate_files_vec variable to Vec<[&str; 2]> from vec!{}
    //FOR WRITER; writer can write "results" in &str format
    let mut duplicate_files_vec: Vec<[&str; 2]> = vec!{["ORIGINAL FILE PATH", "DUPLICATE FILE PATH"]};

    //iterate through each duplicate file in duplicate_files
    for i in 0..duplicate_files.len() {
        //push duplicate file entry onto duplicate_files_vec
        duplicate_files_vec.push([duplicate_files[i].0.as_str(), duplicate_files[i].1.as_str()]);
    }

    //create & assign writer_result variable to Result<> from from_path()
    let writer_result: Result<csv::Writer<fs::File>, csv::Error> = csv::Writer::from_path(file_path);
    //match writer_result
    match writer_result {

        //if Ok...
        Ok(mut writer) => {

            //iterate through each duplicate file in duplicate_files_vec
            for i in 0..duplicate_files_vec.len() {

                //create & assign write_record_result variable to Result<> from write_record()
                let write_record_result: Result<(), csv::Error> = writer.write_record(duplicate_files_vec[i]);
                //match write_record_result
                match write_record_result {

                    //if Ok...
                    Ok(_) => {},

                    //if Err...
                    Err(_) => {
                        println!("Could not write \"{:?}\" to {}", duplicate_files_vec[i], file_path);
                    },

                }

            }
        },

        //if Err...
        Err(_) => {
            println!("Could not create CSV writer for filepath: {}", file_path);
        },

    }

}