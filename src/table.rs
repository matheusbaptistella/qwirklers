use crate::tile::Tile;

pub struct Table {
    grid: [[Option<Tile>; 108]; 108],
}

impl Table {
    pub fn new() -> Self {
        Table {
            grid: [[None; 108]; 108]
        }
    }

    pub fn add_tile(&mut self, tile: Tile, i: usize, j: usize) {
        if let Some(_) = self.grid[i][j] {
            panic!("There is a tile here already!");
        }

        self.grid[i][j] = Some(tile);
    }
}
