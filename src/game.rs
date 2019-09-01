extern crate glfw;
extern crate gl;
use std::sync::mpsc::Receiver;
use self::glfw::{Context, Key, Action};
use crate::sprite::Sprite;
use crate::texture::Texture;
use cgmath::{Vector2, Vector3};
use std::collections::HashMap;

enum GameState {
  GameActive,
  GameMenu,
  GameWin
}

pub struct Game<'a> {
  state: GameState,
  width: u32,
  height: u32,
  sprite: Sprite,
  textures: HashMap<&'a str, Texture>,
}

// NOTE: not the same version as in common.rs!
fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            _ => {}
        }
    }
}

impl<'a> Game<'a> {
  pub fn init(width: u32, height: u32) -> Game<'a> {
     // glfw: initialize and configure
    // ------------------------------
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    // glfw window creation
    // --------------------
    let (mut window, events) = glfw.create_window(width, height, "Breakuut", glfw::WindowMode::Windowed)
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

    let mut game = Game {
      width, 
      height, 
      state: GameState::GameMenu,
      sprite: Sprite::new(width, height),
      textures: HashMap::new()
    };

    game.load();

    while !window.should_close() {
      // events
      // -----
      process_events(&mut window, &events);

      unsafe {
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
      }

      game.update();

      // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
      // -------------------------------------------------------------------------------
      window.swap_buffers();
      glfw.poll_events();
    }

    game
  }
}

impl<'a> Game<'a> {
  pub fn loadTexture(&mut self, name: &'a str, src: &str) {
    self.textures.insert(name, Texture::new(src));
  }

  pub fn getTexture(&self, name: &'a str) -> &Texture {
    self.textures.get(name).unwrap()
  }

  pub fn draw(&self, texture: &Texture, position: Vector2<f32>, size: Vector2<f32>, rotate: f32, color: Vector3<f32>) {
    self.sprite.draw(texture, position, size, rotate, color);
  }
}