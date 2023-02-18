use std::num::ParseIntError;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Size {
    Small,
    Big,
}

#[derive(Clone, PartialEq, Debug)]
struct Piece {
    owner: usize,
    size: Size,
}

#[derive(PartialEq, Debug)]
struct Player {
    name: String,
    piece_pool: Vec<Piece>,
}

#[derive(Clone, PartialEq, Debug)]
enum Cell {
    OutOfBounds,
    Empty,
    Piece(Piece),
}

#[derive(Debug)]
struct GameState {
    game_board: [[Cell; 10]; 10],
    turn_order: [Player; 2],
    turn_count: usize,
}

#[derive(Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct PiecePlacement {
    coordinate: Coordinate,
    size: Size,
}

#[derive(Debug)]
enum MatchDir {
    Horizontal,
    Vertical,
    LeftDiagonal,
    RightDiagonal,
}

impl PiecePlacement {
    fn ask_player() -> Result<PiecePlacement, ParseIntError> {
        let mut position_input = (String::new(), String::new());
    
        println!("Position: X");
        std::io::stdin()
            .read_line(&mut position_input.0)
            .expect("Failed to read line");
    
        println!("Position: Y");
        std::io::stdin()
            .read_line(&mut position_input.1)
            .expect("Failed to read line");
    
        let mut size_input = String::new();
    
        println!("Size:");
        std::io::stdin()
            .read_line(&mut size_input)
            .expect("Failed to read line");
    
        let move_coordinate = Coordinate {
            x: position_input.0.trim().parse()?,
            y: position_input.0.trim().parse()?,
        };
    
        Result::Ok(PiecePlacement {
            coordinate: move_coordinate,
            size: Size::Small,
        })
    }
}

impl GameState {
    fn check_cell(&self, coordinate: Coordinate) -> Result<Vec<MatchDir>, &'static str> {
        if coordinate.x > 7 || coordinate.x < 2 {
            return Result::Err("X Coordinate Out of Bounds");
        }
        if coordinate.y > 7 || coordinate.x < 2 {
            return Result::Err("Y Coordinate Out of Bounds");
        }

