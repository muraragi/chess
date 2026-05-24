use std::{fmt::Display, ops::Not};

use macroquad::prelude::*;

use crate::resources::Resources;

const PIECE_SPRITE_SIZE: f32 = 80.0;
const WHITE_SQUARE_COLOR: Color = Color::from_hex(0x7C4C3E);
const DARK_SQUARE_COLOR: Color = Color::from_hex(0x512A2A);
const HIGHLIGHTED_SQUARE_COLOR: Color = Color::from_rgba(0xF0, 0xC0, 0x40, 128);
const MOVE_AVAILABLE_SQUARE_COLOR: Color = Color::from_rgba(0x5B, 0x8F, 0xA8, 128);
const SQUARE_SIZE: f32 = 100.0;

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
struct Pos(i32, i32);

impl Pos {
  fn offset(self, offset: Pos) -> Option<Self> {
    let pos = Self(self.0 + offset.0, self.1 + offset.1);

    if pos.is_on_board() { Some(pos) } else { None }
  }

  fn get_coordinates(self) -> Vec2 {
    let board_offset = calculate_board_offset();

    vec2(
      board_offset.x + self.0 as f32 * SQUARE_SIZE,
      board_offset.y + self.1 as f32 * SQUARE_SIZE,
    )
  }

  fn from_coords(coords: (f32, f32)) -> Option<Self> {
    let board_offset = calculate_board_offset();
    let col = ((coords.0 - board_offset.x) / SQUARE_SIZE).floor() as i32;
    let row = ((coords.1 - board_offset.y) / SQUARE_SIZE).floor() as i32;

    if col < 0 || col > 7 || row < 0 || row > 7 {
      return None;
    }

    Some(Self(col, row))
  }

