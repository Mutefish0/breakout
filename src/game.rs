extern crate glfw;
use self::glfw::Context;

enum GameState {
  GameActive,
  GameMenu,
  GameWin
}

pub struct Game {
  state: GameState,
  width: u32,
  height: u32
}

impl Game {
  pub fn init(width: u32, height: u32) -> Game {
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
    window.set_framebuffer_size_polling(true);
    window.set_cursor_pos_polling(true);
    //window.set_scroll_polling(true);

    // gl: load all OpenGL function pointers
    // ---------------------------------------
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    while !window.should_close() {
        // events
        // -----
        //process_events(&mut window, &events);

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        //window.swap_buffers();
        glfw.poll_events();
    }

    Game {
      width, 
      height, 
      state: GameState::GameMenu
    }
  }
}