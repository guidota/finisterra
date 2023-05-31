use byteorder::ReadBytesExt;

use crate::parse::*;

#[derive(Debug)]
pub struct Image {
    pub file_num: u16,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub id: u16,
}

#[derive(Debug)]
pub struct Animation {
    pub frames: Vec<u16>,
    pub speed: u16,
    pub id: u16,
}

pub trait GraphicsReadExt: std::io::Read {
    fn read_animation(&mut self, id: u16, frames_len: u16) -> Animation {
        Animation {
            frames: (0..frames_len)
                .map(|_| self.read_integer())
                .collect::<Vec<u16>>(),
            speed: self.read_integer(),
            id,
        }
    }

    fn read_image(&mut self, id: u16) -> Image {
        Image {
            file_num: dbg!(self.read_integer()),
            x: dbg!(self.read_integer()),
            y: dbg!(self.read_integer()),
            width: dbg!(self.read_integer()),
            height: dbg!(self.read_integer()),
            id,
        }
    }
}

impl<R: std::io::Read + ?Sized> GraphicsReadExt for R {}

#[derive(Debug)]
pub enum GraphicsError {
    InvalidData,
}

pub fn parse_graphics(path: &str) -> Result<(Vec<Image>, Vec<Animation>), GraphicsError> {
    let graphics = File::open(path).expect("Failed to open graphics file");
    let reader = &mut BufReader::new(graphics);
    let mut images = Vec::new();
    let mut animations = Vec::new();

    reader.skip_header();
    reader.skip_temp_ints(5);

    let mut grh = reader.read_integer();
    while grh > 0 {
        let frames_len = reader.read_integer();

        match frames_len {
            0 => return Err(GraphicsError::InvalidData),
            1 => images.push(dbg!(reader.read_image(grh))),
            _ => animations.push(reader.read_animation(grh, frames_len)),
        }
        grh = match reader.read_u16::<Endian>() {
            Ok(val) => val,
            Err(_) => break,
        };
    }

    Ok((images, animations))
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn test_graphics() {
//         use crate::ao_libre::graphics::parse_graphics;
//         let result = dbg!(parse_graphics("assets/init/graphics.ao").unwrap());
//         assert_eq!(result.0.len(), 6630);
//         assert_eq!(result.1.len(), 633);
//     }
// }
