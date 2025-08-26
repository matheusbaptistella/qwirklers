use crate::tile::Tile;
use rand::Rng;

pub struct Bag {
    tiles: Vec<Tile>,
}

impl Bag {
    pub fn new() -> Self {
        let tiles = (0..3).flat_map(|_| Tile::all()).collect();
        Bag { tiles }
    }

    fn try_pick_one(&mut self) -> Option<Tile> {
        if self.tiles.len() == 0 {
            return None;
        }

        let mut rng = rand::rng();
        let index = rng.random_range(0..self.tiles.len());

        Some(self.tiles.swap_remove(index))
    }

    pub fn try_pick_n(&mut self, n: u8) -> Option<Vec<Tile>> {
        if n == 0 || n > 6 {
            panic!("You are not allowed to get this number of tiles!")
        }

        (0..n).map(|_| self.try_pick_one()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod try_pick_one {
        use super::*;

        #[test]
        fn try_pick_one_nonempty_bag_returns_some() {
            let mut bag = Bag::new();
            let tile = bag.try_pick_one();

            assert!(tile.is_some(), "expected Some, got {tile:?}");
        }

        #[test]
        fn try_pick_one_empty_bag_returns_none() {
            let mut bag = Bag::new();
            bag.tiles = Vec::new();

            let tile = bag.try_pick_one();

            assert!(tile.is_none(), "expected None, got {tile:?}");
        }
    }

    mod try_pick_n {
        use crate::tile::{Color, Shape};

        use super::*;

        #[test]
        fn try_pick_n_nonempty_bag_returns_some() {
            let mut bag = Bag::new();
            let tiles = bag.try_pick_n(6);

            assert!(tiles.is_some(), "expected Some, got {tiles:?}");
            assert_eq!(tiles.unwrap().len(), 6)
        }

        #[test]
        fn try_pick_n_empty_bag_returns_none() {
            let mut bag = Bag::new();
            bag.tiles = Vec::new();

            let tiles = bag.try_pick_n(6);

            assert!(tiles.is_none(), "expected None, got {tiles:?}");
        }

        #[test]
        fn try_pick_n_insufficient_bag_returns_some() {
            let mut bag = Bag::new();
            bag.tiles = vec![Tile::new(Color::Blue, Shape::Circle)];

            let tiles = bag.try_pick_n(6);

            assert!(tiles.is_some(), "expected Some, got {tiles:?}");
            assert_eq!(tiles.unwrap().len(), 1)
        }
    }
}
