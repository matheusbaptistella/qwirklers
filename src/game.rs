use crate::{bag::Bag, player::Player, table::Table};

pub struct Game {
    table: Table,
    players: Vec<Player>,
    bag: Bag,
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
        // Prioritize players that joined teh game early.
        let (p, d) = self.players.iter().rev().map(|p| (p, p.declare_largest())).max_by_key(|(_, d)| d.count()).unwrap();

        loop {

        }
    }
}
