use crate::Offset;

#[derive(Default, Clone, Debug)]
pub struct Body {
    pub animations: [usize; 4],
    pub head_offset: Offset,
    pub override_file_num: Option<usize>,
}

// #[derive(Default, Clone, Debug)]
// pub struct Template {
//     pub x: usize,
//     pub y: usize,
//     pub width: usize,
//     pub height: usize,
//     // amount of frames per dir
//     pub frames_by_dir: (usize, usize, usize, usize),
// }