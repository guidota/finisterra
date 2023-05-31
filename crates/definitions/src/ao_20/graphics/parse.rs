use std::{
    collections::BTreeMap,
    io::{BufReader, Read},
};

use byteorder::ReadBytesExt;

use super::{Animation, Image};

use crate::parse::{get_binary_reader, ArgentumReadExt, Endian, Long};

pub trait GraphicsReadExt: std::io::Read {
    fn read_animation(&mut self, id: Long, frames_len: u16) -> Animation {
        Animation {
            frames: (0..frames_len)
                .map(|_| self.read_long().to_string())
                .collect::<Vec<_>>(),
            speed: self.read_long(),
            id: id.to_string(),
        }
    }

    fn read_image(&mut self, id: Long) -> Image {
        Image {
            file_num: self.read_long(),
            x: self.read_integer(),
            y: self.read_integer(),
            width: self.read_integer(),
            height: self.read_integer(),
            id: id.to_string(),
        }
    }
}

impl<R: std::io::Read + ?Sized> GraphicsReadExt for R {}

#[derive(Debug)]
pub enum GraphicsError {
    FileNotFound,
    InvalidData,
}

type Graphics = (BTreeMap<String, Image>, BTreeMap<String, Animation>);

pub fn parse_from_reader<T: Read>(reader: &mut BufReader<T>) -> Result<Graphics, GraphicsError> {
    let mut images = BTreeMap::<String, Image>::new();
    let mut animations = BTreeMap::<String, Animation>::new();

    reader.read_long();
    reader.read_long();

    while let Ok(grh) = reader.read_u32::<Endian>() {
        if grh == 0 {
            break;
        }
        match reader.read_integer() {
            0 => {
                return Err(GraphicsError::InvalidData);
            }
            1 => {
                images.insert(grh.to_string(), reader.read_image(grh));
            }
            frames_len => {
                animations.insert(grh.to_string(), reader.read_animation(grh, frames_len));
            }
        }
    }

    Ok((images, animations))
}

pub fn parse_from_bytes(bytes: &[u8]) -> Result<Graphics, GraphicsError> {
    let mut reader = BufReader::new(bytes);
    parse_from_reader(&mut reader)
}

pub fn parse_graphics(path: &str) -> Result<Graphics, GraphicsError> {
    let mut reader = get_binary_reader(path).map_err(|_| GraphicsError::FileNotFound)?;
    parse_from_reader(&mut reader)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_graphics() {
        use super::*;
        let result = parse_graphics("../client/assets/ao_20/init/graficos.ind").unwrap();
        assert_eq!(result.0.len(), 52734);
        assert_eq!(result.1.len(), 2802);
    }
}
