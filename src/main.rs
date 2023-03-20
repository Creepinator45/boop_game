use std::{fmt, num::ParseIntError};

//Error structures designed to mimic std::num::ParseIntError
#[derive(Debug, Clone, PartialEq, Eq)]
struct CheckCellError {
    kind: CellErrorKind,
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum CellErrorKind {
    CheckingOutOfBounds,
    OutOfBoundsX,
    OutOfBoundsY,
}
impl CheckCellError {
    fn __description(&self) -> &str {
        match self.kind {
            CellErrorKind::CheckingOutOfBounds => {
                "attempting to check out of bounds, something's wrong!"
            }
            CellErrorKind::OutOfBoundsX => "x value out of bounds",
            CellErrorKind::OutOfBoundsY => "y value out of bounds",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PlacePieceError {
    kind: PieceErrorKind,
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum PieceErrorKind {
    OutOfBoundsX,
    OutOfBoundsY,
    PositionOccupied,
    MissingPiece,
}
impl PlacePieceError {
    fn __description(&self) -> &str {
        match self.kind {
            PieceErrorKind::OutOfBoundsX => "x value out of bounds",
            PieceErrorKind::OutOfBoundsY => "y value out of bounds",
            PieceErrorKind::PositionOccupied => "attempting to place piece at accupied position",
            PieceErrorKind::MissingPiece => "attempting to place piece that is not in piece pool",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParseCoordinateError {
    kind: CoordinateErrorKind,
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum CoordinateErrorKind {
    Empty,
    InvalidFormat,
    ValueErrorX(ParseIntError),
    ValueErrorY(ParseIntError),
}
impl ParseCoordinateError {
    fn __description(&self) -> &str {
        match self.kind {
            CoordinateErrorKind::Empty => "cannot parse coordinate from empty string",
            CoordinateErrorKind::InvalidFormat => "invalid format, should be \"x,y\"",
            CoordinateErrorKind::ValueErrorX(_) => "problem parsing x value",
            CoordinateErrorKind::ValueErrorY(_) => "problem parsing y value",
        }
    }
}
impl fmt::Display for ParseCoordinateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.__description().fmt(f)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParseSizeError {
    kind: SizeErrorKind,
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum SizeErrorKind {
    Empty,
    UnknownValue,
}
impl ParseSizeError {
    fn __description(&self) -> &str {
        match self.kind {
            SizeErrorKind::Empty => "cannot parse size from empty string",
            SizeErrorKind::UnknownValue => {
                "unknown value. Valid sizes are \"s\",\"b\",\"small\", or \"big\""
            }
        }
    }
}
impl fmt::Display for ParseSizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.__description().fmt(f)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsePiecePlacementError {
    kind: PiecePlacementErrorKind,
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum PiecePlacementErrorKind {
    Empty,
    InvalidFormat,
    ValueErrorSize(ParseSizeError),
    ValueErrorCoordinate(ParseCoordinateError),
}
impl ParsePiecePlacementError {
    fn __description(&self) -> &str {
        match self.kind {
            PiecePlacementErrorKind::Empty => "cannot parse piece placement from empty string",
            PiecePlacementErrorKind::InvalidFormat => {
                "invalid format, should be \"size,coordinate\""
            }
            PiecePlacementErrorKind::ValueErrorSize(_) => "problem parsing size value",
            PiecePlacementErrorKind::ValueErrorCoordinate(_) => "problem parsing coordinate value",
        }
    }
}
impl fmt::Display for ParsePiecePlacementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.__description().fmt(f)
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Eq)]
enum Size {
    Small,
    Big,
}
impl std::str::FromStr for Size {
    type Err = ParseSizeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "" => Err(ParseSizeError {
                kind: SizeErrorKind::Empty,
            }),
            "s" | "small" => Ok(Size::Small),
            "b" | "big" => Ok(Size::Big),
            _ => Err(ParseSizeError {
                kind: SizeErrorKind::UnknownValue,
            }),
        }
    }
}

#[derive(Clone, PartialEq, Debug, Eq)]
struct Piece {
    owner: usize,
    size: Size,
}

#[derive(PartialEq, Debug)]
struct Player {
    name: String,
    piece_pool: Vec<Piece>,
}

#[derive(Clone, PartialEq, Debug, Eq)]
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

    fn check_cell(&self, coordinate: Coordinate) -> Result<Vec<ThreeInRow>, CheckCellError> {
        if coordinate.x > 7 || coordinate.x < 2 {
            return Result::Err(CheckCellError {
                kind: CellErrorKind::OutOfBoundsX,
            });
        }
        if coordinate.y > 7 || coordinate.x < 2 {
            return Result::Err(CheckCellError {
                kind: CellErrorKind::OutOfBoundsY,
            });
        }

        match &self.game_board[coordinate.x][coordinate.y] {
            Cell::OutOfBounds => Result::Err(CheckCellError {
                kind: CellErrorKind::CheckingOutOfBounds,
            }),
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
            1 => {
                if let Cell::Piece(Piece { owner, size: _ }) =
                    &self.game_board[constrained_matches[0].0.x][constrained_matches[0].0.y]
                {
                    if let Cell::Piece(Piece {
                        owner: _,
                        size: size0,
                    }) = self.game_board[constrained_matches[0].0.x][constrained_matches[0].0.y]
                    {
                        if let Cell::Piece(Piece {
                            owner: _,
                            size: size1,
                        }) =
                            self.game_board[constrained_matches[0].1.x][constrained_matches[0].1.y]
                        {
                            if let Cell::Piece(Piece {
                                owner: _,
                                size: size2,
                            }) = self.game_board[constrained_matches[0].2.x]
                                [constrained_matches[0].2.y]
                            {
                                if size0 == Size::Big && size1 == Size::Big && size2 == Size::Big {
                                    win(&self.turn_order[*owner].name)
                                }
                            }
                        }
                    }

                    self.turn_order[*owner].piece_pool.extend([
                        Piece {
                            owner: *owner,
                            size: Size::Big,
                        },
                        Piece {
                            owner: *owner,
                            size: Size::Big,
                        },
                        Piece {
                            owner: *owner,
                            size: Size::Big,
                        },
                    ]);

                    self.game_board[constrained_matches[0].0.x][constrained_matches[0].0.y] =
                        Cell::Empty;
                    self.game_board[constrained_matches[0].1.x][constrained_matches[0].1.y] =
                        Cell::Empty;
                    self.game_board[constrained_matches[0].2.x][constrained_matches[0].2.y] =
                        Cell::Empty;
                } else {
                    panic!("match coordinate not a piece")
                }
            }
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

    fn place_piece(&mut self, piece_placement: PiecePlacement) -> Result<(), PlacePieceError> {
        let player_index = self.turn_count % self.turn_order.len();

        let coordinate = Coordinate {
            x: piece_placement.coordinate.x + 2,
            y: piece_placement.coordinate.y + 2,
        };

        if coordinate.x > 7 || coordinate.x < 2 {
            return Result::Err(PlacePieceError {
                kind: PieceErrorKind::OutOfBoundsX,
            });
        }
        if coordinate.y > 7 || coordinate.y < 2 {
            return Result::Err(PlacePieceError {
                kind: PieceErrorKind::OutOfBoundsY,
            });
        }

        if self.game_board[coordinate.x][coordinate.y] != Cell::Empty {
            return Result::Err(PlacePieceError {
                kind: PieceErrorKind::PositionOccupied,
            });
        }
        if !self.turn_order[player_index].piece_pool.contains(&Piece {
            owner: player_index,
            size: piece_placement.size,
        }) {
            return Result::Err(PlacePieceError {
                kind: PieceErrorKind::MissingPiece,
            });
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
            match &self.game_board[coordinate.x - 1 + d.x][coordinate.y - 1 + d.y] {
                Cell::Piece(piece)
                    if piece.size == Size::Big && piece_placement.size == Size::Small =>
                {
                    continue
                }
                Cell::Piece(piece) => {
                    match self.game_board[coordinate.x - 2 + d.x * 2][coordinate.y - 2 + d.y * 2] {
                        Cell::Piece(_) => continue,
                        Cell::OutOfBounds => {
                            self.turn_order[piece.owner].piece_pool.push(piece.clone());
                            self.game_board[coordinate.x - 1 + d.x][coordinate.y - 1 + d.y] =
                                Cell::Empty;
                        }
                        Cell::Empty => {
                            self.game_board[coordinate.x - 2 + d.x * 2]
                                [coordinate.y - 2 + d.y * 2] = self.game_board
                                [coordinate.x - 1 + d.x][coordinate.y - 1 + d.y]
                                .clone();

                            self.game_board[coordinate.x - 1 + d.x][coordinate.y - 1 + d.y] =
                                Cell::Empty;
                        }
                    }
                }
                _ => continue,
            }
        }
        Result::Ok(())
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Coordinate {
    x: usize,
    y: usize,
}
impl std::str::FromStr for Coordinate {
    type Err = ParseCoordinateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or(ParseCoordinateError {
            kind: CoordinateErrorKind::InvalidFormat,
        })?;
        let x_fromstr = match x.parse::<usize>() {
            Ok(x) => x,
            Err(error) => {
                return Err(ParseCoordinateError {
                    kind: CoordinateErrorKind::ValueErrorX(error),
                })
            }
        };
        let y_fromstr = match y.parse::<usize>() {
            Ok(y) => y,
            Err(error) => {
                return Err(ParseCoordinateError {
                    kind: CoordinateErrorKind::ValueErrorY(error),
                })
            }
        };

        Ok(Coordinate {
            x: x_fromstr,
            y: y_fromstr,
        })
    }
}

#[derive(Debug)]
struct PiecePlacement {
    coordinate: Coordinate,
    size: Size,
}
impl std::str::FromStr for PiecePlacement {
    type Err = ParsePiecePlacementError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (size, coordinate) = s.split_once(",").ok_or(ParsePiecePlacementError {
            kind: PiecePlacementErrorKind::InvalidFormat,
        })?;
        let size_fromstr = match size.parse::<Size>() {
            Ok(size) => size,
            Err(error) => {
                return Err(ParsePiecePlacementError {
                    kind: PiecePlacementErrorKind::ValueErrorSize(error),
                })
            }
        };
        let coordinate_fromstr = match coordinate.parse::<Coordinate>() {
            Ok(coordinate) => coordinate,
            Err(error) => {
                return Err(ParsePiecePlacementError {
                    kind: PiecePlacementErrorKind::ValueErrorCoordinate(error),
                })
            }
        };

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

#[derive(Debug, PartialEq, Clone)]
struct ThreeInRow(Coordinate, Coordinate, Coordinate);

fn win(winner: &str) {
    println!("{winner} wins!")
}
fn main() {
    let mut game_state = GameState::init();
    game_state.display();

    loop {
        let player_move = match PiecePlacement::ask_player(
            &game_state.turn_order[game_state.turn_count % 2].name,
        ) {
            Ok(ok) => ok,
            Err(ParsePiecePlacementError { kind }) => {
                match kind {
                    PiecePlacementErrorKind::Empty => panic!(),
                    PiecePlacementErrorKind::InvalidFormat => {
                        println!("Invalid format: should be \"size,x,y\"")
                    }
                    PiecePlacementErrorKind::ValueErrorSize(_) => {
                        println!("Invalid Size");
                    }
                    PiecePlacementErrorKind::ValueErrorCoordinate(_) => {
                        println!("Invalid Coordinate");
                    }
                };
                continue;
            }
        };
        match game_state.place_piece(player_move) {
            Ok(_) => (),
            Err(PlacePieceError { kind }) => {
                match kind {
                    PieceErrorKind::OutOfBoundsX => println!("Out of Bounds"),
                    PieceErrorKind::OutOfBoundsY => println!("Out of Bounds"),
                    PieceErrorKind::PositionOccupied => println!("Occupied Position"),
                    PieceErrorKind::MissingPiece => println!("Piece Unavailable"),
                }
                continue;
            }
        }
        game_state.check_board(None);
        game_state.turn_count += 1;

        game_state.display();
    }
}
