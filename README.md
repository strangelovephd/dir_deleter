** Current Status:** Not Working

# Program Flow:

### Arguments:
0. Program file name.
1. flag for mode (--d -directory, --f -file)
2. Starting directory name.
3. Target directory/file to delete.

### Example:
   * dir_deleter --d ~/dev/ ./target/
   * dir_deleter --f ~/other_directory ./file.temp

### Logic:
    
    Determine mode based on flag (set mode for files or directories)

    Verify starting directory exists.
        - Support relative path?
        - If invalid, panic!("Starting directory does not exist")

    If Directory mode:
        - Iterate through child directories and search for the target.
            - If found, push target to a Vec<PathBuf>.
            - Continue iterating.
        - Print out the Vec<PathBuf> for final confirmation.
            - Individual confirmation?
            - Mode for Individual and Total Confirmation?  Default Individual.
        - Print what is being deleted.

    If File mode:
        - Iterate through child directories and search for target.
            - If found, push target to a Vec<PathBuf>.
            - Continue iterating.
        - etc.

### Ideas:
 - Flag to store deleted file/directory names in a .dir_deleter file.
 - Don't actually delete the files, move them to a temp directory and after the temp directory gets to a certain size, delete the oldest files.
