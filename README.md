# sweepster
![sweepster logo](sweepster.png?raw=true "sweepster")
Your very own command-line file custodian! Find duplicate files, and other helpful tools.

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
SHOULD NOT include dot. For instance, pdf should be specified as "pdf", not ".pdf".
