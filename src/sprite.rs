use crate::shader::Shader;
use crate::texture::Texture;

struct Sprite {
  shader: Shader
}

impl Sprite {
  fn new() -> Sprite {
    let shader = Shader::new("src/sprite.vs", "src/sprite.fs");
    Sprite {
      shader
    }
  }
  fn draw() {

  }
}