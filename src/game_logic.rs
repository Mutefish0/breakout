use crate::game::Game;
use cgmath::{vec3, vec2};

impl<'a> Game<'a> {
  pub fn load(&mut self) {
    self.loadTexture("face", "src/res/face.png");
  }

  pub fn update(&self) {
    self.draw(self.getTexture("face"), vec2(200.0, 200.0), vec2(300.0, 400.0), 45.0, vec3(1.0, 1.0, 1.0));
  }
}