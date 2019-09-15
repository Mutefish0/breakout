extern crate glfw;
extern crate gl;
use std::collections::HashSet;
use std::f32;
use self::glfw::{Context, Key, Action};
use crate::sprite::Sprite;
use crate::game_object::{GameObject, Ball, Brick, Bg, Paddle};
use cgmath::{vec3, vec2, InnerSpace, Vector2, Matrix2, Deg, Rad};
use rand::prelude::*;
use rand::distributions::WeightedIndex;
use crate::{WIDTH, HEIGHT};
use crate::window::{Window};

pub enum Direction {
  UP,
  RIGHT,
  DOWN,
  LEFT
}

pub struct Collision (bool, Direction, f32);

pub struct Game {
  window: Window,
  sprite: Sprite,
  keys: HashSet<Key>,
  
  bg: Bg,
  player: Paddle,
  ball: Ball,
  bricks: Vec<Brick>,
}

impl Game {
  pub fn new() -> Game {
    let window = Window::new();
    let bg = Bg::new();

    let (player, ball, bricks) = Self::gen_level();

    Game {
      sprite: Sprite::new(WIDTH, HEIGHT),
      window,

      keys: HashSet::new(),
      player,
      ball,
      bricks,
      bg,
    }
  }
}

impl Game {
  pub fn process_events(&mut self) {
    for (_, event) in glfw::flush_messages(&self.window.events) {
        match event {
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => self.window.win.set_should_close(true),
            glfw::WindowEvent::Key(key, _, Action::Press, _) => {
              self.keys.insert(key);
            },
            glfw::WindowEvent::Key(key, _, Action::Release, _) => {
              self.keys.remove(&key);
            },
            _ => {}
        }
    }
  }

  pub fn run(&mut self) {
    let mut last_time = 0.0;
    while !self.window.win.should_close() {
      self.process_events();

      unsafe {
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
      }

      let curr_time = self.window.glfw.get_time() as f32;
      self.update(curr_time - last_time);
      last_time = curr_time;

      // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
      // -------------------------------------------------------------------------------
      self.window.win.swap_buffers();
      self.window.glfw.poll_events();
    }
  }

  pub fn check_collision(goa: &GameObject, gob: &GameObject) -> bool {
    let collision_x = !(
      goa.position[0] >= gob.position[0] + gob.size[0]
      || goa.position[0] + goa.size[0] < gob.position[0]
    );
    let collision_y = !(
      goa.position[1] >= gob.position[1] + gob.size[1]
      || goa.position[1] + goa.size[1] < gob.position[1]
    );
    return collision_x && collision_y;
  }

  pub fn calc_vector_direction(v: Vector2<f32>) -> Direction {
    let left = v.dot(vec2(1.0, 0.0));
    let down = v.dot(vec2(0.0, 1.0));
    let right = v.dot(vec2(-1.0, 0.0));
    let up = v.dot(vec2(0.0, -1.0));
    let max = left.max(down.max(right.max(up)));
    if max == left {
      Direction::LEFT
    } else if max == down {
      Direction::DOWN
    } else if max == right {
      Direction::RIGHT
    } else {
      Direction::UP
    }
  }

  pub fn check_circle_rect_collision(circle: &GameObject, rect: &GameObject) -> Collision {
    let circle_radius = circle.size[0] / 2.0;
    let circle_center = circle.position + circle.size / 2.0;
    let rect_half_extends = rect.size / 2.0;
    let rect_center = rect.position + rect_half_extends;
    let diff = circle_center - rect_center;
    let clamped_diff = vec2(
        (-rect_half_extends[0]).max(rect_half_extends[0].min(diff[0])),
        (-rect_half_extends[1]).max(rect_half_extends[1].min(diff[1]))
    );
    let closest = rect_center + clamped_diff;

    let is_outside = clamped_diff.dot(closest - circle_center) < 0.0;

    let magnitude = (closest - circle_center).magnitude();
    let penetration = circle_radius - magnitude;
    Collision (
      penetration > 0.0 && is_outside,
      Self::calc_vector_direction(diff),
      penetration
    )
  }

  fn gen_level() -> (Paddle, Ball, Vec<Brick>) {
    let mut tile_data = vec![];
    let width = 12;
    let height = 6;
    let lv_width = WIDTH;
    let lv_height = HEIGHT / 2;

    let (unit_width, unit_height) = (lv_width as f32 / width as f32, lv_height as f32 / height as f32);

    let choices = [1, 2, 3, 4, 5];
    let weights = [3, 4, 4, 4, 4];
    let dist = WeightedIndex::new(&weights).unwrap();

    let mut rng = thread_rng();
    for _ in 0..width * height {
      tile_data.push(choices[dist.sample(&mut rng)]);
    }

    let mut bricks = vec![];
    {
      for (idx, t) in tile_data.iter().enumerate() {
        let row = idx / width;
        let column = idx % width;
        let position = vec2(column as f32 * unit_width, row as f32 * unit_height);
        match t {
          1 => bricks.push(Brick::new(position, vec2(unit_width, unit_height), vec3(0.8, 0.8, 0.7), true)),
          2 => bricks.push(Brick::new(position, vec2(unit_width, unit_height), vec3(0.2, 0.6, 1.0), false)),
          3 => bricks.push(Brick::new(position, vec2(unit_width, unit_height), vec3(0.0, 0.7, 0.0), false)),
          4 => bricks.push(Brick::new(position, vec2(unit_width, unit_height), vec3(0.8, 0.8, 0.4), false)),
          5 => bricks.push(Brick::new(position, vec2(unit_width, unit_height), vec3(1.0, 0.5, 0.0), false)),
          _ => ()
        };
      }
    }

    let ball = Ball::new(vec2(750.0, 550.0));
    let player = Paddle::new(vec2(500.0, 580.0));

    (player, ball, bricks)
  }
}

