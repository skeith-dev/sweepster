# sweepster <img src="sweepster.png" alt="sweepster" width="160" height="160" align="right" title="sweepster logo">
### Your very own command-line file custodian! Find duplicate files, and other helpful tools.

## Description
sweepster is a file custodian; a helpful tool which keeps your computer tidy and free of clutter! Written in Rust,
sweepster has performance in mind when it executes one of several different tasks, outlined below:

### Search a directory for duplicate files BY NAME
Walks through a specified directory and identifies all files which do not have unique names. This means that the
first-detected file of a given name is treated as the "original". All files thereafter are considered to
be duplicates.

### Search a directory for duplicate files BY CONTENTS
Walks through a specified directory and identifies all files which do not have unique contents. This means that the
first-detected file with particular contents is treated as the "original". All files thereafter are considered to
be duplicates.


### Search a directory for files of a GIVEN NAME
Walks through a specified directory and identifies all files of a given name. This file name SHOULD NOT include the
file extension.

### Search a directory for files of a GIVEN TYPE
Walks through a specified directory and identifies all files of a given type, or file extension. This extension type
SHOULD NOT include dot. For instance, pdf should be specified as "pdf", not ".pdf". Additionally, these extension
types ARE case-sensitive. For example, to find ALL pdf files in a directory, pdf should be specified as "pdf"
and "PDF".

### Search a directory for files last modified before a GIVEN CUTOFF DATE
Walks through a specified directory and identifies all files which have last been modified prior to a given
cutoff date. The cutoff date should be given in the format YYYY-mm-dd; for instance, 2021-01-01.

### Search a directory for empty directories (folders)
Walks through a specified directory and identifies all empty directories (folders).

### Generate an archive of a directory
Walks through a specified directory and creates an identically-structured archive at a given directory. This archive
will contain all the same folders as are in the "original" directory. Any files in the "original" directory which
were last modified prior to a given cutoff date are moved over to their identical location in the archive. For example,
a file named "test.txt" exists in a specified location "/this/is/an/example/test.txt". The file was last modified on
2021-12-01. The user enters the parameters to generate an archive of "/this/is/an", with a cutoff date of 2022-01-01.
After the custodian finishes executing this job, a new folder named "archive" (specified by the user) exists, and
contains the file "test.txt" at the location "... /archive/example/test.txt".
