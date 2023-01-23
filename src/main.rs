
#[derive(PartialEq)]
#[derive(Debug)]
enum Size {
    Small,
    Big,
}

#[derive(PartialEq)]
#[derive(Debug)]
struct Piece {
    owner: usize,
    size: Size,
}

#[derive(PartialEq)]
#[derive(Debug)]
struct Player {
    name: String,
    piece_pool: Vec<Piece>
}

#[derive(PartialEq)]
#[derive(Debug)]
enum Cell {
    Empty,
    Piece(Piece),
}

#[derive(Debug)]
struct GameState {
    game_board: [[Cell; 6]; 6],
    turn_order: Vec<Player>,
    turn_count: u32,
}

impl GameState {

    fn check_cell(&self, coordinate: (usize, usize)) -> Vec<&str> {
        match &self.game_board[coordinate.0][coordinate.1] {
            Cell::Empty => {
                println!("Empty Cell");
                Vec::new()
            }
            Cell::Piece(Piece{owner:current_owner, size:current_size}) => {
                println!("{:?}, {:?}", current_owner, current_size);

                let mut matches: Vec<&str> = Vec::new();

                //horizontal check
                if let Cell::Piece(Piece{owner:_current_owner, size:_}) = &self.game_board[coordinate.0+1][coordinate.1] {
                    if let Cell::Piece(Piece{owner:_current_owner, size:_}) = &self.game_board[coordinate.0-1][coordinate.1] {
                        println!("Horizontal Match!");
                        matches.push("Horizontal");
                    }
                }
                //vertical check
                if let Cell::Piece(Piece{owner:_current_owner, size:_}) = &self.game_board[coordinate.0][coordinate.1+1] {
                    if let Cell::Piece(Piece{owner:_current_owner, size:_}) = &self.game_board[coordinate.0][coordinate.1-1] {
                        println!("Vertical Match!");
                        matches.push("Vertical");
                    }
                }
                //left diagonal check
                if let Cell::Piece(Piece{owner:_current_owner, size:_}) = &self.game_board[coordinate.0+1][coordinate.1+1] {
                    if let Cell::Piece(Piece{owner:_current_owner, size:_}) = &self.game_board[coordinate.0-1][coordinate.1-1] {
                        println!("Left Diagonal Match!");
                        matches.push("Left Diagonal");
                    }
                }
                //right diagonal check
                if let Cell::Piece(Piece{owner:_current_owner, size:_}) = &self.game_board[coordinate.0+1][coordinate.1-1] {
                    if let Cell::Piece(Piece{owner:_current_owner, size:_}) = &self.game_board[coordinate.0-1][coordinate.1+1] {
                        println!("Right Diagonal Match!");
                        matches.push("Right Diagonal");
                    }
                }
                matches
            }
        }
    }

    fn check_board(&self) {
        for row_index in 1..self.game_board.len()-1 {
            for column_index in 1..self.game_board[row_index].len() {
                let _ = dbg!(&self.check_cell((row_index, column_index)));
            }
        }
    }
}

fn init() -> GameState {
    GameState {
        game_board: [
            [Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            [Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            [Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            [Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            [Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            [Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
        ],
        turn_order: vec![
        Player{name: String::from("Player 1"), piece_pool: 
            vec![Piece{owner: 0, size: Size::Small}, 
                 Piece{owner: 0, size: Size::Small}, 
                 Piece{owner: 0, size: Size::Small}, 
                 Piece{owner: 0, size: Size::Small}, 
                 Piece{owner: 0, size: Size::Small}, 
                 Piece{owner: 0, size: Size::Small}, 
                 Piece{owner: 0, size: Size::Small}, 
                 Piece{owner: 0, size: Size::Small},
        ]},
        Player{name: String::from("Player 2"), piece_pool: 
            vec![Piece{owner: 1, size: Size::Small}, 
                 Piece{owner: 1, size: Size::Small}, 
                 Piece{owner: 1, size: Size::Small}, 
                 Piece{owner: 1, size: Size::Small}, 
                 Piece{owner: 1, size: Size::Small}, 
                 Piece{owner: 1, size: Size::Small}, 
                 Piece{owner: 1, size: Size::Small}, 
                 Piece{owner: 1, size: Size::Small},
        ]},
        ],
        turn_count: 0,
    }
}

fn main() {
    let mut game_state = init();

    game_state.game_board[1][1] = Cell::Piece(Piece{owner: 0, size: Size::Big});
    game_state.game_board[2][0] = Cell::Piece(Piece{owner: 0, size: Size::Small});
    game_state.game_board[0][2] = Cell::Piece(Piece{owner: 0, size: Size::Small});

    game_state.check_board();
}
