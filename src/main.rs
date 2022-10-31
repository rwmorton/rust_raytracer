use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{WindowCanvas, Texture};
use std::time::Duration;

fn main() -> Result<(), String> {
    const width: usize = 800;
    const height: usize = 600;

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Raytracer Demo",width as u32,height as u32)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator
        .create_texture(
            sdl2::pixels::PixelFormatEnum::ARGB8888,
            sdl2::render::TextureAccess::Streaming,
            width as u32,
            height as u32
        )
        .expect("Couldn't make texture.");

    //
    // RALPH
    //
    let mut fb: [u8; width*height*4] = [0; width*height*4];

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }

        // Update frame buffer
        for j in 0..(width*height*4) {
            i = (i + 1) % 255;
            //
            fb[j] = i;
        }
        texture.update(
            sdl2::rect::Rect::new(0,0,width as u32,height as u32),
            unsafe {
                &fb.align_to().1
            },
            32*width
        )
        .expect("Couldn't copy framebuffer to texture");
        
        canvas.copy(&texture,None,None);

        // Render
        // canvas.set_draw_color(Color::RGBA(0,0,0,255));
        canvas.clear();
        canvas.present();

        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}


// mod view;
// use view::window::Window;

// fn main() {
//     let window: Window = Window::new(800,600);
//     window.run().unwrap();
// }
