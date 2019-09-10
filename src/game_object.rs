use cgmath::{ Vector2, Vector3, vec2, vec3 };
use crate::texture::Texture;
use crate::{WIDTH, HEIGHT};
use std::hash::{Hash, Hasher};

pub struct GameObject {
  pub position: Vector2<f32>,
  pub size: Vector2<f32>,
  pub color: Vector3<f32>,
  pub texture: Texture
}

impl GameObject {
  pub fn new(
    src: &str, 
    position: Vector2<f32>, 
    size: Vector2<f32>, 
    color: Vector3<f32>, 
  ) -> GameObject {
    GameObject {
      position,
      size,
      color,
      texture: Texture::new(src)
    }
  }
}

pub struct Ball {
  pub game_object: GameObject,
  pub velocity: Vector2<f32>,
  pub radius: f32,
  pub is_stuck: bool,
}

impl Ball {
  pub fn new(position: Vector2<f32>) -> Ball {
    Ball {
      is_stuck: true,
      radius: 12.5,
      velocity: vec2(100.0, -350.0),
      game_object: GameObject::new("src/res/face.png", position, vec2(25.0, 25.0), vec3(1.0, 1.0, 1.0))
    }
  }
}

static mut NEXT_BRICK_ID: u32 = 0;

pub struct Brick {
  pub game_object: GameObject,
  pub is_solid: bool,
  pub is_destroyed: bool,
  id: u32,
}

impl PartialEq for Brick {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
  }
}

impl Eq for Brick {}

impl Hash for Brick {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.id.hash(state);
  }
}

impl Brick {
  pub fn new(position: Vector2<f32>, size: Vector2<f32>, color: Vector3<f32>, is_solid: bool) -> Brick {
    let src = if is_solid { "src/res/block_solid.png" } else { "src/res/block.png" };
    unsafe {
      NEXT_BRICK_ID += 1;
        Brick {
        id: NEXT_BRICK_ID,
        is_solid,
        is_destroyed: false,
        game_object: GameObject::new(src, position, size, color)
      }
    }
  }
}

pub struct Bg {
  pub game_object: GameObject
}

impl Bg {
  pub fn new() -> Bg {
    Bg {
      game_object: GameObject::new("src/res/bg.jpg", vec2(0.0, 0.0), vec2(WIDTH as f32, HEIGHT as f32), vec3(1.0, 1.0, 1.0))
    }
  }
}

pub struct Paddle {
  pub game_object: GameObject,
  pub velocity: f32,
}

impl Paddle {
  pub fn new(position: Vector2<f32>) -> Paddle {
    Paddle {
      game_object: GameObject::new("src/res/paddle.png", position, vec2(100.0, 20.0), vec3(1.0, 1.0, 1.0)),
      velocity: 500.0
    }
  }
}


