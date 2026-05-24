use macroquad::{Error, prelude::*};

use crate::chess::Board;
use crate::resources::Resources;
use crate::ui::{MenuAction, UI};

mod chess;
mod resources;
mod ui;

fn get_window_config() -> Conf {
  Conf {
    window_title: "Chess".to_string(),
    window_width: 1920,
    window_height: 1080,
    ..Default::default()
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GameState {
  Menu,
  Chess,
}

#[macroquad::main(get_window_config)]
async fn main() -> Result<(), Error> {
  set_pc_assets_folder("assets");
  set_default_filter_mode(FilterMode::Linear);
  let resources = Resources::new().await?;
  build_textures_atlas();

  let mut state: GameState = GameState::Menu;

  let mut board = Board::new();
  let ui = UI::new();

  loop {
    clear_background(Color::from_hex(0x303134));
    match state {
      GameState::Menu => match ui.render_menu(board.game_started()) {
        MenuAction::Continue => state = GameState::Chess,
        MenuAction::NewGame => {
          board = Board::new();
          state = GameState::Chess;
        }
        MenuAction::Quit => return Ok(()),
        MenuAction::None => {}
      },
      GameState::Chess => {
        board.render(&resources);
        board.draw_info();

        if is_mouse_button_released(MouseButton::Left) {
          board.handle_click();
        }

        if is_key_pressed(KeyCode::Escape) {
          state = GameState::Menu;
        }
      }
    }

    next_frame().await
  }
}
