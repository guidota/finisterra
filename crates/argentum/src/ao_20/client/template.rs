use ini::Ini;
use rustc_hash::FxHashMap;

use crate::parse::{get_ini_reader, ArgentumIniPropertyReadExt, ArgentumIniReadExt};

// [Molde1]
// X=0
// Y=0
// Width=27
// Height=47
// Dir1=6
// Dir2=6
// Dir3=5
// Dir4=5

#[derive(Clone, Debug)]
pub struct Template {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    // amount of frames per dir
    pub dirs: (usize, usize, usize, usize),
}

impl Template {
    pub fn total_rects(&self) -> usize {
        self.dirs.0 + self.dirs.1 + self.dirs.2 + self.dirs.3
    }
}
//
// pub fn parse_templates_from_bytes(mut bytes: &[u8]) -> BTreeMap<usize, Template> {
//     let mut templates = BTreeMap::new();
//     let ini = Ini::read_from(&mut bytes).expect("File doesn't exist");
//
//     for template in 1..=ini.get_count("Moldes") {
//         templates.insert(template, parse_template(template, &ini));
//     }
//
//     templates
// }

pub fn parse_templates(path: &str) -> FxHashMap<usize, Template> {
    let mut templates = FxHashMap::default();

    let ini = get_ini_reader(path).unwrap();
    for template in 1..=ini.get_count("Moldes") {
        templates.insert(template, parse_template(template, &ini));
    }

    templates
}

fn parse_template(number: usize, ini: &Ini) -> Template {
    let template_section = ini
        .section(Some(&format!("Molde{number}")))
        .expect("Molde {head_number} doesn't exist");
    Template {
        x: template_section.get_number("X"),
        y: template_section.get_number("Y"),
        width: template_section.get_number("Width"),
        height: template_section.get_number("Height"),
        dirs: (
            template_section.get_number("Dir1"),
            template_section.get_number("Dir2"),
            template_section.get_number("Dir3"),
            template_section.get_number("Dir4"),
        ),
    }
}

pub struct Rect {
    pub min: (usize, usize),
    pub max: (usize, usize),
}

impl IntoIterator for Template {
    type Item = (usize, Rect);

    type IntoIter = TempalateIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        TempalateIntoIterator {
            template: self,
            index: 0,
        }
    }
}

pub struct TempalateIntoIterator {
    template: Template,
    index: usize,
}

impl Iterator for TempalateIntoIterator {
    type Item = (usize, Rect);

    fn next(&mut self) -> Option<(usize, Rect)> {
        if self.index >= self.template.total_rects() {
            return None;
        }
        let mut y = 0;
        let mut x = self.index;
        if self.index >= self.template.dirs.0 {
            y += 1;
            x -= self.template.dirs.0;
        }
        if self.index >= self.template.dirs.0 + self.template.dirs.1 {
            y += 1;
            x -= self.template.dirs.1;
        }
        if self.index >= self.template.dirs.0 + self.template.dirs.1 + self.template.dirs.2 {
            y += 1;
            x -= self.template.dirs.2;
        }

        let min_x = self.template.x + (x * self.template.width);
        let min_y = self.template.y + (y * self.template.height);
        let rect = Rect {
            min: (min_x, min_y),
            max: (min_x + self.template.width, min_y + self.template.height),
        };

        self.index += 1;
        Some((y, rect))
    }
}
