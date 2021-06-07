use std::{
    fs::File,
    path::Path,
};

#[derive(Default)]
pub struct FileTaskProvider {
    file: Option<File>,
}

impl FileTaskProvider {
    pub fn new() -> FileTaskProvider {
        Default::default()
    }

    pub fn new_from_file(file: File) -> FileTaskProvider {
        FileTaskProvider {
            file: Some(file)
        }
    }

    pub fn new_from_path<T>(path: T) -> FileTaskProvider
        where T: AsRef<Path> {
        FileTaskProvider {
            file: File::open(path).ok()
        }
    }
}