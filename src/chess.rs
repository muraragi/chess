use std::{array::from_fn, fmt::Display, ops::Not};

use macroquad::prelude::*;

use crate::resources::Resources;

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

impl Not for PieceColor {
  type Output = Self;
  fn not(self) -> Self::Output {
    match self {
      PieceColor::White => PieceColor::Black,
      PieceColor::Black => PieceColor::White,
    }
  }
}

impl Display for PieceColor {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      PieceColor::White => write!(f, "White"),
      PieceColor::Black => write!(f, "Black"),
    }
  }
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

fn calculate_board_offset() -> Vec2 {
  vec2(
    (screen_width() - 8.0 * SQUARE_SIZE) / 2.0,
    (screen_height() - 8.0 * SQUARE_SIZE) / 2.0,
  )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Piece {
  color: PieceColor,
  kind: PieceKind,
  value: i32,
}

const PIECE_SPRITE_SIZE: f32 = 80.0;

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

  pub fn render(&self, square_pos: Vec2, resources: &Resources) {
    let pick_piece_color = |b, w| {
      if self.color == PieceColor::Black {
        b
      } else {
        w
      }
    };

    let sprite = match &self.kind {
      PieceKind::King => pick_piece_color(&resources.b_king, &resources.w_king),
      PieceKind::Queen => pick_piece_color(&resources.b_queen, &resources.w_queen),
      PieceKind::Bishop => pick_piece_color(&resources.b_bishop, &resources.w_bishop),
      PieceKind::Rook => pick_piece_color(&resources.b_rook, &resources.w_rook),
      PieceKind::Knight => pick_piece_color(&resources.b_knight, &resources.w_knight),
      PieceKind::Pawn => pick_piece_color(&resources.b_pawn, &resources.w_pawn),
    };

    let aspect = sprite.width() / sprite.height();
    let h = PIECE_SPRITE_SIZE;
    let offset = (SQUARE_SIZE - PIECE_SPRITE_SIZE) / 2.0;

    let offset_x_error = match &self.kind {
      PieceKind::Pawn => 8.0,
      PieceKind::King => 2.0,
      PieceKind::Queen => -2.0,
      _ => 0.0,
    };

    draw_texture_ex(
      sprite,
      square_pos.x + offset + offset_x_error,
      square_pos.y + offset,
      WHITE,
      DrawTextureParams {
        dest_size: Some(vec2(h * aspect, h)),
        ..Default::default()
      },
    );
  }
}

const ORTHOGONALS: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];
const DIAGONALS: [(i32, i32); 4] = [(-1, 1), (1, -1), (1, 1), (-1, -1)];
const ALL_DIRECTIONS: [(i32, i32); 8] = [
  (0, -1),
  (0, 1),
  (-1, 0),
  (1, 0),
  (-1, 1),
  (1, -1),
  (1, 1),
  (-1, -1),
];
const KNIGHT_JUMPS: [(i32, i32); 8] = [
  (1, 2),
  (1, -2),
  (-1, 2),
  (-1, -2),
  (2, 1),
  (2, -1),
  (-2, 1),
  (-2, -1),
];

const WHITE_SQUARE_COLOR: Color = Color::from_hex(0x7C4C3E);
const DARK_SQUARE_COLOR: Color = Color::from_hex(0x512A2A);
const HIGHLIGHTED_SQUARE_COLOR: Color = Color::from_rgba(0xF0, 0xC0, 0x40, 128);
const MOVE_AVAILABLE_SQUARE_COLOR: Color = Color::from_rgba(0x5B, 0x8F, 0xA8, 128);
const SQUARE_SIZE: f32 = 100.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Square {
  piece: Option<Piece>,
  file: File,
  rank: Rank,
  selected: bool,
  move_available: bool,
}

impl Square {
  pub fn render(&self, i: i32, resources: &Resources) {
    let col = i % 8;
    let row = i / 8;

    let color = if (col + row) % 2 == 0 {
      WHITE_SQUARE_COLOR
    } else {
      DARK_SQUARE_COLOR
    };

    let board_offset = calculate_board_offset();

    let square_pos = vec2(
      board_offset.x + col as f32 * SQUARE_SIZE,
      board_offset.y + row as f32 * SQUARE_SIZE,
    );

    draw_rectangle(square_pos.x, square_pos.y, SQUARE_SIZE, SQUARE_SIZE, color);
    if self.selected {
      draw_rectangle(
        square_pos.x,
        square_pos.y,
        SQUARE_SIZE,
        SQUARE_SIZE,
        HIGHLIGHTED_SQUARE_COLOR,
      );
    }
    if self.move_available {
      draw_rectangle(
        square_pos.x,
        square_pos.y,
        SQUARE_SIZE,
        SQUARE_SIZE,
        MOVE_AVAILABLE_SQUARE_COLOR,
      );
    }
    if let Some(piece) = &self.piece {
      piece.render(square_pos, resources);
    }
  }
}

