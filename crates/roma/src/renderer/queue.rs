use std::slice;

use crate::DrawImageParams;

const DEFAULT_QUEUE_SIZE: usize = 1000;

type Queue = Vec<(u64, Vec<DrawImageParams>)>;

#[derive(Default)]
pub struct SpriteQueue {
    queue: Queue,
    texture_ids: Vec<u64>,
    size: usize,
}

impl SpriteQueue {
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn texture_ids(&self) -> Vec<u64> {
        self.texture_ids.to_vec()
    }

    pub fn push(&mut self, texture_id: u64, params: DrawImageParams) {
        if let Err(index) = self.texture_ids.binary_search(&texture_id) {
            self.texture_ids.insert(index, texture_id);
        }
        self.size += 1;
        match self.queue.binary_search_by_key(&texture_id, |i| i.0) {
            Err(index) => {
                let mut vec = Vec::with_capacity(DEFAULT_QUEUE_SIZE);
                vec.push(params);
                self.queue.insert(index, (texture_id, vec));
            }
            Ok(i) => {
                self.queue[i].1.push(params);
            }
        }
    }

    pub fn push_all(&mut self, texture_id: u64, params: &mut [DrawImageParams]) {
        if params.is_empty() {
            return;
        }

        self.size += params.len();
        if let Err(index) = self.texture_ids.binary_search(&texture_id) {
            self.texture_ids.insert(index, texture_id);
        }
        match self.queue.binary_search_by_key(&texture_id, |i| i.0) {
            Err(index) => {
                let mut vec = Vec::with_capacity(std::cmp::max(DEFAULT_QUEUE_SIZE, params.len()));
                vec.extend_from_slice(params);
                self.queue.insert(index, (texture_id, vec));
            }
            Ok(i) => {
                self.queue[i].1.extend_from_slice(params);
            }
        }
    }

    pub fn reset(&mut self) {
        for texture_id in &self.texture_ids {
            if let Ok(index) = self.queue.binary_search_by_key(texture_id, |i| i.0) {
                self.queue[index].1.clear();
            }
        }
        self.size = 0;
        self.texture_ids.clear();
    }

    pub fn batches(&mut self) -> slice::IterMut<(u64, Vec<DrawImageParams>)> {
        self.queue.iter_mut()
    }
}
