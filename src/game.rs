extern crate glfw;
extern crate gl;
use std::collections::HashSet;
use std::f32;
use self::glfw::{Context, Key, Action};
use crate::sprite::Sprite;
use crate::game_object::{GameObject, Ball, Brick, Bg, Paddle};
use cgmath::{vec3, vec2};
use crate::{WIDTH, HEIGHT};

use crate::window::{Window};

enum GameState {
  GameActive,
  GameMenu,
  GameWin
}

pub struct Game {
  state: GameState,
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

    let tile_data = [
      1, 1, 1, 1, 1, 1,
      2, 2, 0, 0, 2, 2,
      3, 3, 4, 4, 3, 3,
    ];

    let width = 6;
    let height = 3;
    let lv_width = WIDTH;
    let lv_height = HEIGHT / 2;

    let (unit_width, unit_height) = (lv_width as f32 / width as f32, lv_height as f32 / height as f32);

    let mut bricks = vec![];

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

    let bg = Bg::new();
    let ball = Ball::new(vec2(750.0, 550.0));
    let player = Paddle::new(vec2(500.0, 580.0));

    Game {
      state: GameState::GameMenu,
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
      ball.velocity[1] = -ball.velocity[1];
      ball_go.position[1] = player_go.position[1] - ball_go.size[1];
    }
  }

  fn check_ball_brick_collision(&mut self) {
    for brick in self.bricks.iter_mut() {
      if Self::check_collision(&self.ball.game_object, &brick.game_object) {
        if brick.is_solid {

        } else {
          brick.is_destroyed = true;
        }
      }
    }
  }

  fn update(&mut self, dt: f32) {
    let player = &mut self.player;
    let ball = &mut self.ball;
    let player_go = &mut player.game_object;
    let ball_go = &mut ball.game_object;

    if self.keys.contains(&Key::A) {
      if player_go.position[0] > 0.0 {
        player_go.position[0] -= player.velocity * dt;
      }
    }

    if self.keys.contains(&Key::D) {
      if player_go.position[0] < WIDTH as f32 - player_go.size[0] {
        player_go.position[0] += player.velocity * dt;
      } 
    }

    if self.keys.contains(&Key::Space) {
      ball.is_stuck = false;
    }    
      
    if !ball.is_stuck {
      if ball_go.position[0] >= WIDTH as f32 - ball_go.size[0] {
        ball.velocity[0] = -ball.velocity[0];
        ball_go.position = vec2(WIDTH as f32 - ball_go.size[0], ball_go.position[1]);
      } else if ball_go.position[0] <= 0.0 {
        ball.velocity[0] = -ball.velocity[0];
        ball_go.position = vec2(0.0, ball_go.position[1]);
      }
      if ball_go.position[1] <= 0.0 {
        ball.velocity[1] = -ball.velocity[1];
        ball_go.position[1] = 0.0;
      }
      ball_go.position += ball.velocity * dt;
    } else {
      ball_go.position[0] = player_go.position[0] + player_go.size[0] / 2.0 - ball_go.size[0] / 2.0;
      ball_go.position[1] = player_go.position[1] - ball_go.size[1];
    }

    self.check_ball_player_collision();
    self.check_ball_brick_collision();

    self.draw(&self.bg.game_object);
    for brick in &self.bricks {
      if !brick.is_destroyed {
        self.draw(&brick.game_object);
      }
    }
    self.draw(&self.player.game_object);
    self.draw(&self.ball.game_object);
  }
}