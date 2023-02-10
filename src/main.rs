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

#[derive(Debug)]
enum MatchDir {
    Horizontal,
    Vertical,
    LeftDiagonal,
    RightDiagonal,
}

impl GameState {

    fn check_cell(&self, coordinate: (usize, usize)) -> Result<Vec<MatchDir>, &'static str> {

        if coordinate.0 > self.game_board.len() {
            return Result::Err("X Coordinate Out of Bounds");
        }
        if coordinate.1 > self.game_board[coordinate.0].len() {
            return Result::Err("Y Coordinate Out of Bounds");
        }

        match &self.game_board[coordinate.0][coordinate.1] {
            Cell::Empty => {
                Result::Ok(Vec::new())
            }
            Cell::Piece(Piece{owner:current_owner, size:_}) => {

                let mut matches: Vec<MatchDir> = Vec::new();

                //horizontal check
                if let Cell::Piece(Piece{owner:current_owner1, size:_}) = self.game_board[coordinate.0+1][coordinate.1] {
                    if let Cell::Piece(Piece{owner:current_owner2, size:_}) = self.game_board[coordinate.0-1][coordinate.1] {
                        if current_owner == &current_owner1 && &current_owner1 == &current_owner2 {
                            matches.push(MatchDir::Horizontal);
                        }
                    }
                }
                //vertical check
                if let Cell::Piece(Piece{owner:current_owner1, size:_}) = self.game_board[coordinate.0][coordinate.1+1] {
                    if let Cell::Piece(Piece{owner:current_owner2, size:_}) = self.game_board[coordinate.0][coordinate.1-1] {
                        if current_owner == &current_owner1 && &current_owner1 == &current_owner2 {
                            matches.push(MatchDir::Vertical);
                        }
                    }
                }
                //left diagonal check
                if let Cell::Piece(Piece{owner:current_owner1, size:_}) = self.game_board[coordinate.0+1][coordinate.1+1] {
                    if let Cell::Piece(Piece{owner:current_owner2, size:_}) = self.game_board[coordinate.0-1][coordinate.1-1] {
                        if current_owner == &current_owner1 && &current_owner1 == &current_owner2 {
                            matches.push(MatchDir::LeftDiagonal);
                        }
                    }
                }
                //right diagonal check
                if let Cell::Piece(Piece{owner:current_owner1, size:_}) = self.game_board[coordinate.0+1][coordinate.1-1] {
                    if let Cell::Piece(Piece{owner:current_owner2, size:_}) = self.game_board[coordinate.0-1][coordinate.1+1] {
                        if current_owner == &current_owner1 && &current_owner1 == &current_owner2 {
                            matches.push(MatchDir::RightDiagonal);
                        }
                    }
                }
                Result::Ok(matches)
            }
        }
    }

    fn check_board(&self) {
        for row_index in 1..self.game_board.len()-1 {
            for column_index in 1..self.game_board[row_index].len()-1 {
                let _ = dbg!(&self.check_cell((row_index, column_index)).unwrap());
            }
        }
    }

    fn place_piece(&mut self, coordinate: (usize, usize), size: Size) -> Result<(), &'static str>{

        if coordinate.0 > self.game_board.len() {
            return Result::Err("X Coordinate Out of Bounds");
        }
        if coordinate.1 > self.game_board[coordinate.0].len() {
            return Result::Err("Y Coordinate Out of Bounds");
        }
        if self.game_board[coordinate.0][coordinate.1] != Cell::Empty {
            return Result::Err("Attempted Illegal Move");
        }

        let player_index = dbg!(self.turn_count % self.turn_order.len());

        self.game_board[coordinate.0][coordinate.1] = Cell::Piece(Piece { owner: (player_index), size: (size) });

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

    game_state.game_board[2][0] = Cell::Piece(Piece{owner: 0, size: Size::Small});
    game_state.game_board[0][2] = Cell::Piece(Piece{owner: 0, size: Size::Small});

    let _ = game_state.place_piece((2,0), Size::Big);

    dbg!(game_state.check_board());

}
