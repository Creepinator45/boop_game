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
    Empty,
    Piece(Piece),
}

#[derive(Debug)]
struct GameState {
    game_board: [[Cell; 6]; 6],
    turn_order: [Player; 2],
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
                }) = self.game_board[coordinate.0 + 1][coordinate.1]
                {
                    if let Cell::Piece(Piece {
                        owner: current_owner2,
                        size: _,
                    }) = self.game_board[coordinate.0 - 1][coordinate.1]
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
                }) = self.game_board[coordinate.0][coordinate.1 + 1]
                {
                    if let Cell::Piece(Piece {
                        owner: current_owner2,
                        size: _,
                    }) = self.game_board[coordinate.0][coordinate.1 - 1]
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
                }) = self.game_board[coordinate.0 + 1][coordinate.1 + 1]
                {
                    if let Cell::Piece(Piece {
                        owner: current_owner2,
                        size: _,
                    }) = self.game_board[coordinate.0 - 1][coordinate.1 - 1]
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
                }) = self.game_board[coordinate.0 + 1][coordinate.1 - 1]
                {
                    if let Cell::Piece(Piece {
                        owner: current_owner2,
                        size: _,
                    }) = self.game_board[coordinate.0 - 1][coordinate.1 + 1]
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
        for row_index in 1..self.game_board.len() - 1 {
            for column_index in 1..self.game_board[row_index].len() - 1 {
                let _ = dbg!(&self.check_cell((row_index, column_index)).unwrap());
            }
        }
    }

    fn place_piece(&mut self, coordinate: (usize, usize), size: Size) -> Result<(), &'static str> {
        let player_index = dbg!(self.turn_count % self.turn_order.len());

        if coordinate.0 > self.game_board.len() {
            return Result::Err("X Coordinate Out of Bounds");
        }
        if coordinate.1 > self.game_board[coordinate.0].len() {
            return Result::Err("Y Coordinate Out of Bounds");
        }
        if self.game_board[coordinate.0][coordinate.1] != Cell::Empty {
            return Result::Err("Placement Position Occupied");
        }
        if !self.turn_order[player_index].piece_pool.contains(&Piece {
            owner: player_index,
            size,
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
                        size,
                    }
                })
                .expect("needle not found"),
        );

        self.game_board[coordinate.0][coordinate.1] = Cell::Piece(Piece {
            owner: (player_index),
            size: (size),
        });

        match coordinate.0 {
            0 => {todo!()}
            6 => {todo!()}
            2 => {todo!()}
            5 => {todo!()}
            _ => match coordinate.1 {
                0 => {todo!()}
                6 => {todo!()}
                2 => {todo!()}
                5 => {todo!()}
                _ => {
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
                        if self.game_board[coordinate.0 - 1 + d.0][coordinate.1 - 1 + d.1]
                            == Cell::Empty
                        {
                            continue;
                        }
                        if self.game_board[coordinate.0 - 2 + d.0 * 2][coordinate.1 - 2 + d.1 * 2]
                            != Cell::Empty
                        {
                            continue;
                        }

                        self.game_board[coordinate.0 - 2 + d.0 * 2][coordinate.1 - 2 + d.1 * 2] =
                            self.game_board[coordinate.0 - 1 + d.0][coordinate.1 - 1 + d.1].clone();
                        self.game_board[coordinate.0 - 1 + d.0][coordinate.1 - 1 + d.1] =
                            Cell::Empty;
                    }
                }
            },
        }
        Result::Ok(())
    }
}

fn init() -> GameState {
    GameState {
        game_board: [
            [
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
            [
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
            [
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
            [
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
            [
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
            [
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
        ],
        turn_order: [
            Player {
                name: String::from("Player 1"),
                piece_pool: vec![
                    Piece {
                        owner: 0,
                        size: Size::Small,
                    },
                    Piece {
                        owner: 0,
                        size: Size::Small,
                    },
                    Piece {
                        owner: 0,
                        size: Size::Small,
                    },
                    Piece {
                        owner: 0,
                        size: Size::Small,
                    },
                    Piece {
                        owner: 0,
                        size: Size::Small,
                    },
                    Piece {
                        owner: 0,
                        size: Size::Small,
                    },
                    Piece {
                        owner: 0,
                        size: Size::Small,
                    },
                    Piece {
                        owner: 0,
                        size: Size::Small,
                    },
                ],
            },
            Player {
                name: String::from("Player 2"),
                piece_pool: vec![
                    Piece {
                        owner: 1,
                        size: Size::Small,
                    },
                    Piece {
                        owner: 1,
                        size: Size::Small,
                    },
                    Piece {
                        owner: 1,
                        size: Size::Small,
                    },
                    Piece {
                        owner: 1,
                        size: Size::Small,
                    },
                    Piece {
                        owner: 1,
                        size: Size::Small,
                    },
                    Piece {
                        owner: 1,
                        size: Size::Small,
                    },
                    Piece {
                        owner: 1,
                        size: Size::Small,
                    },
                    Piece {
                        owner: 1,
                        size: Size::Small,
                    },
                ],
            },
        ],
        turn_count: 0,
    }
}

fn main() {
    let mut game_state = init();

    loop {

        let mut position_input = String::new();

        println!("Position:");
        std::io::stdin()
            .read_line(&mut position_input)
            .expect("Failed to read line");

        let mut size_input = String::new();

        println!("Size:");
        std::io::stdin()
            .read_line(&mut size_input)
            .expect("Failed to read line");


        println!("You will attempt to place a {size_input} piece at {position_input}");

    }
}
