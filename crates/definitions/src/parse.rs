use byteorder::{LittleEndian, ReadBytesExt};
use ini::{Ini, Properties};

pub use std::{fs::File, io::BufReader};

use crate::error::Error;

/// VB6 Data types
pub type Endian = LittleEndian;
pub type Long = u32;
pub type Integer = u16;
pub type Byte = u8;

impl<R: std::io::Read + ?Sized> ArgentumReadExt for R {}
pub trait ArgentumReadExt: std::io::Read {
    fn read_long(&mut self) -> u32 {
        self.read_u32::<Endian>().unwrap()
    }

    fn read_integer(&mut self) -> u16 {
        self.read_u16::<Endian>().unwrap()
    }

    fn read_bool(&mut self) -> bool {
        self.read_u8().unwrap() == 0
    }

    fn read_byte(&mut self) -> u8 {
        self.read_u8().unwrap()
    }

    fn read_string(&mut self) -> String {
        let lenght = self.read_integer();
        let mut vec: Vec<u8> = vec![];
        for _ in 0..lenght {
            vec.push(self.read_u8().unwrap());
        }

        String::from_utf8_lossy(&vec).into_owned()
    }

    fn skip_header(&mut self) {
        let mut header_string = [0; 255];
        self.read_exact(&mut header_string).unwrap();
        self.read_long();
        self.read_long();
    }

    fn skip_temp_ints(&mut self, amount: usize) {
        for _ in 0..amount {
            self.read_integer();
        }
    }
}

pub fn get_binary_reader(file_path: &str) -> Result<BufReader<File>, Error> {
    let file = File::open(file_path);
    match file {
        Ok(file) => {
            let buffer = BufReader::new(file);
            Ok(buffer)
        }
        Err(e) => {
            println!("error reading file {}: {}", file_path, e);
            Err(Error::FileNotFound)
        }
    }
}

// Ini format utils

pub fn get_ini_reader(file: &str) -> Result<Ini, Error> {
    Ini::load_from_file(file).map_err(|_| Error::FileNotFound)
}

pub trait ArgentumIniReadExt {
    fn get_count(&self, key: &str) -> usize;
}

pub trait ArgentumIniPropertyReadExt {
    fn get_number(&self, key: &str) -> usize;
}

impl ArgentumIniReadExt for Ini {
    fn get_count(&self, key: &str) -> usize {
        let init = self
            .section(Some("INIT"))
            .expect("No INIT section! for {key}");

        init.get(key)
            .expect("No {key}!")
            .parse::<usize>()
            .expect("{key} is not a number")
    }
}

impl ArgentumIniPropertyReadExt for Properties {
    fn get_number(&self, key: &str) -> usize {
        self.get(key).unwrap_or("0").parse().unwrap_or(0)
    }
}
