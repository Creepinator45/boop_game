#[derive(Debug)]
struct ParseCoordinateError;
#[derive(Debug)]
struct ParseSizeError;
#[derive(Debug)]
enum ParsePiecePlacementError {
    ParsePiecePlacementError,
    ParseCoordinateError(ParseCoordinateError),
    ParseSizeError(ParseSizeError),
}

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

#[derive(Debug, PartialEq, Clone, Copy)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct PiecePlacement {
    coordinate: Coordinate,
    size: Size,
}

#[derive(Debug, PartialEq, Clone)]
struct ThreeInRow(Coordinate, Coordinate, Coordinate);

impl std::str::FromStr for Size {
    type Err = ParseSizeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "s" | "small" => Ok(Size::Small),
            "b" | "big" => Ok(Size::Big),
            _ => Err(ParseSizeError),
        }
    }
}
impl std::str::FromStr for Coordinate {
    type Err = ParseCoordinateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or(ParseCoordinateError)?;
        let x_fromstr = x.parse::<usize>().map_err(|_| ParseCoordinateError)?;
        let y_fromstr = y.parse::<usize>().map_err(|_| ParseCoordinateError)?;

        Ok(Coordinate {
            x: x_fromstr,
            y: y_fromstr,
        })
    }
}
impl std::str::FromStr for PiecePlacement {
    type Err = ParsePiecePlacementError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (size, coordinate) = s
            .split_once(",")
            .ok_or(ParsePiecePlacementError::ParseSizeError(ParseSizeError))?;
        let size_fromstr = size
            .parse::<Size>()
            .map_err(|_| ParsePiecePlacementError::ParseCoordinateError(ParseCoordinateError))?;
        let coordinate_fromstr = coordinate
            .parse::<Coordinate>()
            .map_err(|_| ParsePiecePlacementError::ParsePiecePlacementError)?;

        Ok(PiecePlacement {
            coordinate: coordinate_fromstr,
            size: size_fromstr,
        })
    }
}
impl PiecePlacement {
    fn ask_player(player_name: &str) -> Result<PiecePlacement, ParsePiecePlacementError> {
        let mut input = String::new();

        println!("{} to move:", player_name);
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let piece_placement = input.trim().parse()?;

        Result::Ok(piece_placement)
    }
}

impl GameState {
    #[rustfmt::skip]
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

    fn display(&self) {
        let mut player1pieces = String::new();
        for p in &self.turn_order[0].piece_pool {
            match p {
                Piece { owner: _, size } if size == &Size::Small => player1pieces += "x",
                Piece { owner: _, size } if size == &Size::Big => player1pieces += "X",
                _ => panic!(),
            }
        }
        println!("{}", player1pieces);

        for y in 2..8 {
            let mut row_cells = String::new();

            for x in 2..8 {
                match self.game_board[x][y] {
                    Cell::Empty => row_cells += "_ ",
                    Cell::Piece(Piece { owner, size }) if owner == 0 && size == Size::Small => {
                        row_cells += "x "
                    }
                    Cell::Piece(Piece { owner, size }) if owner == 0 && size == Size::Big => {
                        row_cells += "X "
                    }
                    Cell::Piece(Piece { owner, size }) if owner == 1 && size == Size::Small => {
                        row_cells += "o "
                    }
                    Cell::Piece(Piece { owner, size }) if owner == 1 && size == Size::Big => {
                        row_cells += "O "
                    }
                    _ => panic!(),
                }
            }

            println!("{}", row_cells);
        }

        let mut player2pieces = String::new();
        for p in &self.turn_order[1].piece_pool {
            match p {
                Piece { owner: _, size } if size == &Size::Small => player2pieces += "o",
                Piece { owner: _, size } if size == &Size::Big => player2pieces += "O",
                _ => panic!(),
            }
        }
        println!("{}", player2pieces);
    }

    fn check_cell(&self, coordinate: Coordinate) -> Result<Vec<ThreeInRow>, &'static str> {
        if coordinate.x > 7 || coordinate.x < 2 {
            return Result::Err("X Coordinate Out of Bounds");
        }
        if coordinate.y > 7 || coordinate.x < 2 {
            return Result::Err("Y Coordinate Out of Bounds");
        }

