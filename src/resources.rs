use macroquad::{
  Error,
  texture::{Texture2D, load_texture},
};

pub struct Resources {
  pub b_bishop: Texture2D,
  pub b_king: Texture2D,
  pub b_rook: Texture2D,
  pub b_knight: Texture2D,
  pub b_queen: Texture2D,
  pub b_pawn: Texture2D,
  pub w_bishop: Texture2D,
  pub w_king: Texture2D,
  pub w_rook: Texture2D,
  pub w_knight: Texture2D,
  pub w_queen: Texture2D,
  pub w_pawn: Texture2D,
}

impl Resources {
  pub async fn new() -> Result<Self, Error> {
    let b_bishop = load_texture("b_bishop.png").await?;
    let b_king = load_texture("b_king.png").await?;
    let b_rook = load_texture("b_rook.png").await?;
    let b_knight = load_texture("b_knight.png").await?;
    let b_queen = load_texture("b_queen.png").await?;
    let b_pawn = load_texture("b_pawn.png").await?;
    let w_bishop = load_texture("w_bishop.png").await?;
    let w_king = load_texture("w_king.png").await?;
    let w_rook = load_texture("w_rook.png").await?;
    let w_knight = load_texture("w_knight.png").await?;
    let w_queen = load_texture("w_queen.png").await?;
    let w_pawn = load_texture("w_pawn.png").await?;

    Ok(Self {
      b_bishop,
      b_king,
      b_rook,
      b_knight,
      b_queen,
      b_pawn,
      w_bishop,
      w_king,
      w_rook,
      w_knight,
      w_queen,
      w_pawn,
    })
  }
}
