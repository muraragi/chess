use macroquad::{Error, prelude::*};

use crate::chess::Board;
use crate::resources::Resources;

mod chess;
mod resources;

fn get_window_config() -> Conf {
  Conf {
    window_title: "Chess".to_string(),
    window_width: 1920,
    window_height: 1080,
    ..Default::default()
  }
}
#[macroquad::main(get_window_config)]
async fn main() -> Result<(), Error> {
  set_pc_assets_folder("assets");
  set_default_filter_mode(FilterMode::Linear);
  let resources = Resources::new().await?;
  build_textures_atlas();

  let mut board = Board::new();
  board.generate_piece_moves();

  loop {
    clear_background(Color::from_hex(0x303134));

    board.render(&resources);
    board.draw_info();

    if is_mouse_button_released(MouseButton::Left) {
      board.handle_click();
    }

    if is_key_down(KeyCode::Escape) {
      std::process::exit(0);
    }

    next_frame().await
  }
}
