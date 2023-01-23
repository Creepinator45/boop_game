
#[derive(PartialEq)]
#[derive(Debug)]
enum Size {
    Small,
    Big,
}

#[derive(PartialEq)]
#[derive(Debug)]
struct Piece {
    owner: Player,
    size: Size,
}

#[derive(PartialEq)]
#[derive(Debug)]
enum Player {
    Player1,
    Player2,
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
    turn_player: Player
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
        for row_index in 1..5 {
            for column_index in 1..5 {
                let _ = dbg!(&self.check_cell((row_index, column_index)));
            }
        }
    }
}

fn main() {
    let mut game_state = GameState {
        game_board: [
            [Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            [Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            [Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            [Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            [Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            [Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
        ],
        turn_player: Player::Player1
    };

    game_state.game_board[1][1] = Cell::Piece(Piece{owner: Player::Player1, size: Size::Big});
    game_state.game_board[2][0] = Cell::Piece(Piece{owner: Player::Player1, size: Size::Small});
    game_state.game_board[0][2] = Cell::Piece(Piece{owner: Player::Player1, size: Size::Small});

    game_state.check_board();
}
