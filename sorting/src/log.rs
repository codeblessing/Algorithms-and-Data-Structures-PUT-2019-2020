use std::cell::RefCell;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub struct Log {
    file: RefCell<Option<RefCell<File>>>,
}

impl Log {
    pub fn new() -> Log {
        Log {
            file: RefCell::new(None),
        }
    }

    pub fn open(&self, path: &Path, append: bool) {
        let file = OpenOptions::new()
            .write(true)
            .append(append)
            .create(true)
            .truncate(!append)
            .open(path);

        match file {
            Ok(result) => *self.file.borrow_mut() = Some(RefCell::new(result)),
            Err(err) => eprintln!("Error! {}", err),
        }
    }

    pub fn log(&self, msg: &str) {
        match &*self.file.borrow() {
            Some(file) => {
                let _ = file.borrow_mut().write(msg.as_bytes());
            }
            None => {
                eprintln!("Error! File doesn't exists!");
            }
        };
    }
}
