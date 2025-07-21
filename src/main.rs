use std::{fmt, io};

fn main() {
    start();
}

fn start() {
    loop {
        let game_mode = select_game_mode();

        match game_mode {
            Some(mode) => Game::with_mode(mode).play(),
            None => break,
        }
    }
}

fn select_game_mode() -> Option<GameMode> {
    let mut input = String::new();

    loop {
        print!("Select an option to play:\n[1] Local\n[2] Online\n[3] Exit\n\n>> ");

        io::stdin().read_line(&mut input).expect("Failed to read Game Mode from user");

        match input.trim() {
            "1" => return Some(GameMode::Local),
            "2" => return Some(GameMode::Online),
            "3" => return None,
            _ => println!("Invalid option!\n"),
        }

        input.clear();
    }
}

struct Game {
    mode: GameMode,
    bag: Bag,
}

impl Game {
    fn with_mode(mode: GameMode) -> Self {
        Game {
            mode: mode,
        }
    }

    fn play(&mut self) {
        match self.mode {
            GameMode::Local => self.play_local(),
            GameMode::Online => println!("TODO"),
        }
    }

    fn play_local(&mut self) {
        let num_players = self.select_number_of_players();

    }

    fn select_number_of_players(&mut self) -> u8{
        let mut input = String::new();
        
        loop {
            print!("Select number of players (min 2, max 4):\n>> ");

            io::stdin().read_line(&mut input).expect("Failed to read Game Mode from user");

            let num_players = input.trim().parse::<u8>();

            match num_players {
                Ok(num) => match num {
                    num if (num > 1 && num < 5) => return num,
                    _ => println!("Invalid amount of players!"),
                }
                Err(_) => println!("Invalid amount of players!"),
            }

            input.clear();
        }
    }
}

struct Bag {
    tiles: Vec<(Tile, u8)>,
}

impl Bag {
    fn new() -> Self {
        let mut tiles = Vec::new();
        let all_colors = [Color::Red, Color::Orange, Color::Yellow, Color::Green, Color::Blue, Color::Purple];
        let all_shapes = [Shape::Circle, Shape::Star, Shape::Diamond, Shape::Starburst, Shape::Clover];

        for color in all_colors {
            for shape in all_shapes {
                tiles.push((Tile { color, shape}, 3));
            }
        }

        Bag {
            tiles: tiles,
        }
    }
}

struct Tile {
    color: Color,
    shape: Shape,
}


enum Color { 
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
}


enum Shape {
    Circle,
    Star,
    Diamond,
    Square,
    Starburst,
    Clover,
}


enum GameMode {
    Local,
    Online,
}