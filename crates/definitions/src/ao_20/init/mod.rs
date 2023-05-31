pub mod parse;

#[derive(Debug)]
pub enum Body {
    AnimatedWithTemplate {
        template_id: usize,
        file_num: usize,
        head_offset: (isize, isize),
    },
    Animated {
        walks: (usize, usize, usize, usize),
        head_offset: (isize, isize),
    },
}

impl Body {
    pub fn get_head_offset(&self) -> (isize, isize) {
        match self {
            Body::AnimatedWithTemplate {
                template_id: _,
                file_num: _,
                head_offset,
            } => *head_offset,
            Body::Animated {
                walks: _,
                head_offset,
            } => *head_offset,
        }
    }
}

#[derive(Debug)]
pub struct Head(pub usize, pub usize, pub usize, pub usize);

#[derive(Debug)]
pub struct Weapon(usize, usize, usize, usize);
