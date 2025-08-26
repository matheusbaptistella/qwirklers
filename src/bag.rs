use crate::tile::Tile;
use rand::Rng;

struct Bag {
    tiles: Vec<Tile>,
}

impl Bag {
    pub fn new() -> Self {
        let tiles = (0..3).flat_map(|_| Tile::all()).collect();
        Bag { tiles }
    }

    pub fn try_pick_one(&mut self) -> Option<Tile> {
        let mut rng = rand::rng();

        if self.tiles.len() == 0 {
            return None;
        }

        let index = rng.random_range(0..self.tiles.len());

        Some(self.tiles.swap_remove(index))
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
}
