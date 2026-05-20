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
  has_moved: bool,
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

    Self {
      color,
      kind,
      value,
      has_moved: false,
    }
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

const ORTHOGONALS: [Pos; 4] = [Pos(0, -1), Pos(0, 1), Pos(-1, 0), Pos(1, 0)];
const DIAGONALS: [Pos; 4] = [Pos(-1, 1), Pos(1, -1), Pos(1, 1), Pos(-1, -1)];
const ALL_DIRECTIONS: [Pos; 8] = [
  Pos(0, -1),
  Pos(0, 1),
  Pos(-1, 0),
  Pos(1, 0),
  Pos(-1, 1),
  Pos(1, -1),
  Pos(1, 1),
  Pos(-1, -1),
];
const KNIGHT_JUMPS: [Pos; 8] = [
  Pos(1, 2),
  Pos(1, -2),
  Pos(-1, 2),
  Pos(-1, -2),
  Pos(2, 1),
  Pos(2, -1),
  Pos(-2, 1),
  Pos(-2, -1),
];

const WHITE_SQUARE_COLOR: Color = Color::from_hex(0x7C4C3E);
const DARK_SQUARE_COLOR: Color = Color::from_hex(0x512A2A);
const HIGHLIGHTED_SQUARE_COLOR: Color = Color::from_rgba(0xF0, 0xC0, 0x40, 128);
const MOVE_AVAILABLE_SQUARE_COLOR: Color = Color::from_rgba(0x5B, 0x8F, 0xA8, 128);
const SQUARE_SIZE: f32 = 100.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pos(i32, i32);

impl Pos {
  fn offset(self, difference: Pos) -> Option<usize> {
    let nc = self.0 + difference.0;
    let nr = self.1 + difference.1;

    if nc < 0 || nc > 7 || nr < 0 || nr > 7 {
      return None;
    }

    Some((nr * 8 + nc) as usize)
  }
}

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

  fn is_empty(&self) -> bool {
    self.piece.is_none()
  }

  fn is_enemy(&self, color: PieceColor) -> bool {
    matches!(self.piece, Some(p) if p.color != color)
  }
}

fn is_square_attacked(squares: &[Square; 64], target_idx: usize, by: PieceColor) -> bool {
  let target = Pos(target_idx as i32 % 8, target_idx as i32 / 8);

  let any_at = |offsets: &[Pos], kind: PieceKind| {
    offsets.iter().any(|&d| {
      target
        .offset(d)
        .is_some_and(|idx| matches!(squares[idx].piece, Some(p) if p.color == by && p.kind == kind))
    })
  };

  let any_ray = |dirs: &[Pos], k1: PieceKind, k2: PieceKind| {
    dirs.iter().any(|&dir| {
      for k in 1..8i32 {
        let Some(idx) = target.offset(Pos(dir.0 * k, dir.1 * k)) else {
          return false;
        };
        match squares[idx].piece {
          None => continue,
          Some(p) if p.color == by && (p.kind == k1 || p.kind == k2) => return true,
          Some(_) => return false,
        }
      }
      false
    })
  };

  let pawn_dir = match by {
    PieceColor::White => 1,
    PieceColor::Black => -1,
  };
  let pawn_offsets = [Pos(-1, -pawn_dir), Pos(1, -pawn_dir)];

  any_at(&KNIGHT_JUMPS, PieceKind::Knight)
    || any_at(&ALL_DIRECTIONS, PieceKind::King)
    || any_at(&pawn_offsets, PieceKind::Pawn)
    || any_ray(&ORTHOGONALS, PieceKind::Rook, PieceKind::Queen)
    || any_ray(&DIAGONALS, PieceKind::Bishop, PieceKind::Queen)
}

fn push_jump(
  squares: &[Square; 64],
  from: Pos,
  dir: Pos,
  color: PieceColor,
  out: &mut Vec<usize>,
) {
  if let Some(to) = from
    .offset(dir)
    .filter(|&t| squares[t].is_empty() || squares[t].is_enemy(color))
  {
    out.push(to);
  }
}

fn push_slide(
  squares: &[Square; 64],
  from: Pos,
  dir: Pos,
  color: PieceColor,
  out: &mut Vec<usize>,
) {
  for k in 1..8i32 {
    let Some(to) = from.offset(Pos(dir.0 * k, dir.1 * k)) else {
      break;
    };
    match squares[to].piece {
      Some(p) if p.color == color => break,
      Some(_) => {
        out.push(to);
        break;
      }
      None => out.push(to),
    }
  }
}

