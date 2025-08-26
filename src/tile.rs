#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shape {
    Circle,
    Star,
    Diamond,
    Square,
    Starburst,
    Clover,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Tile {
    pub color: Color,
    pub shape: Shape,
}

impl Color {
    pub const ALL: [Color; 6] = [
        Color::Red,
        Color::Orange,
        Color::Yellow,
        Color::Green,
        Color::Blue,
        Color::Purple,
    ];
}

impl Shape {
    pub const ALL: [Shape; 6] = [
        Shape::Circle,
        Shape::Star,
        Shape::Diamond,
        Shape::Square,
        Shape::Starburst,
        Shape::Clover,
    ];
}

impl Tile {
    pub const fn new(color: Color, shape: Shape) -> Self {
        Self { color, shape }
    }

    pub fn all() -> impl Iterator<Item = Tile> {
        Color::ALL
            .into_iter()
            .flat_map(|c| Shape::ALL.into_iter().map(move |s| Tile::new(c, s)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_tiles() {
        let colors = Color::ALL;
        let shapes = Shape::ALL;
        let mut tiles = Vec::with_capacity(36);

        for c in colors {
            for s in shapes {
                tiles.push(Tile::new(c, s))
            }
        }

        assert_eq!(Tile::all().collect::<Vec<Tile>>(), tiles);
    }
}
