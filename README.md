# sweepster <img src="sweepster.png" alt="sweepster" width="160" height="160" align="right" title="sweepster logo">

### Your very own command-line file custodian! Find duplicate files, and other helpful tools.

## Description

sweepster is a file custodian; a helpful tool which keeps your computer tidy and free of clutter! Written in Rust,
sweepster has performance in mind when it executes one of several different tasks, outlined below:

## Setup

_TODO_

## Usage

Sweepster is a command-line tool run like any other, such as "vi" or "nc". After correct setup, simply run "sweepster" in your command line, followed by flags.

_FIXME_

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

Executing this command will **search** in the folder at **target** example/directory for (**option**) duplicate files based on the **criteria** of their file contents, **print**ing each comparison as it occurs