fn push_pawn(squares: &[Square; 64], from: Pos, color: PieceColor, out: &mut Vec<usize>) {
  let (dir, start_row) = match color {
    PieceColor::White => (1, 1),
    PieceColor::Black => (-1, 6),
  };

  if let Some(fwd) = from
    .offset(Pos(0, dir))
    .filter(|&t| squares[t].is_empty())
  {
    out.push(fwd);

    if from.1 == start_row
      && let Some(fwd2) = from
        .offset(Pos(0, dir * 2))
        .filter(|&t| squares[t].is_empty())
    {
      out.push(fwd2);
    }
  }

  for dc in [-1i32, 1] {
    if let Some(target) = from
      .offset(Pos(dc, dir))
      .filter(|&t| squares[t].is_enemy(color))
    {
      out.push(target);
    }
  }
}

fn push_castling(squares: &[Square; 64], from: Pos, color: PieceColor, out: &mut Vec<usize>) {
  let row: i32 = match color {
    PieceColor::White => 0,
    PieceColor::Black => 7,
  };

  if from != Pos(4, row) {
    return;
  }
  let king_idx = (row * 8 + 4) as usize;
  let Some(king) = squares[king_idx].piece else {
    return;
  };
  if king.has_moved || is_square_attacked(squares, king_idx, !color) {
    return;
  }

  let try_side =
    |out: &mut Vec<usize>, rook_col: i32, empty_cols: &[i32], pass_cols: &[i32], dest_col: i32| {
      let rook_idx = (row * 8 + rook_col) as usize;
      let Some(rook) = squares[rook_idx].piece else {
        return;
      };
      if rook.kind != PieceKind::Rook || rook.color != color || rook.has_moved {
        return;
      }
      for &c in empty_cols {
        if !squares[(row * 8 + c) as usize].is_empty() {
          return;
        }
      }
      for &c in pass_cols {
        if is_square_attacked(squares, (row * 8 + c) as usize, !color) {
          return;
        }
      }
      out.push((row * 8 + dest_col) as usize);
    };

  try_side(out, 7, &[5, 6], &[5, 6], 6);
  try_side(out, 0, &[1, 2, 3], &[2, 3], 2);
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
        self.apply_move(prev_index, square_index);
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
    let from = Pos(square_index as i32 % 8, square_index as i32 / 8);
    let color = self.turn_color;
    let mut candidates: Vec<usize> = Vec::new();

    match piece_kind {
      PieceKind::King => {
        for &d in ALL_DIRECTIONS.iter() {
          push_jump(&self.squares, from, d, color, &mut candidates);
        }
        push_castling(&self.squares, from, color, &mut candidates);
      }
      PieceKind::Knight => {
        for &d in KNIGHT_JUMPS.iter() {
          push_jump(&self.squares, from, d, color, &mut candidates);
        }
      }
      PieceKind::Queen => {
        for &d in ALL_DIRECTIONS.iter() {
          push_slide(&self.squares, from, d, color, &mut candidates);
        }
      }
      PieceKind::Rook => {
        for &d in ORTHOGONALS.iter() {
          push_slide(&self.squares, from, d, color, &mut candidates);
        }
      }
      PieceKind::Bishop => {
        for &d in DIAGONALS.iter() {
          push_slide(&self.squares, from, d, color, &mut candidates);
        }
      }
      PieceKind::Pawn => push_pawn(&self.squares, from, color, &mut candidates),
    }

    for to in candidates {
      if self.is_move_legal(square_index, to) {
        self.mark_available(to);
      }
    }
  }

  fn is_move_legal(&self, from: usize, to: usize) -> bool {
    let Some(piece) = self.squares[from].piece else {
      return false;
    };

    let mut sim = self.squares;
    sim[to].piece = Some(piece);
    sim[from].piece = None;

    let color = piece.color;
    let Some(king_idx) = sim
      .iter()
      .position(|s| matches!(s.piece, Some(p) if p.color == color && p.kind == PieceKind::King))
    else {
      return true;
    };

    !is_square_attacked(&sim, king_idx, !color)
  }

  fn apply_move(&mut self, from: usize, to: usize) {
    let mut piece = self.squares[from].piece;
    if let Some(p) = piece.as_mut() {
      p.has_moved = true;
    }
    self.squares[to].piece = piece;
    self.squares[from].piece = None;

    if let Some(p) = piece
      && p.kind == PieceKind::King
    {
      let from_col = (from % 8) as i32;
      let to_col = (to % 8) as i32;
      let row = from / 8;
      let (rook_from_col, rook_to_col) = match to_col - from_col {
        2 => (7usize, 5usize),
        -2 => (0usize, 3usize),
        _ => return,
      };
      let rook_from = row * 8 + rook_from_col;
      let rook_to = row * 8 + rook_to_col;
      let mut rook = self.squares[rook_from].piece;
      if let Some(r) = rook.as_mut() {
        r.has_moved = true;
      }
      self.squares[rook_to].piece = rook;
      self.squares[rook_from].piece = None;
    }
  }

  fn mark_available(&mut self, to: usize) {
    self.squares[to].move_available = true;
  }
}
