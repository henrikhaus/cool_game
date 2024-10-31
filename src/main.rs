mod renderer; // Import the renderer module

use renderer::Renderer; // Bring Renderer into scope
use sdl2::event::Event;
use std::time::Duration;
use sdl2::sys::{SDL_SetRelativeMouseMode};
use sdl2::sys::SDL_bool::SDL_TRUE;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
        .window("3D Game", 800, 600)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as _);

    let (width, height) = window.drawable_size();
    let aspect_ratio = width as f32 / height as f32;
    let mut renderer = Renderer::new(aspect_ratio);

    let mut event_pump = sdl.event_pump().unwrap();

    unsafe {
        SDL_SetRelativeMouseMode(SDL_TRUE);
    }

    'running: loop {
        // Handle quit event
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'running;
            }
            // Capture mouse movement for rotation
            if let Event::MouseMotion { xrel, yrel, .. } = event {
                let xoffset = xrel as f32;
                let yoffset = -yrel as f32; // Invert y to match typical camera rotation behavior
                renderer.camera.rotate(xoffset, yoffset);
            }
        }

        // Get the current state of the keyboard
        let keyboard_state = event_pump.keyboard_state();

        // Continuous movement based on key state
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::W) {
            renderer.camera.move_forward();
        }
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::S) {
            renderer.camera.move_backward();
        }
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::A) {
            renderer.camera.strafe_left();
        }
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::D) {
            renderer.camera.strafe_right();
        }

        // Clear, render, and update the screen
        renderer.clear();
        renderer.render();
        window.gl_swap_window();

        // Frame delay
        std::thread::sleep(Duration::from_millis(16)); // ~60 FPS
    }
}
