use csv;
use std::collections::HashMap;
use std::fs::{self, DirEntry};
use std::ffi::OsStr;
use std::path::Path;


/* ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** */
//"GET FROM" functions
/* ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** */

fn file_path_from_dir_entry(dir_entry: &DirEntry) -> &str {

    //create & assign dir_entry_buff to PathBuf from path()
    let dir_entry_buff: std::path::PathBuf = dir_entry.path();

    //create & assign file_path_option variable to Option<&str> from to_str()
    let file_path_option: Option<&str> = dir_entry_buff.to_str();
    //match Option<&str> file_path_option
    match file_path_option {

        //if Some...
        Some(file_path) => {
            return file_path;
        },

        //if None...
        None => {
            println!("Could not parse DirEntry into file path &str");
            return "";
        },

    }

}

fn file_name_from_dir_entry(dir_entry: &DirEntry) -> &str {

    //create & assign dir_entry_buff to OsString from file_name()
    let dir_entry_os_string: std::ffi::OsString = dir_entry.file_name();

    //create & assign file_name_option variable to Option<&str> from to_str()
    let file_name_option: Option<&str> = dir_entry_os_string.to_str();
    //match Option<&str> file_name_option
    match file_name_option {

        //if Some...
        Some(file_name) => {
            return file_name;
        },

        //if None...
        None => {
            println!("Could not parse DirEntry into file name &str");
            return "";
        },

    }

}

fn file_name_from_file_path(file_path: &str) -> &str {

    let name_option: Option<&str> = Path::new(file_path).file_name().and_then(OsStr::to_str);
    match name_option {
        Some(name) => name,
        None => "",
    }

}

fn file_extension_from_file_path(file_path: &str) -> &str {

    let extension_option: Option<&str> = Path::new(file_path).extension().and_then(OsStr::to_str);
    match extension_option {
        Some(extension) => extension,
        None => "",
    }

}

fn file_size_from_file_path(file_path: &str) -> u64 {

    let metadata_result = fs::metadata(file_path);
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
//ORGANIZING FUNCTIONS
/* ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** */

pub fn organize_files_by_type(dir_path: &str, file_types: &mut HashMap<&str, Vec<&str>>) {

    //create & assign read_dir_result variable to Result<> from read_dir()
    let read_dir_result: Result<fs::ReadDir, std::io::Error> = fs::read_dir(dir_path);
    
    //match Result<> read_dir_result
    match read_dir_result {

        //if Ok...
        Ok(read_dir) => {

            //create & assign paths variable to non-Err read_dir
            let paths: fs::ReadDir = read_dir;
            
            //iterate through every path in paths
            for path in paths {

                //match Result<> path
                match path {

                    //if Ok...
                    Ok(dir_entry) => {

                        println!("{}", dir_entry.path().display());

                        //if the entry in the directory is a directory...
                        if dir_entry.path().is_dir() {

                            //create & assign file_path variable to String from path_from_dir_entry
                            let file_path: &str = file_path_from_dir_entry(&dir_entry);
                            //recursively call function
                            organize_files_by_type(file_path, file_types);

                        //if the entry in the directory is a file...
                        } else {

                            //create & assign file_path variable to String from path_from_dir_entry()
                            let file_path: &str = file_path_from_dir_entry(&dir_entry);
                            //create & assign file_name variable to String from name_from_dir_entry()
                            let file_extension: &str = file_extension_from_file_path(file_path);

                            //if the file_types HashMap already contains the file extension...
                            if file_types.contains_key(file_extension) {

                                //create & assign extension_vec_option variable to Option<> from get_mut()
                                //get_mut() returns a mutable reference (wrapped in an Option) to the value at the key
                                let extension_vec_option: Option<&mut Vec<&str>> = file_types.get_mut(file_extension);
                                match extension_vec_option {
                                    //if Some...
                                    Some(extension_vec) => {
                                        //push file entry file type vector
                                        extension_vec.push(file_path);
                                    },
                                    //if None...
                                    None => {
                                        println!("Could not get HashMap key of extension: {}", file_extension);
                                    },
                                }

                            //if the file_types HashMap DOES NOT already contain the file extension...
                            } else {
                                file_types.insert(file_extension, vec!{file_path});
                            }

                        }
                        
                    },

                    //if Err...
                    Err(_) => {
                        println!("Could not open directory at path: {}", dir_path);
                    }

                }

            }

        },

        //if Err...
        Err(_) => {
            println!("Could not open directory at path: {}", dir_path);
        },

    }

}

/* ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** */
//CLEANING FUNCTIONS
/* ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** */

pub fn find_duplicates_by_name(same_type_files: Vec<&str>, duplicate_files: &mut Vec<(&str, &str)>) {

    let mut files_by_name: HashMap<&str, &str> = HashMap::new();

    for i in 0..same_type_files.len() {

        let file_name: &str = file_name_from_file_path(same_type_files[i]);

        let duplicate_file_result: Option<&str> = files_by_name.insert(file_name, same_type_files[i]);
        match duplicate_file_result {

            Some(duplicate_file) => {
                println!("Found duplicate file: {}", duplicate_file);
                duplicate_files.push((file_name, duplicate_file));
            },

            None => {},

        }

    }

}

pub fn find_duplicate_files_by_size(same_type_files: Vec<&str>, duplicate_files: &mut Vec<(&str, &str)>) {

    let mut files_by_name: HashMap<u64, &str> = HashMap::new();

    for i in 0..same_type_files.len() {

        let file_name = file_name_from_file_path(same_type_files[i]);
        let file_size = file_size_from_file_path(same_type_files[i]);

        let duplicate_file_result: Option<&str> = files_by_name.insert(file_size, same_type_files[i]);
        match duplicate_file_result {

            Some(duplicate_file) => {
                println!("Found duplicate file: {}", duplicate_file);
                duplicate_files.push((file_name, duplicate_file));
            },

            None => {},

        }

    }

}

/* ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** */
//ORGANIZING FUNCTIONS
/* ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** */

pub fn export_duplicates_to_csv(file_path: &str, duplicate_files: Vec<(&str, &str)>) {

    //create & assign duplicate_files_vec variable to Vec<[&str; 2]> from vec!{}
    //FOR WRITER; writer can write "results" in &str format
    let mut duplicate_files_vec: Vec<[&str; 2]> = vec!{["file name", "file path"]};

    //iterate through each duplicate file in duplicate_files
    for i in 0..duplicate_files.len() {
        //push duplicate file entry onto duplicate_files_vec
        duplicate_files_vec.push([duplicate_files[i].0, duplicate_files[i].1]);
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