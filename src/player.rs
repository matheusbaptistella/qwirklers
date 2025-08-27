use std::collections::HashMap;

use crate::tile::{Color, Shape, Tile};

pub enum Dominant {
    Color(Color, u8),
    Shape(Shape, u8),
}

#[derive(Clone, Default, Eq, PartialEq)]
pub struct Player {
    tiles: Vec<Tile>,
    score: u64,
}

impl Dominant {
    pub fn count(&self) -> u8 {
        match self {
            Dominant::Color(_, c) | Dominant::Shape(_, c) => *c,
        }
    }
}

impl Player {
    pub fn new() -> Self {
        Player::default()
    }

    pub fn declare_largest(&self) -> Dominant {
        let mut colors_c: HashMap<Color, u8> = HashMap::new();
        let mut shapes_c: HashMap<Shape, u8> = HashMap::new();

        for t in &self.tiles {
            *colors_c.entry(t.color).or_insert(0) += 1;
            *shapes_c.entry(t.shape).or_insert(0) += 1;
        }

        let (color, c_count) = colors_c.iter().max_by_key(|e| e.1).unwrap();
        let (shape, s_count) = shapes_c.iter().max_by_key(|e| e.1).unwrap();

        if c_count > s_count {
            return Dominant::Color(*color, *c_count);
        }

        Dominant::Shape(*shape, *s_count)
    }

    pub fn insert_tiles(&mut self, tiles: &[Tile]) {
        self.tiles.extend_from_slice(tiles);
    }
}
