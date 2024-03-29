use std::rc::Rc;

use engine::engine::TextureID;

pub struct TextureArray {
    indices: Vec<Option<u32>>,
    textures: Vec<Rc<wgpu::TextureView>>,
    samplers: Vec<Rc<wgpu::Sampler>>,
    bind_group: Option<wgpu::BindGroup>,
}

impl TextureArray {
    pub fn new() -> Self {
        Self {
            indices: vec![],
            textures: vec![],
            samplers: vec![],
            bind_group: None,
        }
    }

    pub fn size(&self) -> u32 {
        self.textures.len() as u32
    }

    pub fn has_texture(&self, id: TextureID) -> bool {
        if id >= self.indices.len() as u32 {
            return false;
        }
        self.indices[id as usize].is_some()
    }

    pub fn get_index(&self, id: TextureID) -> Option<u32> {
        if id >= self.indices.len() as u32 {
            return None;
        }
        self.indices[id as usize]
    }

    pub fn push(
        &mut self,
        id: TextureID,
        texture: Rc<wgpu::TextureView>,
        sampler: Rc<wgpu::Sampler>,
    ) {
        let size = self.indices.len() as u32;
        if id >= size {
            for _ in size..id + 1 {
                self.indices.push(None);
            }
        }
        if self.indices[id as usize].is_some() {
            return;
        }

        let index = self.textures.len() as u32;
        self.indices[id as usize] = Some(index);
        self.textures.push(texture);
        self.samplers.push(sampler);
        self.bind_group = None;
    }

    pub fn prepare(&mut self, device: &wgpu::Device, bind_group_layout: &wgpu::BindGroupLayout) {
        if self.textures.is_empty() {
            return;
        }
        if self.bind_group.is_none() {
            log::info!(
                "recreating texture array bind group: {} textures",
                self.textures.len()
            );
            let texture_view_array = self
                .textures
                .iter()
                .map(|texture| texture.as_ref())
                .collect::<Vec<_>>();
            let samplers = self
                .samplers
                .iter()
                .map(|sampler| sampler.as_ref())
                .collect::<Vec<_>>();

            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureViewArray(&texture_view_array),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::SamplerArray(&samplers),
                    },
                ],
                layout: bind_group_layout,
                label: Some("bind group"),
            });
            self.bind_group = Some(bind_group);
        }
    }

    pub fn get_bind_group(&self) -> Option<&wgpu::BindGroup> {
        self.bind_group.as_ref()
    }
}
