use macroquad::{prelude::*, ui::*};

use crate::chess::PieceColor;

const BACKGROUND_COLOR: Color = Color::from_hex(0x7C4C3E);
const BUTTON_COLOR: Color = Color::from_hex(0x512A2A);
const BUTTON_HEIGHT: f32 = 50.0;
const BUTTON_SPACING: f32 = 72.0;
const BUTTON_BASE_OFFSET: f32 = -120.0;

pub enum MenuAction {
  Continue,
  NewGame,
  Quit,
  None,
}

pub enum GameOverAction {
  NewGame,
  BackToMenu,
  None,
}

pub struct UI;

impl UI {
  pub fn new() -> Self {
    let label_style = root_ui()
      .style_builder()
      .text_color(WHITE)
      .font_size(48)
      .build();

    let window_style = root_ui()
      .style_builder()
      .color(BACKGROUND_COLOR)
      .margin(RectOffset::new(0.0, 0.0, 0.0, 0.0))
      .build();

    let button_style = root_ui()
      .style_builder()
      .background_margin(RectOffset::new(16.0, 16.0, 16.0, 16.0))
      .margin(RectOffset::new(16.0, 0.0, -8.0, -8.0))
      .color(BUTTON_COLOR)
      .text_color(WHITE)
      .text_color_hovered(BLACK)
      .font_size(52)
      .build();

    let skin = Skin {
      label_style,
      window_style,
      button_style,
      ..root_ui().default_skin()
    };

    root_ui().push_skin(&skin);

    Self
  }

  fn menu_button(&self, ui: &mut Ui, label: &str, screen: Vec2, slot: i32) -> bool {
    let size = Vec2::new(screen.x / 3.0, BUTTON_HEIGHT);
    let x = (screen.x / 2.0) - (size.x / 2.0);
    let y = (screen.y / 2.0) + BUTTON_BASE_OFFSET + (slot as f32 * BUTTON_SPACING);
    widgets::Button::new(label)
      .size(size)
      .position(vec2(x, y))
      .ui(ui)
  }

  pub fn render_menu(&self, can_continue: bool) -> MenuAction {
    let screen = Vec2::new(screen_width(), screen_height());
    let mut action = MenuAction::None;

    root_ui().window(hash!(), vec2(0.0, 0.0), screen, |ui| {
      ui.label(Vec2::new(15.0, 0.0), "CHESS");
      if can_continue && self.menu_button(ui, "CONTINUE", screen, 0) {
        action = MenuAction::Continue;
      }
      if self.menu_button(ui, "NEW GAME", screen, 1) {
        action = MenuAction::NewGame;
      }
      if self.menu_button(ui, "QUIT", screen, 2) {
        action = MenuAction::Quit;
      }
    });

    action
  }

  pub fn render_game_over(&self, winner: PieceColor) -> GameOverAction {
    let screen = Vec2::new(screen_width(), screen_height());
    let mut action = GameOverAction::None;

    draw_rectangle(0.0, 0.0, screen.x, screen.y, Color::new(0.0, 0.0, 0.0, 0.5));

    let size = vec2(screen.x / 2.0, screen.y / 2.0);
    let pos = (vec2(screen.x, screen.y) - size) * 0.5;
    widgets::Window::new(hash!(), pos, size)
      .titlebar(false)
      .movable(false)
      .ui(&mut *root_ui(), |ui| {
        ui.label(
          Vec2::new(16.0, 8.0),
          format!("CHECK MATE! WINNER - {}", winner).as_str(),
        );
        if self.menu_button(ui, "NEW GAME", size, 1) {
          action = GameOverAction::NewGame
        }
        if self.menu_button(ui, "MENU", size, 2) {
          action = GameOverAction::BackToMenu;
        }
      });

    action
  }
}
