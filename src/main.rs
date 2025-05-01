use graph1::core::context::{GraphContext, WindowContext};
use graph1::utils::color::adapters::rgba_to_0rgb;
use graph1::utils::color::palettes::RetroNeon;

use crate::user_data::DemoUserData;
use minifb::{Key, Window, WindowOptions};

mod bouncy;
mod user_data;

/// Width of the window, in pixels
const WIN_WIDTH: u32 = 480;
/// Height of the window, in pixels
const WIN_HEIGHT: u32 = 240;

/// Number of threads to use for rendering
const NUM_THREADS: usize = 4;

fn main() {
    let mut width = WIN_WIDTH as usize;
    let mut height = WIN_HEIGHT as usize;

    // The window context
    let win_ctx = WindowContext::new(
        WIN_WIDTH,
        WIN_HEIGHT,
        Some(RetroNeon::CYBER_BLUE),
        Some(RetroNeon::LASER_LIME),
    );

    // Output buffer to be passed to `minifb` for displaying on the screen
    let mut output_buf_0rgb: Vec<u32> = vec![win_ctx.background_color; win_ctx.get_num_pixels()];

    // Graph context
    let mut ctx: GraphContext<DemoUserData> =
        GraphContext::new(win_ctx, false, false, None, NUM_THREADS, None);

    // Configure `minifb` window
    let mut window_options: WindowOptions = WindowOptions::default();
    window_options.resize = true;
    window_options.scale_mode = minifb::ScaleMode::Center;

    let mut window = Window::new(
        "Graph1 + Minifb app",
        ctx.win.w_usize,
        ctx.win.h_usize,
        window_options,
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(60);

    // MAIN LOOP
    // **************
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Handle window resizing.
        // Check if the window size has changed.
        let (new_width, new_height) = window.get_size();

        if new_width != width || new_height != height {
            // Update dimensions and buffer
            width = new_width;
            height = new_height;
            ctx.resize(width as u32, height as u32);
            output_buf_0rgb.resize(ctx.win.get_num_pixels(), 0);
            println!("Window resized to: {}x{}", width, height);
        }

        // ===[ COLOR ADAPTER ]===========================
        bouncy::render_frame(&mut ctx);

        // ===[ COLOR ADAPTER ]===========================
        rgba_to_0rgb(
            &mut output_buf_0rgb,
            &mut ctx.frame_buf,
            ctx.num_threads,
            false,
        );

        // ===[ REDRAW THE MAIN WINDOW ]===========================
        window
            .update_with_buffer(&output_buf_0rgb, ctx.win.w_usize, ctx.win.h_usize)
            .unwrap();

        // increment the frame count
        ctx.frame_count += 1;
    }
}
