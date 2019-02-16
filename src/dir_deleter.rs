use std::path::PathBuf;
use std::fs;
use std::str::FromStr;

pub enum ProgramFlags {
    DirectoryMode,
    FileMode,
    SingleMode,
    Grouped,   
}

pub struct DirDeleter {
    flags: Vec<ProgramFlags>,
    starting_dir: PathBuf,
    target: PathBuf,    
}

impl DirDeleter {
    pub fn new(arguments: Vec<String>) -> DirDeleter {
        let mut flags = Vec::new();

        DirDeleter::parse_flags(&mut flags, &arguments[1]);

        let start = PathBuf::from_str(arguments[2].as_str()).unwrap();
        let tar = PathBuf::from_str(arguments[3].as_str()).unwrap();

        DirDeleter {
            flags: flags,
            starting_dir: start,
            target: tar,
        }
    }

    fn parse_flags(flags_vec: &mut Vec<ProgramFlags>, flags: &String) {

        unimplemented!();
    }
}