use std::rc::Rc;
use std::time::Duration;

use crate::animations::Animation;
use crate::error::Error;
use crate::image::Image;
use crate::parse::{get_binary_reader, ArgentumReadExt, Endian};
use byteorder::ReadBytesExt;
use rustc_hash::FxHashMap;

// pub mod client;
pub mod maps;
// pub mod server;

pub struct Graphics {
    pub images: FxHashMap<u32, Rc<Image>>,
    pub animations: FxHashMap<usize, Animation<u32>>,
}

/// graficos.ini or graficos.ind
pub fn load_graphics(path: &str) -> Result<Graphics, Error> {
    let mut reader = get_binary_reader(path)?;
    let mut images = FxHashMap::<u32, Rc<Image>>::default();
    let mut animations = FxHashMap::<usize, Animation<u32>>::default();

    reader.read_long();
    reader.read_long();

    while let Ok(grh) = reader.read_u32::<Endian>() {
        if grh == 0 {
            break;
        }
        match reader.read_integer() {
            0 => {
                return Err(Error::Parse);
            }
            1 => {
                let image = Image {
                    file: reader.read_long() as u32,
                    x: reader.read_integer(),
                    y: reader.read_integer(),
                    width: reader.read_integer(),
                    height: reader.read_integer(),
                    id: grh,
                };
                images.insert(grh, Rc::new(image));
            }
            frames_len => {
                let animation = Animation {
                    frames: (0..frames_len)
                        .map(|_| reader.read_long() as u32)
                        .collect::<Vec<_>>(),
                    duration: Duration::from_millis(reader.read_long() as u64),
                };
                animations.insert(grh as usize, animation);
            }
        }
    }
    Ok(Graphics { images, animations })
}
