# sweepster <img src="sweepster.png" alt="sweepster" width="160" height="160" align="right" title="sweepster logo">

### Your very own command-line file custodian! Find duplicate files, and other helpful tools.

## Description

sweepster is a file custodian; a helpful tool which keeps your computer tidy and free of clutter! Written in Rust,
sweepster has performance in mind when it executes one of several different tasks, outlined below:

## Setup

To setup sweepster as an available command from the command line, move the executable file (sweepster [Unix], sweepster.exe [Windows]) to the appropriate platform-dependent directory which houses command-line executable files. For Unix, this is typically /bin or /usr/bin. For Windows, this is typically C:\WINDOWS\System32; however, any directory which is a PATH system variable will work. The PATH is the system variable that your operating system uses to locate needed executables from the command line or Terminal window.

## Usage

Sweepster is a command-line tool run like any other, such as "vi" or "nc". After correct setup, simply run "sweepster" in your command line with an ACTION and a TARGET, followed by flags.

### ARGUMENTS
- ACTION
  - -a --action
  - "search" - Self explanatory :]
  - "sweep" - Delete files
  - "store" - Create an "archive" of a specified directory. An archive is a new folder which has an identical folder structure as the original, and which contains all files from the original which were last modified before a given cutoff date
- TARGET
  - -t --target
  - Specify a valid filepath to a target directory. A target directory is the directory upon which the action is being taken.

### OPTIONS
- OPTION
  - -o --option
  - "duplicates" - Identify files which are duplicates of one another based on a given criteria.
  - "by_criteria" - Identify files which match a given criteria.
- CRITERIA
  - -c --criteria
  - "by_name" - By file name (extension included)
  - "by_contents" *duplicates only* - By the byte contents of the files
  - "by_type" *by_criteria only* - By file type (extension)
  - "by_last_modified" *by_criteria only* - Self explanatory :]
  - "empty_directories" *by_criteria only* - Self explanatory :]
- FILE_NAMES
  - -n --file-names
  - Specify file names to search for, separated by a single space
- FILE_EXTENSIONS
  - -e --file-types ("e" for extensions)
  - Specify file types (extensions) to search for, case-sensitive, separated by a single space
- CUTOFF_DATE
  - -d --cutoff-date
  - Specify a cutoff date to search for, or to create an archive from; format as YYYY-mm-dd
- INCLUDE_EXTENSION
  - -i --include-extension
  - Present - Include the file extension as part of the file name
  - Not present - Don't include the file extension as part of the file name
- PRINT
  - -p --print *by_contents only*
  - Present - Print every file comparison as it occurs; nice for checking progress
  - Not present - Don't print file comparisons; only print matches
- CSV_PATH
  - -v --csv-path
  - Specify a valid filepath to a csv file containing the results of the search
- STORAGE_PATH
  - -s --storage-path
  - Specify a valid filepath to a storage directory

Running "sweepster -h" lists each of the different flag options, shown below:
```
Usage: sweepster [OPTIONS] <ACTION> <TARGET>

Arguments:
  <ACTION>  
  <TARGET>  

Options:
  -o, --option <OPTION>                    
  -c, --criteria <CRITERIA>                
  -n, --file-names <FILE_NAMES>            
  -e, --file-extensions <FILE_EXTENSIONS>  
  -d, --cutoff-date <CUTOFF_DATE>          
  -i, --include-extension                  
  -p, --print                              
  -v, --csv-path <CSV_PATH>                
  -s, --storage-path <STORAGE_PATH>        
  -h, --help                               Print help
  -V, --version                            Print version
```

### Examples

```
sweepster search example/directory -o duplicates -c by_contents -p
```
Executing this command will **search** the folder at **target** example/directory for (**option**) duplicate files based on the **criteria** of their file contents, **print**ing each comparison as it occurs

```
sweepster sweep example/directory -o by_criteria -c by_name -n test testing
```
Executing this command will **sweep** the folder at **target** example/directory for (**option**) files which are (**criteria**) **name**d any of the following: test, testing (file extension [**include-extension**] NOT included)
