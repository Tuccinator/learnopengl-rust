extern crate glfw;
extern crate gl;

use glfw::{Action, Key, Context, WindowHint, OpenGlProfileHint, WindowMode, Window};

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(WindowHint::ContextVersion(3, 3));
    glfw.window_hint(WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

    let (mut window, _events) = glfw.create_window(800, 600, "LearnOpenGL", WindowMode::Windowed).expect("Failed to create GLFW window.");
    
    gl::load_with(|s| window.get_proc_address(s) as *const std::os::raw::c_void);

    glfw.make_context_current(Some(&window));
    
    unsafe {
        gl::Viewport(0, 0, 800, 600);
    }

    window.set_framebuffer_size_polling(true);


    // render loop  
    loop {
        if window.should_close() {
            break;
        }

        process_input(&mut window);

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.swap_buffers();
        glfw.poll_events();
    }
}

fn process_input(window: &mut Window) {
    if window.get_key(Key::Escape) == Action::Press {
        window.set_should_close(true);
    }
}