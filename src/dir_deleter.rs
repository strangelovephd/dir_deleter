use std::path::PathBuf;
use std::fs;
use std::str::FromStr;
use std::iter;
use std::io::ErrorKind;

pub enum ProgramFlags {
    DirectoryMode,
    FileMode,
    SingleMode,
    GroupedMode,   
}

pub struct DirDeleter {
    flags: Option<Vec<ProgramFlags>>,
    starting_dir: PathBuf,
    target: PathBuf,    
}

impl DirDeleter {
    pub fn new(arguments: Vec<String>) -> DirDeleter {

        let flags = DirDeleter::parse_flags(&arguments[1]);

        let start = PathBuf::from_str(arguments[2].as_str()).unwrap();
        let tar = PathBuf::from_str(arguments[3].as_str()).unwrap();

        DirDeleter {
            flags: flags,
            starting_dir: start,
            target: tar,
        }
    }

    fn parse_flags(flags: &String) -> Option<Vec<ProgramFlags>> {
        if !(flags[..2].as_bytes() == "--".as_bytes()) { 
            return None; 
        }

        let mut collection: Vec<ProgramFlags> = Vec::new();

        for c in flags.as_bytes() {
            if *c == b'-' { continue; }
            match *c {
                b'd' => collection.push(ProgramFlags::DirectoryMode),
                b'f' => collection.push(ProgramFlags::FileMode),
                b's' => collection.push(ProgramFlags::SingleMode),
                b'g' => collection.push(ProgramFlags::GroupedMode),
                _ => {},
            }
        }
        Some(collection)
    }

    fn validate(&self) -> std::io::Result<()> {
        let attr = fs::metadata(self.starting_dir.as_path())?;
        if !attr.is_dir() {
            return Err(ErrorKind::NotFound);
        }

        Ok(())
    }

    pub fn run(&self) {
        self.validate();
        unimplemented!();
    }
}