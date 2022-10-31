// https://github.com/Rust-SDL2/rust-sdl2
// https://rust-sdl2.github.io/rust-sdl2/sdl2/index.html

use sdl2::{Sdl,VideoSubsystem};
use sdl2::video;
use sdl2::render::{Canvas,Texture};
// use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::time::Duration;

use crate::image::film::Film;
use crate::image::color::Color;

static WINDOW_TITLE: &str = "Rust Raytracing Demo";

#[allow(dead_code)]
pub struct Window {
    width: u32,
    height: u32,
    context: Sdl,
    video: VideoSubsystem,
    canvas: Canvas<video::Window>,
    //
    film: std::rc::Rc<Film>,
    //
    // TEMP
    i: u8
}

impl Window {
    /// Construct window
    pub fn new(width: u32,height: u32,film: Film) -> Window {
        //
        let context = sdl2::init().unwrap();
        let video = context.video().unwrap();
        let window: video::Window = video
            .window(WINDOW_TITLE,width,height)
            .position_centered()
            .build()
            .expect("Failed to initialize SDL2 video subsystem");
        let canvas = window
            .into_canvas()
            .build()
            .expect("Failed to make SDL2 canvas");

        Window {
            width,height,
            context,video,canvas,
            //
            film: std::rc::Rc::new(film),
            //
            // TEMP
            i: 0
        }
    }

    /// Main loop
    pub fn run(mut self) -> Result<(),String> {

        // set up texture
        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture(
                sdl2::pixels::PixelFormatEnum::ARGB8888,
                sdl2::render::TextureAccess::Streaming,
                self.width as u32,
                self.height as u32
            )
            .expect("Couldn't create SDL2 texture");

        let mut event_pump = self.context.event_pump()?;
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} | Event::KeyDown {
                        keycode: Some(Keycode::Escape),..
                    } => {
                        break 'running;
                    },
                    _ => {}
                }
            }

            texture.update(
                sdl2::rect::Rect::new(0,0,self.width as u32,self.height as u32),
                unsafe {
                    &self.film.frame_buffer.align_to().1
                },
                4*self.width as usize
            )
            .expect("Coudln't copy framebuffer to texture");

            let color: Color = Color::new(0.5,0.5,0.5,1.).unwrap();
            // self.film.clear(color) ; //.unwrap();

            self.canvas.copy(&texture,None,None);

            self.canvas.present();

            self.update();
            // self.render();

            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }

        Ok(())
    }

    /// Update
    fn update(&mut self) {
        //
        // TEMP
        self.i = (self.i + 1) % 255;
    }

    /// Render
    fn render(&mut self) {
        //
        // TEMP
        // self.canvas.set_draw_color(Color::RGB(self.i, 64, 255 - self.i));
        self.canvas.clear();

        self.canvas.present();
    }
}
