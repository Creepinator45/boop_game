use std::result;


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
    turn_count: usize,
}

impl GameState {

    fn check_cell(&self, coordinate: (usize, usize)) -> Vec<&str> {
        match &self.game_board[coordinate.0][coordinate.1] {
            Cell::Empty => {
                Vec::new()
            }
            Cell::Piece(Piece{owner:current_owner, size:current_size}) => {
                let mut matches: Vec<&str> = Vec::new();
                println!("{:?}", &current_owner);
                println!("{:?}", &current_size);
                //horizontal check
                if let Cell::Piece(Piece{owner:_current_owner, size:_}) = self.game_board[coordinate.0+1][coordinate.1] {
                    if let Cell::Piece(Piece{owner:_current_owner, size:_}) = self.game_board[coordinate.0-1][coordinate.1] {
                        matches.push("Horizontal");
                    }
                }
                //vertical check
                if let Cell::Piece(Piece{owner:_current_owner, size:_}) = self.game_board[coordinate.0][coordinate.1+1] {
                    if let Cell::Piece(Piece{owner:_current_owner, size:_}) = self.game_board[coordinate.0][coordinate.1-1] {
                        matches.push("Vertical");
                    }
                }
                //left diagonal check
                if let Cell::Piece(Piece{owner:_current_owner, size:_}) = self.game_board[coordinate.0+1][coordinate.1+1] {
                    if let Cell::Piece(Piece{owner:_current_owner, size:_}) = self.game_board[coordinate.0-1][coordinate.1-1] {
                        matches.push("Left Diagonal");
                    }
                }
                //right diagonal check
                if let Cell::Piece(Piece{owner:_current_owner, size:_}) = self.game_board[coordinate.0+1][coordinate.1-1] {
                    if let Cell::Piece(Piece{owner:_current_owner, size:_}) = self.game_board[coordinate.0-1][coordinate.1+1] {
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

    fn place_piece(&mut self, coordinate: (usize, usize), size: Size) -> Result<(), &'static str>{
        let player_index = dbg!(self.turn_count % self.turn_order.len());
        if self.game_board[coordinate.0][coordinate.1] == Cell::Empty{
            self.game_board[coordinate.0][coordinate.1] = Cell::Piece(Piece { owner: (player_index), size: (size) });
        }
        else {
            return Result::Err("Attempted Illegal Move");
        };

        Result::Ok(())
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

    let _ = game_state.place_piece((1,1), Size::Big);

    game_state.game_board[2][0] = Cell::Piece(Piece{owner: 1, size: Size::Small});
    game_state.game_board[0][2] = Cell::Piece(Piece{owner: 0, size: Size::Small});

    let _ = game_state.place_piece((2,0), Size::Big);

    dbg!(game_state.check_board());
}
