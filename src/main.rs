use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use std::time::Duration;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // build a window object
    let window = video_subsystem
        .window("init game dev", 800, 800)
        .position_centered()
        .build()
        .unwrap();

    // create a drawing surface
    let mut canvas = window.into_canvas().build().unwrap();

    // initialize a texture creator
    let creator = canvas.texture_creator();

    // create a texture for our rotating rectangle
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGB888, 400, 300)
        .unwrap();

    let mut red = 0; // set the initial value for our red color channel
    let mut angle = 0.0; // set the initial position of our rotation
    let mut event_pump = sdl_context.event_pump().unwrap();

    'game_loop: loop {
        // cycle through color spectrum
        red = (red + 1) % 255;

        // turn our rectangle by half a degree
        angle = (angle + 0.5) % 360.;

        // check for quit or esc key events
        // exit the game_loop if matched
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'game_loop;
                }
                _ => {}
            }
        }

        canvas
            // temporarily changes the canvas target to the referenced texture
            .with_texture_canvas(&mut texture, |texture_canvas| {
                texture_canvas.clear();
                texture_canvas.set_draw_color(Color::RGB(red, 64, 255 - red));
                texture_canvas
                    .fill_rect(Rect::new(50, 50, 400, 400))
                    .unwrap();
            })
            .unwrap();

        // set the initial color for the canvas
        canvas.set_draw_color(Color::RGB(0, 0, 0));

        // creates the destination rectangle to which our texture will be applied
        let dst = Rect::new(50, 50, 400, 400);
        canvas.clear();

        // not certain I fully understand this yet.
        canvas
            .copy_ex(&texture, None, dst, angle, None, false, false)
            .unwrap();

        // flushes the internal SDL "backbuffer" which renders any draw commands since its last execution
        canvas.present();

        // splits 60/1billion nanoseconds (slows the loop to set frame limit to 60/s)
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
