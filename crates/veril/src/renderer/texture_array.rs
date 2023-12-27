use std::rc::Rc;

use nohash_hasher::IntMap;

pub struct TextureArray {
    pub indices: IntMap<u64, u32>,
    textures: Vec<Rc<wgpu::TextureView>>,
    samplers: Vec<Rc<wgpu::Sampler>>,
    bind_group: Option<wgpu::BindGroup>,
}

impl TextureArray {
    pub fn new() -> Self {
        Self {
            indices: IntMap::default(),
            textures: vec![],
            samplers: vec![],
            bind_group: None,
        }
    }

    pub fn push(&mut self, id: u64, texture: Rc<wgpu::TextureView>, sampler: Rc<wgpu::Sampler>) {
        if self.indices.contains_key(&id) {
            return;
        }
        let index = self.textures.len() as u32;

        self.indices.insert(id, index);
        self.textures.push(texture);
        self.samplers.push(sampler);
        self.bind_group = None;
    }

    pub fn prepare(&mut self, device: &wgpu::Device, bind_group_layout: &wgpu::BindGroupLayout) {
        if self.textures.is_empty() {
            return;
        }
        if self.bind_group.is_none() {
            log::info!("recreating texture array bind group");
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
