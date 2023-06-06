use std::{fs::File, io::Read, path::Path};

use image::GenericImageView;

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

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub height: u32,
    pub width: u32,
}

impl Texture {
    pub fn from_path(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        file_path: &str,
    ) -> anyhow::Result<Self> {
        let read_file = read_file(file_path);
        if let Ok(bytes) = read_file {
            return Texture::from_bytes(device, queue, &bytes, file_path);
        }

        anyhow::bail!("couldn't read texture from path");
    }

    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
        label: &str,
    ) -> anyhow::Result<Self> {
        let img = image::load_from_memory(bytes)?;
        Ok(Self::from_image(device, queue, &img, Some(label)))
    }

    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: &image::DynamicImage,
        label: Option<&str>,
    ) -> Self {
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        let format = wgpu::TextureFormat::Rgba8UnormSrgb;
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[format],
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Self {
            texture,
            view,
            sampler,
            width: dimensions.0,
            height: dimensions.1,
        }
    }
    //
    // pub fn from_texture(
    //     device: &wgpu::Device,
    //     texture: wgpu::Texture,
    //     label: Option<&str>,
    // ) -> Self {
    //     let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    //     let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
    //         label,
    //         ..Default::default()
    //     });
    //
    //     let size = texture.size();
    //
    //     Self {
    //         texture,
    //         view,
    //         sampler,
    //         width: size.width,
    //         height: size.height,
    //     }
    // }
}
