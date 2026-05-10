use std::array::from_fn;

use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum File {
  A,
  B,
  C,
  D,
  E,
  F,
  G,
  H,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rank {
  One = 1,
  Two = 2,
  Three = 3,
  Four = 4,
  Five = 5,
  Six = 6,
  Seven = 7,
  Eight = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceColor {
  White,
  Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceKind {
  King,
  Queen,
  Bishop,
  Rook,
  Knight,
  Pawn,
}

const FILES: [File; 8] = [
  File::A,
  File::B,
  File::C,
  File::D,
  File::E,
  File::F,
  File::G,
  File::H,
];

const RANKS: [Rank; 8] = [
  Rank::One,
  Rank::Two,
  Rank::Three,
  Rank::Four,
  Rank::Five,
  Rank::Six,
  Rank::Seven,
  Rank::Eight,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Piece {
  color: PieceColor,
  kind: PieceKind,
  value: i32,
}

impl Piece {
  pub fn new(color: PieceColor, kind: PieceKind) -> Self {
    let value = match kind {
      PieceKind::King => 0,
      PieceKind::Queen => 9,
      PieceKind::Bishop => 3,
      PieceKind::Rook => 5,
      PieceKind::Knight => 3,
      PieceKind::Pawn => 1,
    };

    Self { color, kind, value }
  }

  pub fn new_starting(file: File, rank: Rank) -> Option<Self> {
    match rank {
      Rank::Two => Some(Self::new(PieceColor::White, PieceKind::Pawn)),
      Rank::Seven => Some(Self::new(PieceColor::Black, PieceKind::Pawn)),
      Rank::One | Rank::Eight => {
        let color = if rank == Rank::One {
          PieceColor::White
        } else {
          PieceColor::Black
        };

        let kind = match file {
          File::A | File::H => PieceKind::Rook,
          File::B | File::G => PieceKind::Knight,
          File::C | File::F => PieceKind::Bishop,
          File::D => PieceKind::Queen,
          File::E => PieceKind::King,
        };

        Some(Self::new(color, kind))
      }
      _ => None,
    }
  }
}

const WHITE_SQUARE_COLOR: Color = Color::from_hex(0x7C4C3E);
const DARK_SQUARE_COLOR: Color = Color::from_hex(0x512A2A);
const SQUARE_SIZE: f32 = 104.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Square {
  piece: Option<Piece>,
  file: File,
  rank: Rank,
}

impl Square {
  pub fn render(&self, i: i32) {
    let col = i % 8;
    let row = i / 8;

    let color = if (col + row) % 2 == 0 {
      WHITE_SQUARE_COLOR
    } else {
      DARK_SQUARE_COLOR
    };

    let offset_x = (screen_width() - 8.0 * SQUARE_SIZE) / 2.0;
    let offset_y = (screen_height() - 8.0 * SQUARE_SIZE) / 2.0;

    draw_rectangle(
      offset_x + col as f32 * SQUARE_SIZE,
      offset_y + row as f32 * SQUARE_SIZE,
      SQUARE_SIZE,
      SQUARE_SIZE,
      color,
    );
  }
}

pub struct Board {
  pub squares: [Square; 64],
  pub turn_color: PieceColor,
}

impl Board {
  pub fn new() -> Self {
    let squares: [Square; 64] = from_fn(|i| {
      let file = FILES[i % 8];
      let rank = RANKS[i / 8];

      Square {
        file,
        rank,
        piece: Piece::new_starting(file, rank),
      }
    });

    Self {
      squares,
      turn_color: PieceColor::White,
    }
  }
}
