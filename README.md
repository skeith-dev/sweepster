# sweepster `<img src="sweepster.png" alt="sweepster" width="160" height="160" align="right" title="sweepster logo">`

### Your very own command-line file custodian! Find duplicate files, and other helpful tools.

## Description

sweepster is a file custodian; a helpful tool which keeps your computer tidy and free of clutter! Written in Rust,
sweepster has performance in mind when it executes one of several different tasks, outlined below:

## Setup

_TODO_

## Usage

Sweepster is a command-line tool run like any other, such as "vi" or "nc". After correct setup, simply run "sweepster" in your command line, followed by flags.

- ACTION

  - -a --action
  - "search" - Self explanatory :]
  - "sweep" - Delete files
  - "store" - Create an "archive" of a specified directory. An archive is a new folder which has an identical folder structure as the original, and which contains all files from the original which were last modified before a given cutoff date.
  - "Sweepster" - Runs the application in a more user-friendly, menu-and-prompt-style interface.
- TARGET

  - -t --target
  - Specify a valid filepath to a target directory. A target directory is the directory upon which the action is being taken.
- OPTION

  - -o --option
  - "duplicates" - Identify files which are duplicates of one another based on a given criteria.
  - "by_criteria" - Identify files which match a given criteria.
- CRITERIA

  - -c --criteria
  - "by_name" - By file name
  - "by_contents" *duplicates only* - By the byte contents of the files
  - "by_type" *by_criteria only* - By file type (extension)
  - "by_last_modified" *by_criteria only* - Self explanatory :]
  - "empty_directories" *by_criteria only* - Self explanatory :]
- PRINT

  - -p --print *by_contents only*
  - "true" - Print every file comparison as it occurs; nice for checking progress
  - "false"- Don't print file comparisons; only print matches
- CSV_PATH *search only*

  - -v --csv-path
  - Specify a valid filepath to a csv file containing the results of the search
- INCLUDE_EXTENSION

  - -i --include-extension *by_name only*
  - "true" - Include the file extension (ex. ".txt", ".pdf", ".jpeg") as part of file names
  - "false" - Don't include the file extension as part of file names
- FILE_NAMES

  - -n --file-names
  - Specify file names to search for, separated by a single space
- FILE_TYPES

  - -e --file-types ("e" for extensions)
  - Specify file types (extensions) to search for, case-sensitive, separated by a single space
- CUTOFF_DATE

  - -d --cutoff-date
  - Specify a cutoff date to search for, or to create an archive from; format as YYYY-mm-dd
- STORAGE_PATH

  - -s --storage-path
  - Specify a valid filepath to a storage directory

Running "sweepster -h" lists each of the different flag options, shown below:

_UPDATEME_

```
Options:
  -a, --action <ACTION>                      
  -t, --target <TARGET>                      
  -o, --option <OPTION>                      
  -c, --criteria <CRITERIA>                  
  -p, --print <PRINT>                          [possible values: true, false]
  -v, --csv-path <CSV_PATH>                  
  -i, --include-extension <INCLUDE_EXTENSION>  [possible values: true, false]
  -n, --file-names <FILE_NAMES>              
  -e, --file-extensions <FILE_EXTENSIONS>    
  -d, --cutoff-date <CUTOFF_DATE>            
  -s, --storage-path <STORAGE_PATH>          
  -h, --help                                   Print help
  -V, --version                                Print version
```

### Examples

```
sweepster -a search -t example/directory -o duplicates -c by_contents -p true
```

Executing this command will **search** in the folder at **target** example/directory for (**option**) duplicate files based on the **criteria** of their file contents, **print**ing each comparison as it occurs