pub struct Board {
  pub squares: [Square; 64],
  pub turn_color: PieceColor,
  pub is_move_mode: bool,
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
        selected: false,
        move_available: false,
      }
    });

    Self {
      squares,
      turn_color: PieceColor::White,
      is_move_mode: false,
    }
  }

  pub fn draw_info(&self) {
    let current_turn = format!("Current turn - {}", self.turn_color);
    draw_text(&current_turn, 16.0, 32.0, 32.0, WHITE);
  }

  pub fn handle_click(&mut self) {
    let mouse_pos = mouse_position();
    let board_offset = calculate_board_offset();
    let relative_mouse_pos = vec2(mouse_pos.0 - board_offset.x, mouse_pos.1 - board_offset.y);

    if relative_mouse_pos.x <= 0.0 || relative_mouse_pos.y <= 0.0 {
      return;
    }

    let col = ((mouse_pos.0 - board_offset.x) / SQUARE_SIZE) as usize;
    let row = ((mouse_pos.1 - board_offset.y) / SQUARE_SIZE) as usize;

    if col < 8 && row < 8 {
      let square_index = row * 8 + col;
      self.select_square(square_index);
    }
  }

  fn select_square(&mut self, square_index: usize) {
    if self.is_move_mode {
      if let Some(prev_index) = self.squares.iter().position(|s| s.selected)
        && self.squares[square_index].move_available
      {
        let prev_piece = self.squares[prev_index].piece;
        self.squares[square_index].piece = prev_piece;
        self.squares[prev_index].piece = None;
        self.switch_turn();
      }

      self.is_move_mode = false;
      self.clear_selections();
    } else {
      if let Some(piece) = self.squares[square_index].piece
        && piece.color == self.turn_color
      {
        self.clear_selections();
        self.is_move_mode = true;
        self.squares[square_index].selected = true;
        self.calculate_available_moves(square_index, piece.kind);
      }
    }
  }

  fn clear_selections(&mut self) {
    self.squares.iter_mut().for_each(|square| {
      square.selected = false;
      square.move_available = false;
    });
  }

  fn switch_turn(&mut self) {
    self.turn_color = !self.turn_color;
  }

  fn calculate_available_moves(&mut self, square_index: usize, piece_kind: PieceKind) {
    let from = (square_index as i32 % 8, square_index as i32 / 8);

    match piece_kind {
      PieceKind::King => ALL_DIRECTIONS
        .iter()
        .for_each(|&d| self.mark_available_jump(from, d)),
      PieceKind::Knight => KNIGHT_JUMPS
        .iter()
        .for_each(|&d| self.mark_available_jump(from, d)),
      PieceKind::Queen => ALL_DIRECTIONS.iter().for_each(|&d| self.slide(from, d)),
      PieceKind::Rook => ORTHOGONALS.iter().for_each(|&d| self.slide(from, d)),
      PieceKind::Bishop => DIAGONALS.iter().for_each(|&d| self.slide(from, d)),
      PieceKind::Pawn => self.mark_available_pawn_moves(from),
    }
  }

  fn mark_available_pawn_moves(&mut self, from: (i32, i32)) {
    let (dir, start_row) = match self.turn_color {
      PieceColor::White => (1, 1),
      PieceColor::Black => (-1, 6),
    };

    if let Some(fwd) = Self::offset(from, (0, dir)).filter(|&t| self.is_empty(t)) {
      self.mark_available(fwd);

      if from.1 == start_row
        && let Some(fwd2) = Self::offset(from, (0, dir * 2)).filter(|&t| self.is_empty(t))
      {
        self.mark_available(fwd2);
      }
    }

    for dc in [-1i32, 1] {
      if let Some(target) = Self::offset(from, (dc, dir)).filter(|&t| self.is_enemy(t)) {
        self.mark_available(target);
      }
    }
  }

  fn mark_available_jump(&mut self, from: (i32, i32), dir: (i32, i32)) {
    if let Some(to) = Self::offset(from, dir).filter(|&t| self.is_enemy(t)) {
      self.mark_available(to);
    }
  }

  fn slide(&mut self, from: (i32, i32), dir: (i32, i32)) {
    for k in 1..8i32 {
      let Some(to) = Self::offset(from, (dir.0 * k, dir.1 * k)) else {
        break;
      };

      match self.squares[to].piece {
        Some(p) if p.color == self.turn_color => break,
        Some(_) => {
          self.mark_available(to);
          break;
        }
        None => self.mark_available(to),
      }
    }
  }

  fn mark_available(&mut self, to: usize) {
    self.squares[to].move_available = true;
  }

  fn is_empty(&self, to: usize) -> bool {
    self.squares[to].piece.is_none()
  }

  fn is_enemy(&self, to: usize) -> bool {
    matches!(self.squares[to].piece, Some(p) if p.color != self.turn_color)
  }

  fn offset((col, row): (i32, i32), (dc, dr): (i32, i32)) -> Option<usize> {
    let nc = col + dc;
    let nr = row + dr;

    if nc < 0 || nc > 7 || nr < 0 || nr > 7 {
      return None;
    }

    Some((nr * 8 + nc) as usize)
  }
}