impl Game {
  fn draw(&self, game_object: &GameObject) {
    self.sprite.draw(&game_object.texture, game_object.position, game_object.size, 0.0, game_object.color);
  }

  fn check_ball_player_collision(&mut self) {
    let player = &self.player;
    let ball = &mut self.ball;
    let player_go = &player.game_object;
    let ball_go = &mut ball.game_object;
    if Self::check_collision(&player_go, &ball_go) {
      ball.velocity.y = -ball.velocity.y;
      ball_go.position.y = player_go.position.y - ball_go.size.y;
      // 球碰撞点距离挡板中心的距离
      let percentage = (ball_go.position.x + ball_go.size.x / 2.0 - player_go.position.x - player_go.size.x / 2.0) / (player_go.size.x / 2.0);
      let angle = ball.velocity.angle(vec2(0.0, -1.0));
      let mut next_angle = angle + Rad::from(Deg(20.0 * percentage));
      if next_angle > Rad::from(Deg(45.0)) {
        next_angle = Rad::from(Deg(45.0));
      } else if next_angle < Rad::from(Deg(-45.0)) {
        next_angle = Rad::from(Deg(-45.0));
      }
      let rotation = Matrix2::from_angle(next_angle);
      ball.velocity = rotation * vec2(0.0, -1.0) * ball.velocity.magnitude();
    }
  }

  fn check_ball_brick_collision(&mut self) {
    for brick in self.bricks.iter_mut() {
      if brick.is_destroyed {
        continue;
      }
      let Collision (collided, direction, penetration) = Self::check_circle_rect_collision(&self.ball.game_object, &brick.game_object);
      if collided {
        if !brick.is_solid {
          brick.is_destroyed = true;
        }
        match direction {
          Direction::LEFT => {
            self.ball.velocity.x = -self.ball.velocity.x;
            self.ball.game_object.position.x = self.ball.game_object.position.x - penetration; 
          },
          Direction::RIGHT => {
            self.ball.velocity.x = -self.ball.velocity.x;
            self.ball.game_object.position.x = self.ball.game_object.position.x + penetration;
          },
          Direction::DOWN => {
            self.ball.velocity.y = -self.ball.velocity.y;
            self.ball.game_object.position.y = self.ball.game_object.position.y + penetration;
          },
          Direction::UP => {
            self.ball.velocity.y = -self.ball.velocity.y;
            self.ball.game_object.position.y = self.ball.game_object.position.y - penetration;
          },
        }
      }
    }
  }

  fn check_ball_border_collision(&mut self) {
    let ball = &mut self.ball;
    let ball_go = &mut ball.game_object;

    if ball_go.position.x >= WIDTH as f32 - ball_go.size.x {
        ball.velocity.x = -ball.velocity.x;
        ball_go.position.x = WIDTH as f32 - ball_go.size.x;
    } else if ball_go.position[0] <= 0.0 {
      ball.velocity.x = -ball.velocity.x;
      ball_go.position.x = 0.0;
    }
    if ball_go.position.y <= 0.0 {
      ball.velocity.y = -ball.velocity.y;
      ball_go.position.y = 0.0;
    }
  }

  fn check_game_over(&mut self) {
    if self.ball.game_object.position.y > HEIGHT as f32 {
      self.reset();
    }
  }

  fn reset(&mut self) {
    let (player, ball, bricks) = Self::gen_level();
    self.player = player;
    self.ball = ball;
    self.bricks = bricks;
  }

  fn update(&mut self, dt: f32) {
    self.check_ball_border_collision();
    self.check_ball_player_collision();
    self.check_ball_brick_collision();

    let player = &mut self.player;
    let ball = &mut self.ball;
    let player_go = &mut player.game_object;
    let ball_go = &mut ball.game_object;

    if self.keys.contains(&Key::A) {
      if player_go.position.x > 0.0 {
        player_go.position.x -= player.velocity * dt;
      }
    }

    if self.keys.contains(&Key::D) {
      if player_go.position.x < WIDTH as f32 - player_go.size.x {
        player_go.position.x += player.velocity * dt;
      } 
    }

    if self.keys.contains(&Key::Space) {
      ball.is_stuck = false;
    }
      
    if !ball.is_stuck {
      ball_go.position += ball.velocity * dt;
    } else {
      ball_go.position.x = player_go.position.x + player_go.size.x / 2.0 - ball_go.size.x / 2.0;
      ball_go.position.y = player_go.position.y - ball_go.size.y;
    }

    self.draw(&self.bg.game_object);
    for brick in &self.bricks {
      if !brick.is_destroyed {
        self.draw(&brick.game_object);
      }
    }
    self.draw(&self.player.game_object);
    self.draw(&self.ball.game_object);

    self.check_game_over();
  }
}