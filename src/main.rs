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

  let board = Board::new();

  loop {
    clear_background(Color::from_hex(0x303134));

    for (i, square) in board.squares.iter().enumerate() {
      square.render(i as i32);
    }

    draw_texture_ex(
      &resources.b_bishop,
      (screen_width() / 2.0) - 95.0,
      screen_height() / 2.0 + 10.0,
      WHITE,
      DrawTextureParams {
        dest_size: Some(vec2(88.0, 88.0)),
        ..Default::default()
      },
    );

    if is_key_down(KeyCode::Escape) {
      std::process::exit(0);
    }

    next_frame().await
  }
}
