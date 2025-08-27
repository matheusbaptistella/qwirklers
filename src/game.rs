use crate::{bag::Bag, player::Player, table::Table, tile::Tile};

pub struct Game {
    table: Table,
    players: Vec<Player>,
    bag: Bag,
}

pub enum Command {
    PlaceNTiles(Vec<Tile>),
    TradeNTiles(Vec<Tile>),
}

impl Game {
    pub fn new() -> Self {
        Game {
            table: Table::new(),
            players: Vec::new(),
            bag: Bag::new(),
        }
    }

    pub fn add_player(&mut self, player: Player) {
        if self.players.len() >= 4 {
            panic!("Maximum amount of players reached!");
        }

        self.players.push(player)
    }

    pub fn start(&mut self) {
        // Determine the player with the most tile that share one attribute.
        // Prioritize players that joined the game early.
        let (p, d) = self
            .players
            .iter()
            .rev()
            .map(|p| (p, p.declare_largest()))
            .max_by_key(|(p, d)| d.count())
            .unwrap();
        
        let i = self.players.iter().position(|player| player == p).unwrap();

        self.players.rotate_left(i);

        for pidx in (0..self.players.len()).cycle() {
            let player = &mut self.players[pidx];

            let cmd = Command::TradeNTiles(Vec::new());

            match cmd {
                Command::PlaceNTiles(tiles) => {
                    if let Some(new_tiles) = self.bag.try_pick_n(tiles.len() as u8) {
                        player.insert_tiles(&new_tiles);
                    }
                },
                Command::TradeNTiles(tiles) => {
                    if let Some(new_tiles) = self.bag.try_pick_n(tiles.len() as u8) {
                        self.bag.insert(&tiles);
                        player.insert_tiles(&new_tiles);
                    }
                },
            }
        }
    }
}
