use csv;
use std::collections::HashMap;
use std::fs::{self, DirEntry};
use std::ffi::OsStr;
use std::path::Path;


pub fn walk_dir(dir_path: &str) {

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
                            let file_path: String = path_from_dir_entry(&dir_entry);
                            //recursively call function
                            walk_dir(file_path.as_str());

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

pub fn find_duplicates_by_name(dir_path: &str, file_names: &mut HashMap<String, String>, duplicate_files: &mut Vec<(String, String)>) {

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

                        //if the entry in the directory is a directory...
                        if dir_entry.path().is_dir() {

                            //create & assign file_path variable to String from path_from_dir_entry
                            let file_path: String = path_from_dir_entry(&dir_entry);
                            //recursively call function
                            find_duplicates_by_name(file_path.as_str(), file_names, duplicate_files);

                        //if the entry in the directory is a file...
                        } else {

                            //create & assign file_name variable to String from name_from_dir_entry()
                            let file_name: String = name_from_dir_entry(&dir_entry);
                            //create & assign file_path variable to String from path_from_dir_entry()
                            let file_path: String = path_from_dir_entry(&dir_entry);

                            //create & assign duplicate_file_option variable to Option<String> from insert()
                            //insert() returns Some(existing value) at key IF it exists,
                            //else returns None
                            let duplicate_file_option: Option<String> = file_names.insert(file_name.clone(), file_path);
                            //match duplicate_file_option
                            match duplicate_file_option {

                                //if Some...
                                Some(duplicate_file_path) => {

                                    println!("Found duplicate file: {}", duplicate_file_path);
                                    //push new duplicate file entry onto duplicate_files Vec
                                    duplicate_files.push((file_name, duplicate_file_path));
                                },

                                //if None...
                                None => {},

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

pub fn organize_files_by_type(dir_path: &str, file_types: &mut Vec<Vec<String>>) {

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
                            let file_path: String = path_from_dir_entry(&dir_entry);
                            //recursively call function
                            organize_files_by_type(file_path.as_str(), file_types);

                        //if the entry in the directory is a file...
                        } else {

                            //create & assign file_path variable to String from path_from_dir_entry()
                            let file_path: String = path_from_dir_entry(&dir_entry);
                            //create & assign file_name variable to String from name_from_dir_entry()
                            let file_extension: String = file_extension_from_file_path(file_path.as_str());

                            //create & assign contains_extension variable to false
                            let mut contains_extension: bool = false;
                            //create & assign extension_index variable to 0
                            let mut extension_index: usize = 0;
                            //iterate through every file type in file_types
                            for i in 0..file_types.len() {
                                //if the file_extension already exists in file_types...
                                if file_types[i].contains(&file_extension) {
                                    //set contains_extension to true
                                    contains_extension = true;
                                    //set extension_index to i
                                    extension_index = i;
                                    break;
                                }
                            }

                            //if file_extension already exists in file_types...
                            if contains_extension {
                                //push file_path onto the Vec for that file type
                                file_types[extension_index].push(file_path);
                            //if file_extension DOES NOT already exist in file_types...
                            } else {
                                //push a new Vec onto file_types for the new extension type
                                file_types.push(vec!{file_extension});
                                //create & assign file_types_last_index variable as usize from
                                //the current length of file_types
                                let file_types_last_index: usize = file_types.len() - 1;
                                //push file_path onto the Vec for that file type
                                file_types[file_types_last_index].push(file_path);
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

pub fn find_duplicate_files_by_size(file_types: &mut Vec<Vec<String>>, duplicate_files: &mut Vec<(String, String)>) {

    for i in 0..file_types.len() {

        println!();
        println!("File type: {}", file_types[i][0]);
        println!("Number of entries: {}", file_types[i].len());

        if file_types[i].len() == 1 {
            continue;
        }

        for j in 1..file_types[i].len() {

            if j >= file_types[i].len() {
                break;
            }

            let file_1_metadata_result: Result<fs::Metadata, std::io::Error> = fs::metadata(file_types[i][j].as_str());
            match file_1_metadata_result {

                Ok(file_1_metadata) => {

                    for k in j + 1..file_types[i].len() {

                        if k >= file_types[i].len() {
                            break;
                        }

                        let file_2_metadata_result: Result<fs::Metadata, std::io::Error> = fs::metadata(file_types[i][k].as_str());
                        match file_2_metadata_result {

                            Ok(file_2_metadata) => {

                                if file_1_metadata.len() == file_2_metadata.len() {

                                    println!("Found duplicate file: {}", file_types[i][k]);

                                    let file_1_path: String = file_types[i][j].clone();
                                    let file_2_path: String = file_types[i].remove(k);
                                    duplicate_files.push((file_1_path, file_2_path));
                                    
                                }

                            },

                            Err(_) => {
                                println!("Could not get metadata of file at path: {}", file_types[i][k]);
                            }

                        }

                        

                    }

                },

                Err(_) => {
                    println!("Could not get metadata of file at path: {}", file_types[i][j]);
                },

            }

        }

    }

}

fn path_from_dir_entry(dir_entry: &DirEntry) -> String {

    //create & assign dir_entry_buff to PathBuf from path()
    let dir_entry_buff: std::path::PathBuf = dir_entry.path();

    //create & assign file_path_option variable to Option<&str> from to_str()
    let file_path_option: Option<&str> = dir_entry_buff.to_str();
    //match Option<&str> file_path_option
    match file_path_option {

        //if Some...
        Some(file_path) => {
            return String::from(file_path);
        },

        //if None...
        None => {
            println!("Could not parse DirEntry into file path &str");
            return String::from("");
        },

    }

}

fn name_from_dir_entry(dir_entry: &DirEntry) -> String {

    //create & assign dir_entry_buff to OsString from file_name()
    let dir_entry_os_string: std::ffi::OsString = dir_entry.file_name();

    //create & assign file_name_option variable to Option<&str> from to_str()
    let file_name_option: Option<&str> = dir_entry_os_string.to_str();
    //match Option<&str> file_name_option
    match file_name_option {

        //if Some...
        Some(file_name) => {
            return String::from(file_name);
        },

        //if None...
        None => {
            println!("Could not parse DirEntry into file name &str");
            return String::from("");
        },

    }

}

fn file_extension_from_file_path(file_path: &str) -> String {

    let extension_option: Option<&str> = Path::new(file_path).extension().and_then(OsStr::to_str);
    match extension_option {

        Some(extension) => String::from(extension),

        None => String::from(""),

    }

}

pub fn export_duplicates_to_csv(file_path: &str, duplicate_files: Vec<(String, String)>) {

    //create & assign duplicate_files_vec variable to Vec<[&str; 2]> from vec!{}
    //FOR WRITER; writer can write "results" in &str format
    let mut duplicate_files_vec: Vec<[&str; 2]> = vec!{["file name", "file path"]};

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