        match &self.game_board[coordinate.x][coordinate.y] {
            Cell::OutOfBounds => Result::Err("Placing in Null Cell"),
            Cell::Empty => Result::Ok(Vec::new()),
            Cell::Piece(Piece {
                owner: current_owner,
                size: _,
            }) => {
                let mut matches: Vec<MatchDir> = Vec::new();

                //horizontal check
                if let Cell::Piece(Piece {
                    owner: current_owner1,
                    size: _,
                }) = self.game_board[coordinate.x + 1][coordinate.y]
                {
                    if let Cell::Piece(Piece {
                        owner: current_owner2,
                        size: _,
                    }) = self.game_board[coordinate.x - 1][coordinate.y]
                    {
                        if current_owner == &current_owner1 && &current_owner1 == &current_owner2 {
                            matches.push(MatchDir::Horizontal);
                        }
                    }
                }
                //vertical check
                if let Cell::Piece(Piece {
                    owner: current_owner1,
                    size: _,
                }) = self.game_board[coordinate.x][coordinate.y + 1]
                {
                    if let Cell::Piece(Piece {
                        owner: current_owner2,
                        size: _,
                    }) = self.game_board[coordinate.x][coordinate.y - 1]
                    {
                        if current_owner == &current_owner1 && &current_owner1 == &current_owner2 {
                            matches.push(MatchDir::Vertical);
                        }
                    }
                }
                //left diagonal check
                if let Cell::Piece(Piece {
                    owner: current_owner1,
                    size: _,
                }) = self.game_board[coordinate.x + 1][coordinate.y + 1]
                {
                    if let Cell::Piece(Piece {
                        owner: current_owner2,
                        size: _,
                    }) = self.game_board[coordinate.x - 1][coordinate.y - 1]
                    {
                        if current_owner == &current_owner1 && &current_owner1 == &current_owner2 {
                            matches.push(MatchDir::LeftDiagonal);
                        }
                    }
                }
                //right diagonal check
                if let Cell::Piece(Piece {
                    owner: current_owner1,
                    size: _,
                }) = self.game_board[coordinate.x + 1][coordinate.y - 1]
                {
                    if let Cell::Piece(Piece {
                        owner: current_owner2,
                        size: _,
                    }) = self.game_board[coordinate.x - 1][coordinate.y + 1]
                    {
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
        for row_index in 2..7 {
            for column_index in 2..7 {
                let _ = &self
                    .check_cell(Coordinate {
                        x: row_index,
                        y: column_index
                    })
                    .unwrap();
            }
        }
    }

    fn place_piece(&mut self, piece_placement: PiecePlacement) -> Result<(), &'static str> {
        let player_index = self.turn_count % self.turn_order.len();

        let coordinate = Coordinate {
            x: piece_placement.coordinate.x + 2,
            y: piece_placement.coordinate.y + 2,
        };

        if coordinate.x > 7 || coordinate.x < 2 {
            return Result::Err("X Coordinate Out of Bounds");
        }
        if coordinate.y > 7 || coordinate.x < 2 {
            return Result::Err("Y Coordinate Out of Bounds");
        }

        if self.game_board[coordinate.x][coordinate.y] != Cell::Empty {
            return Result::Err("Placement Position Occupied");
        }
        if !self.turn_order[player_index].piece_pool.contains(&Piece {
            owner: player_index,
            size: piece_placement.size,
        }) {
            return Result::Err("Requested Piece Size Not Available");
        }

        self.turn_order[player_index].piece_pool.remove(
            self.turn_order[player_index]
                .piece_pool
                .iter()
                .position(|x| {
                    *x == Piece {
                        owner: player_index,
                        size: piece_placement.size,
                    }
                })
                .expect("needle not found"),
        );

        self.game_board[coordinate.x][coordinate.y] = Cell::Piece(Piece {
            owner: (player_index),
            size: (piece_placement.size),
        });

        let dirs = [
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 2),
            (2, 2),
            (2, 1),
            (2, 0),
            (1, 0),
        ];
        for d in dirs {
            if let Cell::Piece(_) = self.game_board[coordinate.x - 1 + d.0][coordinate.y - 1 + d.1]
            {
                match self.game_board[coordinate.x - 2 + d.0 * 2][coordinate.y - 2 + d.1 * 2] {
                    Cell::Piece(_) => continue,
                    Cell::OutOfBounds => {
                        self.game_board[coordinate.x - 1 + d.0][coordinate.y - 1 + d.1] =
                            Cell::Empty;
                    }
                    Cell::Empty => {
                        self.game_board[coordinate.x - 2 + d.0 * 2][coordinate.y - 2 + d.1 * 2] =
                            self.game_board[coordinate.x - 1 + d.0][coordinate.y - 1 + d.1].clone();

                        self.game_board[coordinate.x - 1 + d.0][coordinate.y - 1 + d.1] =
                            Cell::Empty;
                    }
                }
            }
        }
        Result::Ok(())
    }
}


fn init() -> GameState {
    GameState {
        game_board: [
            [ Cell::OutOfBounds, Cell::OutOfBounds, Cell::OutOfBounds, Cell::OutOfBounds, Cell::OutOfBounds, Cell::OutOfBounds, Cell::OutOfBounds, Cell::OutOfBounds, Cell::OutOfBounds, Cell::OutOfBounds,],
            [ Cell::OutOfBounds, Cell::OutOfBounds, Cell::OutOfBounds, Cell::OutOfBounds, Cell::OutOfBounds, Cell::OutOfBounds, Cell::OutOfBounds, Cell::OutOfBounds, Cell::OutOfBounds, Cell::OutOfBounds,],
            [ Cell::OutOfBounds, Cell::OutOfBounds, Cell::Empty,       Cell::Empty,       Cell::Empty,       Cell::Empty,       Cell::Empty,       Cell::Empty,       Cell::OutOfBounds, Cell::OutOfBounds,],
            [ Cell::OutOfBounds, Cell::OutOfBounds, Cell::Empty,       Cell::Empty,       Cell::Empty,       Cell::Empty,       Cell::Empty,       Cell::Empty,       Cell::OutOfBounds, Cell::OutOfBounds,],
            [ Cell::OutOfBounds, Cell::OutOfBounds, Cell::Empty,       Cell::Empty,       Cell::Empty,       Cell::Empty,       Cell::Empty,       Cell::Empty,       Cell::OutOfBounds, Cell::OutOfBounds,],
            [ Cell::OutOfBounds, Cell::OutOfBounds, Cell::Empty,       Cell::Empty,       Cell::Empty,       Cell::Empty,       Cell::Empty,       Cell::Empty,       Cell::OutOfBounds, Cell::OutOfBounds,],
            [ Cell::OutOfBounds, Cell::OutOfBounds, Cell::Empty,       Cell::Empty,       Cell::Empty,       Cell::Empty,       Cell::Empty,       Cell::Empty,       Cell::OutOfBounds, Cell::OutOfBounds,],
            [ Cell::OutOfBounds, Cell::OutOfBounds, Cell::Empty,       Cell::Empty,       Cell::Empty,       Cell::Empty,       Cell::Empty,       Cell::Empty,       Cell::OutOfBounds, Cell::OutOfBounds,],
            [ Cell::OutOfBounds, Cell::OutOfBounds, Cell::OutOfBounds, Cell::OutOfBounds, Cell::OutOfBounds, Cell::OutOfBounds, Cell::OutOfBounds, Cell::OutOfBounds, Cell::OutOfBounds, Cell::OutOfBounds,],
            [ Cell::OutOfBounds, Cell::OutOfBounds, Cell::OutOfBounds, Cell::OutOfBounds, Cell::OutOfBounds, Cell::OutOfBounds, Cell::OutOfBounds, Cell::OutOfBounds, Cell::OutOfBounds, Cell::OutOfBounds,],
        ],
        turn_order: [
            Player {
                name: String::from("Player 1"),
                piece_pool: vec![
                    Piece { owner: 0, size: Size::Small,},
                    Piece { owner: 0, size: Size::Small,},
                    Piece { owner: 0, size: Size::Small,},
                    Piece { owner: 0, size: Size::Small,},
                    Piece { owner: 0, size: Size::Small,},
                    Piece { owner: 0, size: Size::Small,},
                    Piece { owner: 0, size: Size::Small,},
                    Piece { owner: 0, size: Size::Small,},
                ],
            },
            Player {
                name: String::from("Player 2"),
                piece_pool: vec![
                    Piece { owner: 1, size: Size::Small,},
                    Piece { owner: 1, size: Size::Small,},
                    Piece { owner: 1, size: Size::Small,},
                    Piece { owner: 1, size: Size::Small,},
                    Piece { owner: 1, size: Size::Small,},
                    Piece { owner: 1, size: Size::Small,},
                    Piece { owner: 1, size: Size::Small,},
                    Piece { owner: 1, size: Size::Small,},
                ],
            },
        ],
        turn_count: 0,
    }
}

fn main() {
    let mut game_state = init();

    loop {
        let player_move = PiecePlacement::ask_player().unwrap();

        //println!("You will attempt to place make move {:?}", player_move);

        game_state.place_piece(player_move).unwrap();

        game_state.check_board()
    }
}