        match &self.game_board[coordinate.x][coordinate.y] {
            Cell::OutOfBounds => Result::Err("Checking Out of Bounds"),
            Cell::Empty => Result::Ok(Vec::new()),
            Cell::Piece(Piece {
                owner: current_owner,
                size: _,
            }) => {
                let mut matches: Vec<ThreeInRow> = Vec::new();

                let dirs = [
                    Coordinate { x: 0, y: 0 },
                    Coordinate { x: 0, y: 1 },
                    Coordinate { x: 0, y: 2 },
                    Coordinate { x: 1, y: 2 },
                ];

                for d in dirs {
                    if let Cell::Piece(Piece {
                        owner: current_owner1,
                        size: _,
                    }) = self.game_board[coordinate.x - 1 + d.x][coordinate.y - 1 + d.y]
                    {
                        if let Cell::Piece(Piece {
                            owner: current_owner2,
                            size: _,
                        }) = self.game_board[coordinate.x + 1 - d.x][coordinate.y + 1 - d.y]
                        {
                            if current_owner == &current_owner1
                                && &current_owner1 == &current_owner2
                            {
                                matches.push(ThreeInRow(
                                    Coordinate {
                                        x: coordinate.x - 1 + d.x,
                                        y: coordinate.y - 1 + d.y,
                                    },
                                    Coordinate {
                                        x: coordinate.x,
                                        y: coordinate.y,
                                    },
                                    Coordinate {
                                        x: coordinate.x + 1 - d.x,
                                        y: coordinate.y + 1 - d.y,
                                    },
                                ));
                            }
                        }
                    }
                }
                Result::Ok(matches)
            }
        }
    }

    fn check_board(&mut self, constraining_coordinate: Option<Coordinate>) {
        let mut matches = Vec::new();
        for row_index in 2..8 {
            for column_index in 2..8 {
                matches.extend(
                    self.check_cell(Coordinate {
                        x: row_index,
                        y: column_index,
                    })
                    .unwrap(),
                )
            }
        }

        let mut constrained_matches: Vec<ThreeInRow> = Vec::new();
        match constraining_coordinate {
            None => constrained_matches = matches,
            Some(constraining_coordinate) => {
                for m in matches {
                    let ThreeInRow(c1, c2, c3) = m;
                    if c1 == constraining_coordinate
                        || c2 == constraining_coordinate
                        || c3 == constraining_coordinate
                    {
                        constrained_matches.push(m.clone())
                    }
                }
            }
        }
        match constrained_matches.len() {
            0 => return,
            1 => match &self.game_board[constrained_matches[0].0.x][constrained_matches[0].0.y] {
                Cell::Piece(piece) => {
                    self.turn_order[piece.owner].piece_pool.extend([
                        Piece {
                            owner: piece.owner,
                            size: Size::Big,
                        },
                        Piece {
                            owner: piece.owner,
                            size: Size::Big,
                        },
                        Piece {
                            owner: piece.owner,
                            size: Size::Big,
                        },
                    ]);

                    self.game_board[constrained_matches[0].0.x][constrained_matches[0].0.y] =
                        Cell::Empty;
                    self.game_board[constrained_matches[0].1.x][constrained_matches[0].1.y] =
                        Cell::Empty;
                    self.game_board[constrained_matches[0].2.x][constrained_matches[0].2.y] =
                        Cell::Empty;
                }
                _ => panic!("match coordinate not a piece"),
            },
            _ => {
                self.display();
                let mut position_input = String::new();

                println!("Select constraining Piece");
                std::io::stdin()
                    .read_line(&mut position_input)
                    .expect("Failed to read line");

                let constraining_coordinate: Coordinate = match position_input.trim().parse() {
                    Result::Ok(ok) => ok,
                    Result::Err(_) => {
                        println!("Please input valid position");
                        Coordinate { x: 99, y: 99 }
                    }
                };
                let constraining_coordinate = Coordinate {
                    x: constraining_coordinate.x + 2,
                    y: constraining_coordinate.y + 2,
                };
                self.check_board(Some(constraining_coordinate));
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
        if coordinate.y > 7 || coordinate.y < 2 {
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

        //Bounce Adjacent Pieces
        let dirs = [
            Coordinate { x: 0, y: 0 },
            Coordinate { x: 0, y: 1 },
            Coordinate { x: 0, y: 2 },
            Coordinate { x: 1, y: 2 },
            Coordinate { x: 2, y: 2 },
            Coordinate { x: 2, y: 1 },
            Coordinate { x: 2, y: 0 },
            Coordinate { x: 1, y: 0 },
        ];
        for d in dirs {
            if let Cell::Piece(piece) =
                &self.game_board[coordinate.x - 1 + d.x][coordinate.y - 1 + d.y]
            {
                match self.game_board[coordinate.x - 2 + d.x * 2][coordinate.y - 2 + d.y * 2] {
                    Cell::Piece(_) => continue,
                    Cell::OutOfBounds => {
                        self.turn_order[piece.owner].piece_pool.push(piece.clone());
                        self.game_board[coordinate.x - 1 + d.x][coordinate.y - 1 + d.y] =
                            Cell::Empty;
                    }
                    Cell::Empty => {
                        self.game_board[coordinate.x - 2 + d.x * 2][coordinate.y - 2 + d.y * 2] =
                            self.game_board[coordinate.x - 1 + d.x][coordinate.y - 1 + d.y].clone();

                        self.game_board[coordinate.x - 1 + d.x][coordinate.y - 1 + d.y] =
                            Cell::Empty;
                    }
                }
            }
        }
        Result::Ok(())
    }
}

fn main() {
    let mut game_state = GameState::init();
    game_state.display();

    loop {
        let player_move =
            PiecePlacement::ask_player(&game_state.turn_order[game_state.turn_count % 2].name)
                .unwrap();
        println!("You will attempt to make move {:?}", player_move);

        game_state.place_piece(player_move).unwrap();
        game_state.check_board(None);
        game_state.turn_count += 1;

        game_state.display();
    }
}
