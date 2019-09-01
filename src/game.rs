extern crate glfw;
extern crate gl;
use std::collections::HashSet;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::mpsc::Receiver;
use std::f32;
use self::glfw::{Context, Key, Action, Glfw};
use crate::sprite::Sprite;
use crate::game_object::{GameObject, Ball, Brick, Bg, Paddle};
use cgmath::{vec3, vec2};
use crate::{WIDTH, HEIGHT};

enum GameState {
  GameActive,
  GameMenu,
  GameWin
}

pub struct Game {
  state: GameState,
  window: glfw::Window,
  events: Receiver<(f64, glfw::WindowEvent)>,
  glfw: Glfw,
  sprite: Sprite,
  game_objects: Vec<Rc<RefCell<GameObject>>>,
  keys: HashSet<Key>,
  player: Option<Paddle>,
  ball: Option<Ball>,
}

impl Game {
  pub fn new() -> Game {
     // glfw: initialize and configure
    // ------------------------------
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::Resizable(false));

    // glfw window creation
    // --------------------
    let (mut window, events) = glfw.create_window(WIDTH, HEIGHT, "Breakuut", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    window.set_cursor_pos_polling(true);

    // gl: load all OpenGL function pointers
    // ---------------------------------------
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    unsafe {
      gl::Enable(gl::CULL_FACE);
      gl::Enable(gl::BLEND);
      gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    Game {
      state: GameState::GameMenu,
      sprite: Sprite::new(WIDTH, HEIGHT),
      window,
      events,
      glfw,

      keys: HashSet::new(),
      game_objects: vec![],
      player: None,
      ball: None,
    }
  }
}

impl Game {
  pub fn load_game_object(&mut self, game_object: Rc<RefCell<GameObject>>) {
    self.game_objects.push(game_object);
  }

  pub fn process_events(&mut self) {
    for (_, event) in glfw::flush_messages(&self.events) {
        match event {
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => self.window.set_should_close(true),
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
    self.load();
    let mut last_time = 0.0;
    while !self.window.should_close() {
      self.process_events();

      unsafe {
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
      }

      let curr_time = self.glfw.get_time() as f32;
      self.update(curr_time - last_time);
      last_time = curr_time;

      for game_object in self.game_objects.iter() {
        self.sprite.draw(
          &game_object.borrow().texture, 
          game_object.borrow().position, 
          game_object.borrow().size,
          0.0,
          game_object.borrow().color
        );

      }

      // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
      // -------------------------------------------------------------------------------
      self.window.swap_buffers();
      self.glfw.poll_events();
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
  fn load(&mut self) {
    
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

    let mut bricks: Vec<Brick> = vec![];


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
    let paddle = Paddle::new(vec2(500.0, 580.0));

    self.load_game_object(Rc::clone(&bg.game_object));
    for brick in bricks {
      self.load_game_object(Rc::clone(&brick.game_object));
    }
    self.load_game_object(Rc::clone(&ball.game_object));
    self.load_game_object(Rc::clone(&paddle.game_object));

    self.player = Some(paddle);
    self.ball = Some(ball);
  }

  fn check_ball_player_collision(&mut self) {
    if let Some(ref player) = self.player {
      if let Some(ref mut ball) = self.ball {
        let player_go = player.game_object.borrow_mut();
        let mut ball_go = ball.game_object.borrow_mut();
        if Self::check_collision(&player_go, &ball_go) {
          ball.velocity[1] = -ball.velocity[1];
          ball_go.position[1] = player_go.position[1] - ball_go.size[1];
        }
      }
    }
  }

  fn update(&mut self, dt: f32) {
    if self.keys.contains(&Key::A) {
      if let Some(ref player) = self.player {
        let mut go = player.game_object.borrow_mut();
        if go.position[0] > 0.0 {
          go.position[0] -= player.velocity * dt;
        }
      }
    }
    if self.keys.contains(&Key::D) {
      if let Some(ref player) = self.player {
        let mut go = player.game_object.borrow_mut();
        if go.position[0] < WIDTH as f32 - go.size[0] {
          go.position[0] += player.velocity * dt;
        }
      }
    }
    if self.keys.contains(&Key::Space) {
      if let Some(ref mut ball) = self.ball {
        ball.is_stuck = false;
      }
    }

    self.check_ball_player_collision();

    if let Some(ref mut ball) = self.ball {
      let mut go = ball.game_object.borrow_mut();
      if !ball.is_stuck {
        if go.position[0] >= WIDTH as f32 - go.size[0] {
          ball.velocity[0] = -ball.velocity[0];
          go.position = vec2(WIDTH as f32 - go.size[0], go.position[1]);
        } else if go.position[0] <= 0.0 {
          ball.velocity[0] = -ball.velocity[0];
          go.position = vec2(0.0, go.position[1]);
        }
        if go.position[1] <= 0.0 {
          ball.velocity[1] = -ball.velocity[1];
          go.position[1] = 0.0;
        }

        go.position += ball.velocity * dt;
      } else {
        if let Some(ref player) = self.player {
          let player_go = player.game_object.borrow();
          go.position[0] = player_go.position[0] + player_go.size[0] / 2.0 - go.size[0] / 2.0;
          go.position[1] = player_go.position[1] - go.size[1];
        }
      }
    }
  }
}