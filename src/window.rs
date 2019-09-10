extern crate glfw;
use self::glfw::{ Context };
use std::sync::mpsc::Receiver;
use crate::{WIDTH, HEIGHT};

pub struct Window {
  pub glfw: glfw::Glfw,
  pub win: glfw::Window,
  pub events: Receiver<(f64, glfw::WindowEvent)>
}

impl Window {
  pub fn new() -> Window {
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
    let (mut win, events) = glfw.create_window(WIDTH, HEIGHT, "Breakuut", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    win.make_current();
    win.set_key_polling(true);
    win.set_framebuffer_size_polling(true);
    win.set_cursor_pos_polling(true);

    // gl: load all OpenGL function pointers
    // ---------------------------------------
    gl::load_with(|symbol| win.get_proc_address(symbol) as *const _);

    unsafe {
      gl::Enable(gl::CULL_FACE);
      gl::Enable(gl::BLEND);
      gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    Window {
      glfw,
      win,
      events,
    }
  }
}