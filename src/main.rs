extern crate sdl2;
extern crate gl;

mod renderer;
mod game;
mod utils;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{Duration, Instant};

fn main() {
    // Initialize SDL2
    let sdl_context = sdl2::init().expect("SDL2 initialization failed!");
    let video_subsystem = sdl_context.video().expect("Couldn't initialize video subsystem.");

    // Create an SDL2 window with OpenGL context
    let window = video_subsystem
        .window("3D Game", 800, 600)
        .opengl()
        .build()
        .expect("Failed to create a window");

    let _gl_context = window.gl_create_context().expect("Couldn't create GL context");
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

    // Set up the renderer
    let mut renderer = renderer::Renderer::new();

    // Set up the game state
    let mut game = game::Game::new();

    // Event pump for handling input events
    let mut event_pump = sdl_context.event_pump().expect("Failed to get SDL event pump.");

    let mut last_frame = Instant::now();

    // Main game loop
    'running: loop {
        let delta_time = last_frame.elapsed().as_secs_f32();
        last_frame = Instant::now();

        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }

        // Update game state
        game.update(delta_time);

        // Clear the screen
        renderer.clear();

        // Render the game
        renderer.render(&game);

        // Swap buffers
        window.gl_swap_window();

        // Cap the frame rate
        let frame_duration = Duration::new(0, 16_666_667); // Roughly 60 FPS
        if Instant::now().duration_since(last_frame) < frame_duration {
            std::thread::sleep(frame_duration - Instant::now().duration_since(last_frame));
        }
    }
}