  fn is_on_board(self) -> bool {
    self.0 >= 0 && self.0 <= 7 && self.1 >= 0 && self.1 <= 7
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct Piece {
  color: PieceColor,
  kind: PieceKind,
  has_moved: bool,
  position: Option<Pos>,
  moves: Vec<Pos>,
  selected: bool,
}

impl Piece {
  pub fn new(color: PieceColor, kind: PieceKind, position: Pos) -> Self {
    Self {
      color,
      kind,
      has_moved: false,
      position: Some(position),
      moves: vec![],
      selected: false,
    }
  }

  pub fn render(&self, resources: &Resources) {
    let Some(position) = self.position else {
      return;
    };

    let board_offset = calculate_board_offset();
    let square_pos = vec2(
      board_offset.x + position.0 as f32 * SQUARE_SIZE,
      board_offset.y + position.1 as f32 * SQUARE_SIZE,
    );

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

fn calculate_board_offset() -> Vec2 {
  vec2(
    (screen_width() - 8.0 * SQUARE_SIZE) / 2.0,
    (screen_height() - 8.0 * SQUARE_SIZE) / 2.0,
  )
}

#[derive(Clone)]
pub struct Board {
  pieces: Vec<Piece>,
  pub turn_color: PieceColor,
  pub is_move_mode: bool,
}

impl Board {
  pub fn new() -> Self {
    let back_rank = [
      PieceKind::Rook,
      PieceKind::Knight,
      PieceKind::Bishop,
      PieceKind::Queen,
      PieceKind::King,
      PieceKind::Bishop,
      PieceKind::Knight,
      PieceKind::Rook,
    ];
    let mut pieces = Vec::with_capacity(32);

    for col in 0..8 {
      pieces.push(Piece::new(
        PieceColor::Black,
        back_rank[col],
        Pos(col as i32, 0),
      ));
      pieces.push(Piece::new(
        PieceColor::Black,
        PieceKind::Pawn,
        Pos(col as i32, 1),
      ));
      pieces.push(Piece::new(
        PieceColor::White,
        PieceKind::Pawn,
        Pos(col as i32, 6),
      ));
      pieces.push(Piece::new(
        PieceColor::White,
        back_rank[col],
        Pos(col as i32, 7),
      ));
    }

    let mut board = Self {
      pieces,
      turn_color: PieceColor::White,
      is_move_mode: false,
    };

    board.generate_piece_moves();
    board
  }

  pub fn game_started(&self) -> bool {
    self.pieces.iter().any(|piece| piece.has_moved)
  }

  pub fn render(&self, resources: &Resources) {
    let board_offset = calculate_board_offset();

    for row in 0..8 {
      for col in 0..8 {
        let color = if (col + row) % 2 == 0 {
          WHITE_SQUARE_COLOR
        } else {
          DARK_SQUARE_COLOR
        };
        let square_pos = vec2(
          board_offset.x + col as f32 * SQUARE_SIZE,
          board_offset.y + row as f32 * SQUARE_SIZE,
        );

        draw_rectangle(square_pos.x, square_pos.y, SQUARE_SIZE, SQUARE_SIZE, color);
      }
    }

    for piece in &self.pieces {
      if let Some(pos) = piece.position
        && piece.selected
      {
        let piece_coords = pos.get_coordinates();
        draw_rectangle(
          piece_coords.x,
          piece_coords.y,
          SQUARE_SIZE,
          SQUARE_SIZE,
          HIGHLIGHTED_SQUARE_COLOR,
        );

        for move_square_pos in &piece.moves {
          let square_coords = move_square_pos.get_coordinates();
          draw_rectangle(
            square_coords.x,
            square_coords.y,
            SQUARE_SIZE,
            SQUARE_SIZE,
            MOVE_AVAILABLE_SQUARE_COLOR,
          );
        }
      }

      piece.render(resources);
    }
  }

  pub fn draw_info(&self) {
    let current_turn = format!("Current turn - {}", self.turn_color);
    draw_text(&current_turn, 16.0, 32.0, 32.0, WHITE);
  }

  pub fn handle_click(&mut self) {
    let turn_color = self.turn_color;

    let Some(pos) = Pos::from_coords(mouse_position()) else {
      return;
    };
    if self.is_move_mode {
      let selected_idx = self.pieces.iter().position(|p| p.selected);
      let is_valid_move = selected_idx.is_some_and(|idx| {
        self.pieces[idx].position != Some(pos) && self.pieces[idx].moves.contains(&pos)
      });

      if !is_valid_move {
        self.is_move_mode = false;
        self.clear_selection();
        return;
      }

      let selected_idx = selected_idx.unwrap();
      if let Some(target_idx) = self.pieces.iter().position(|p| p.position == Some(pos)) {
        if target_idx != selected_idx {
          self.pieces[target_idx].position = None;
        }
      }
      self.pieces[selected_idx].position = Some(pos);
      self.pieces[selected_idx].has_moved = true;
      self.is_move_mode = false;
      self.switch_turn();
      self.clear_selection();
      self.generate_piece_moves();
    } else if let Some(clicked) = self.piece_at_mut(pos)
      && clicked.color == turn_color
    {
      clicked.selected = true;
      self.is_move_mode = true;
    }
  }

  pub fn generate_piece_moves(&mut self) {
    let moves = self
      .pieces
      .iter()
      .enumerate()
      .map(|(piece_index, piece)| {
        piece.position.map_or_else(Vec::new, |position| {
          self
            .generate_pseudo_moves_for_piece(piece.color, piece.kind, piece.has_moved, position)
            .into_iter()
            .filter(|target| self.is_move_legal(piece_index, *target))
            .collect()
        })
      })
      .collect::<Vec<_>>();

    for (piece, moves) in self.pieces.iter_mut().zip(moves) {
      piece.moves = moves;
    }
  }

  fn switch_turn(&mut self) {
    self.turn_color = !self.turn_color;
  }

  fn clear_selection(&mut self) {
    for piece in &mut self.pieces {
      piece.selected = false
    }
  }

  fn generate_pseudo_moves_for_piece(
    &self,
    color: PieceColor,
    kind: PieceKind,
    has_moved: bool,
    position: Pos,
  ) -> Vec<Pos> {
    match kind {
      PieceKind::King => {
        let mut moves = self.generate_stepping_moves(color, position, &ALL_DIRECTIONS);
        moves.extend(self.generate_castling_moves(color, has_moved, position));

        moves
      }
      PieceKind::Queen => self.generate_sliding_moves(color, position, &ALL_DIRECTIONS),
      PieceKind::Bishop => self.generate_sliding_moves(color, position, &DIAGONALS),
      PieceKind::Rook => self.generate_sliding_moves(color, position, &ORTHOGONALS),
      PieceKind::Knight => self.generate_stepping_moves(color, position, &KNIGHT_JUMPS),
      PieceKind::Pawn => self.generate_pawn_moves(color, has_moved, position),
    }
  }

  fn is_move_legal(&self, piece_index: usize, target: Pos) -> bool {
    let mut board = self.clone();
    let color = board.pieces[piece_index].color;

    board.apply_move_for_validation(piece_index, target);
    !board.is_king_in_check(color)
  }

  fn apply_move_for_validation(&mut self, piece_index: usize, target: Pos) {
    let captured_piece_indices = self
      .pieces
      .iter()
      .enumerate()
      .filter(|(index, piece)| *index != piece_index && piece.position == Some(target))
      .map(|(index, _)| index)
      .collect::<Vec<_>>();

    for index in captured_piece_indices {
      self.pieces[index].position = None;
      self.pieces[index].moves.clear();
    }

    self.pieces[piece_index].position = Some(target);
  }

  fn is_king_in_check(&self, color: PieceColor) -> bool {
    self
      .pieces
      .iter()
      .find(|piece| piece.color == color && piece.kind == PieceKind::King)
      .and_then(|king| king.position)
      .is_some_and(|king_pos| self.is_square_attacked_by(king_pos, !color))
  }

  fn is_square_attacked_by(&self, position: Pos, attacker_color: PieceColor) -> bool {
    self.pieces.iter().any(|piece| {
      let Some(piece_pos) = piece.position else {
        return false;
      };

      piece.color == attacker_color
        && self.piece_attacks_square(piece.kind, piece_pos, position, attacker_color)
    })
  }

  fn piece_attacks_square(
    &self,
    kind: PieceKind,
    from: Pos,
    target: Pos,
    attacker_color: PieceColor,
  ) -> bool {
    match kind {
      PieceKind::King => ALL_DIRECTIONS
        .iter()
        .filter_map(|offset| from.offset(*offset))
        .any(|pos| pos == target),
      PieceKind::Knight => KNIGHT_JUMPS
        .iter()
        .filter_map(|offset| from.offset(*offset))
        .any(|pos| pos == target),
      PieceKind::Pawn => {
        let direction = if attacker_color == PieceColor::White {
          -1
        } else {
          1
        };

        [-1, 1]
          .into_iter()
          .filter_map(|file_offset| from.offset(Pos(file_offset, direction)))
          .any(|pos| pos == target)
      }
      PieceKind::Queen => self.sliding_piece_attacks_square(from, target, &ALL_DIRECTIONS),
      PieceKind::Bishop => self.sliding_piece_attacks_square(from, target, &DIAGONALS),
      PieceKind::Rook => self.sliding_piece_attacks_square(from, target, &ORTHOGONALS),
    }
  }

  fn sliding_piece_attacks_square(&self, from: Pos, target: Pos, directions: &[Pos]) -> bool {
    for direction in directions {
      let mut current_pos = from;

      while let Some(next_pos) = current_pos.offset(*direction) {
        if next_pos == target {
          return true;
        }

        if !self.is_empty(next_pos) {
          break;
        }

        current_pos = next_pos;
      }
    }

    false
  }

  fn generate_stepping_moves(&self, color: PieceColor, position: Pos, offsets: &[Pos]) -> Vec<Pos> {
    offsets
      .iter()
      .filter_map(|offset| position.offset(*offset))
      .filter(|next_pos| !self.has_friendly_piece(color, *next_pos))
      .collect()
  }

  fn generate_sliding_moves(
    &self,
    color: PieceColor,
    position: Pos,
    directions: &[Pos],
  ) -> Vec<Pos> {
    let mut moves = Vec::new();

    for direction in directions {
      let mut current_pos = position;

      while let Some(next_pos) = current_pos.offset(*direction) {
        if self.has_friendly_piece(color, next_pos) {
          break;
        }

        moves.push(next_pos);

        if self.has_enemy_piece(color, next_pos) {
          break;
        }

        current_pos = next_pos;
      }
    }

    moves
  }

  fn generate_pawn_moves(&self, color: PieceColor, has_moved: bool, position: Pos) -> Vec<Pos> {
    let mut moves = Vec::new();
    let direction = if color == PieceColor::White { -1 } else { 1 };

    if let Some(forward) = position.offset(Pos(0, direction))
      && self.is_empty(forward)
    {
      moves.push(forward);

      if !has_moved
        && let Some(double_forward) = forward.offset(Pos(0, direction))
        && self.is_empty(double_forward)
      {
        moves.push(double_forward);
      }
    }

    moves.extend(
      [-1, 1]
        .into_iter()
        .filter_map(|file_offset| position.offset(Pos(file_offset, direction)))
        .filter(|capture_pos| self.has_enemy_piece(color, *capture_pos)),
    );

    moves
  }

  fn generate_castling_moves(&self, color: PieceColor, has_moved: bool, position: Pos) -> Vec<Pos> {
    if has_moved || self.is_square_attacked_by(position, !color) {
      return Vec::new();
    }

    [(7, 6, [5, 6].as_slice()), (0, 2, [3, 2, 1].as_slice())]
      .into_iter()
      .filter_map(|(rook_col, king_target_col, empty_cols)| {
        let row = position.1;
        let rook_pos = Pos(rook_col, row);
        let rook = self.piece_at(rook_pos)?;

        if rook.color != color || rook.kind != PieceKind::Rook || rook.has_moved {
          return None;
        }

        if empty_cols.iter().any(|col| !self.is_empty(Pos(*col, row))) {
          return None;
        }

        let attacked_cols = if king_target_col == 6 { [5, 6] } else { [3, 2] };
        if attacked_cols
          .iter()
          .any(|col| self.is_square_attacked_by(Pos(*col, row), !color))
        {
          return None;
        }

        Some(Pos(king_target_col, row))
      })
      .collect()
  }

  fn piece_at(&self, position: Pos) -> Option<&Piece> {
    self
      .pieces
      .iter()
      .find(|piece| piece.position == Some(position))
  }

  fn piece_at_mut(&mut self, position: Pos) -> Option<&mut Piece> {
    self
      .pieces
      .iter_mut()
      .find(|piece| piece.position == Some(position))
  }

  fn is_empty(&self, position: Pos) -> bool {
    self.piece_at(position).is_none()
  }

  fn has_friendly_piece(&self, color: PieceColor, position: Pos) -> bool {
    self
      .piece_at(position)
      .is_some_and(|piece| piece.color == color)
  }

  fn has_enemy_piece(&self, color: PieceColor, position: Pos) -> bool {
    self
      .piece_at(position)
      .is_some_and(|piece| piece.color != color)
  }
}
