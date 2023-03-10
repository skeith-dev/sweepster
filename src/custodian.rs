use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::{self, DirEntry, File};
use std::io::{BufReader, Read};


/* ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** */
//"GET FROM" FUNCTIONS
/* ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** */

fn file_path_from_dir_entry(dir_entry: &DirEntry) -> String {

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

fn file_name_from_dir_entry(dir_entry: &DirEntry) -> String {

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

fn file_extension_from_dir_entry(dir_entry: &DirEntry) -> String {

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

fn file_size_from_dir_entry(dir_entry: &DirEntry) -> u64 {

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

/* ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** */
// COUNTING AND ORGANIZING FUNCTIONS
/* ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** */

pub fn count_files_by_type(dir_path: &str, extension_counts: &mut HashMap<String, u32>) {

    let read_dir_result: Result<fs::ReadDir, std::io::Error> = fs::read_dir(dir_path);
    match read_dir_result {

        Ok(read_dir) => {

            let read_dir: fs::ReadDir = read_dir;
            for path in read_dir {

                match path {

                    Ok(dir_entry) => {

                        let file_path: String = file_path_from_dir_entry(&dir_entry);

                        if dir_entry.path().is_dir() {

                            count_files_by_type(file_path.as_str(), extension_counts);

                        } else {

                            let file_extension: String = file_extension_from_dir_entry(&dir_entry);
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

    let read_dir_result: Result<fs::ReadDir, std::io::Error> = fs::read_dir(dir_path);
    match read_dir_result {

        Ok(read_dir) => {

            for path in read_dir {

                match path {

                    Ok(dir_entry) => {

                        if dir_entry.path().is_dir() {

                            let directory_path: String = file_path_from_dir_entry(&dir_entry);
                            organize_files_by_type(directory_path.as_str(), file_cabinet);

                        } else {

                            let file_extension: String = file_extension_from_dir_entry(&dir_entry);

                            if file_cabinet.contains_key(&file_extension) {

                                let extension_vec_option: Option<&mut Vec<DirEntry>> = file_cabinet.get_mut(file_extension.as_str());
                                match extension_vec_option {

                                    Some(extension_vec) => {
                                        extension_vec.push(dir_entry);
                                    },

                                    None => {
                                        println!("Could not get HashMap key of extension: {}", file_extension);
                                    },

                                }

                            } else {
                                file_cabinet.insert(file_extension, vec!{dir_entry});
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

pub fn find_duplicates_by_name(cabinet_drawer: &mut Vec<DirEntry>, duplicate_files: &mut Vec<DirEntry>) {

    let mut files_by_name: HashMap<String, DirEntry> = HashMap::new();

    while !cabinet_drawer.is_empty() {

        let file_option: Option<DirEntry> = cabinet_drawer.pop();
        match file_option {

            Some(file) => {

                let file_name: String = file_name_from_dir_entry(&file);

                if files_by_name.contains_key(&file_name) {
                    duplicate_files.push(file);
                } else {
                    files_by_name.insert(file_name, file);
                }

            },

            None => println!("Could not pop file from file cabinet drawer"),

        }

    }

}

pub fn find_duplicates_by_contents(cabinet_drawer: &mut Vec<DirEntry>, duplicate_files: &mut Vec<DirEntry>) {

    let mut files_by_size: HashMap<u64, Vec<DirEntry>> = HashMap::new();

    while !cabinet_drawer.is_empty() {

        let file_option: Option<DirEntry> = cabinet_drawer.pop();
        match file_option {

            Some(file) => {

                let file_size: u64 = file_size_from_dir_entry(&file);

                if files_by_size.contains_key(&file_size) {

                    let files_of_size_option: Option<&mut Vec<DirEntry>> = files_by_size.get_mut(&file_size);
                    match files_of_size_option {
                        Some(files_of_size) => {
                            files_of_size.push(file);
                        },
                        None => {
                            println!("Could not get the files of size {}", &file_size);
                        },
                    }

                    
                } else {
                    files_by_size.insert(file_size, vec!{file});
                }

            },

            None => println!("Could not pop file from file cabinet drawer"),

        }

    }

    for (key, value) in files_by_size.iter_mut() {

        if value.len() > 1 {
            println!("{} files of size {}", value.len(), key);
        }

        for i in 0..value.len() {

            for j in (i + 1)..value.len() {

                if i >= value.len() || j >= value.len() {
                    continue;
                }

                let are_equal: bool = compare_two_files_by_contents(&value[i], &value[j]);
                if are_equal {

                    let duplicate_file = value.remove(j);
                    duplicate_files.push(duplicate_file);
                    
                }

            }

        }

    }

}

fn compare_two_files_by_contents(dir_entry_1: &DirEntry, dir_entry_2: &DirEntry) -> bool {

    let file_1_path: String = file_path_from_dir_entry(&dir_entry_1);
    let file_2_path: String = file_path_from_dir_entry(&dir_entry_2);

    println!("{} <-> {}", file_1_path, file_2_path);

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

                    return true;

                },

                Err(_) => {
                    println!("Could not open file at path: {}", file_2_path);
                    return false;
                }

            }

        },

        Err(_) => {
            println!("Could not open file at path: {}", file_1_path);
            return false;
        },

    }

}

pub fn find_files_of_given_names(dir_path: &str, file_names: &Vec<String>, files_of_names: &mut Vec<DirEntry>) {

    let read_dir_result: Result<fs::ReadDir, std::io::Error> = fs::read_dir(dir_path);
    match read_dir_result {

        Ok(read_dir) => {

            for path in read_dir {

                match path {

                    Ok(dir_entry) => {

                        if dir_entry.path().is_dir() {

                            let directory_path: String = file_path_from_dir_entry(&dir_entry);
                            find_files_of_given_names(directory_path.as_str(), file_names, files_of_names);

                        } else {

                            let file_name: String = file_name_from_dir_entry(&dir_entry);

                            if file_names.iter().any(|e| file_name.contains(e)) {
                                files_of_names.push(dir_entry);
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
//EXPORTING FUNCTIONS
/* ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** ***** */

pub fn bundle_found_files(duplicate_files: Vec<DirEntry>) -> Vec<[String; 4]> {

    //FOR WRITER; writer can write "results" in &str format
    let mut duplicate_files_bundle: Vec<[String; 4]> = vec!{ [String::from("FILE NAME"), String::from("FILE PATH"), String::from("FILE TYPE"), String::from("FILE SIZE")] };

    duplicate_files.into_iter().for_each(|file| {

        let file_name: String = file_name_from_dir_entry(&file);
        let file_path: String = file_path_from_dir_entry(&file);
        let file_extension: String = file_extension_from_dir_entry(&file);
        let file_size: String = file_size_from_dir_entry(&file).to_string();
        
        duplicate_files_bundle.push( [file_name, file_path, file_extension, file_size] );

    });

    return duplicate_files_bundle;

}

pub fn export_found_files_to_csv(file_path: &str, duplicate_files_bundle: Vec<[String; 4]>) {

    let writer_result: Result<csv::Writer<fs::File>, csv::Error> = csv::Writer::from_path(file_path);
    match writer_result {

        Ok(mut writer) => {

            for i in 0..duplicate_files_bundle.len() {

                let record = [duplicate_files_bundle[i][0].as_str(), duplicate_files_bundle[i][1].as_str(), duplicate_files_bundle[i][2].as_str(), duplicate_files_bundle[i][3].as_str()];

                let write_record_result: Result<(), csv::Error> = writer.write_record(record);
                match write_record_result {

                    Ok(_) => {},

                    Err(_) => {
                        println!("Could not write duplicate file entry to CSV");
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