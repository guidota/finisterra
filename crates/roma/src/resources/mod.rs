use std::{fs::File, io::Read, path::Path};

pub mod camera;
pub mod text_brush;
pub mod texture;

pub struct FileReaderError {
    _msg: String,
}

pub fn open_file(path: &Path) -> Result<File, FileReaderError> {
    match File::open(path) {
        Ok(file) => Ok(file),
        Err(e) => Err(FileReaderError {
            _msg: e.to_string(),
        }),
    }
}

pub fn read_file(path: &str) -> Result<Vec<u8>, FileReaderError> {
    let path = Path::new(path);
    let mut file = open_file(path)?;
    let mut buffer = Vec::new();
    let read_result = file.read_to_end(&mut buffer);
    match read_result {
        Ok(_) => Ok(buffer),
        Err(e) => Err(FileReaderError {
            _msg: e.to_string(),
        }),
    }
}
