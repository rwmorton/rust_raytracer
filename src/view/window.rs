// https://github.com/Rust-SDL2/rust-sdl2
// https://rust-sdl2.github.io/rust-sdl2/sdl2/index.html

use sdl2::{
    {Sdl,VideoSubsystem},
    video,
    render::{Canvas,TextureCreator,Texture},
    event::Event,
    keyboard::Keycode
};

use std::time::Duration;

use crate::{
    image::{film::Film,color::Color},
    scene::{world::*,sphere::*,plane::*},
    math::{point::*,ray::*,vector::*,normal::*}
};

use rand::Rng;

#[allow(dead_code)]
pub struct Window<'a> {
    width: u32,
    height: u32,
    context: Sdl,
    video: VideoSubsystem,
    canvas: Canvas<video::Window>,
    film: &'a mut Film,
    // texture_creator: TextureCreator<video::WindowContext>,
    // texture: Texture<'a>
}

impl<'a> Window<'a> {
    /// Construct window
    pub fn new(title: String,width: u32,height: u32,film: &'a mut Film) -> Window<'a> {
        //
        let context = sdl2::init().unwrap();
        let video = context.video().unwrap();
        let window: video::Window = video
            .window(title.as_str(),width,height)
            .position_centered()
            .build()
            .expect("Failed to initialize SDL2 video subsystem");
        let canvas = window
            .into_canvas()
            .build()
            .expect("Failed to make SDL2 canvas");

        // // set up texture
        // let texture_creator = canvas.texture_creator();
        // let texture = texture_creator
        //     .create_texture(
        //         sdl2::pixels::PixelFormatEnum::ARGB8888,
        //         sdl2::render::TextureAccess::Streaming,
        //         width as u32,
        //         height as u32
        //     )
        //     .expect("Couldn't create SDL2 texture");

        Window {
            width,height,
            context,video,canvas,
            film,
            // texture_creator,texture
        }
    }

    /// Main loop
    pub fn run(mut self) -> Result<(),String> {

        // set up the world
        let mut world = World::new(1);
        let mut s_center = Point::new(0.,0.,1.);
        let mut s_radius = 0.5;
        let mut sphere = Sphere::new(s_radius,s_center);
        world.add_primitive(Box::new(sphere));

        let plane = Plane::new(
            &Point::new(0.,-1.,5.),&
            Normal::new(0.,1.,0.25)
        );
        world.add_primitive(Box::new(plane));

        // ray initialized at origin, pointing in positive Z
        let mut ray: Ray = Ray::new(&Point::new(0.,0.,0.),&Vector::new(0.,0.,1.));
        let mut hit_color = Color::new(1.,0.,0.,1.).unwrap();
        let mut SPEED = 0.01;

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
        
        let mut prev = std::time::Instant::now();
        let mut cur;

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

            let color: Color = Color::new(0.,0.,0.,1.).unwrap();
            self.film.clear(crate::image::color::BLACK);

            // raytrace!
            for i in 0..self.width {
                for j in 0..self.height {
                    let (x,y) = self.film.ndc(i,j);
                    // we only need to adjust ray origin's x and y coord
                    ray.o.x = x;
                    ray.o.y = y;
                    let result = world.hit(&ray);
                    match result {
                        Some(x) => {
                            // got a hit!
                            let mut v = x - s_center;
                            hit_color.r = (v.x * 0.5) + 0.5;
                            hit_color.g = (v.y * 0.5) + 0.5;
                            hit_color.b = (v.z * 0.5) + 0.5;
                            self.film.write_pixel(i as usize,j as usize,hit_color);
                        },
                        None => {
                            // do nothing
                        }
                    }
                }
            }

            // let mut switch: bool = false;

            // for i in 0..self.width {
            //     for j in 0..self.height {
            //         if switch {
            //             self.film.write_pixel(i as usize,j as usize,hit_color);
            //         } else {
            //             let col = Color::new(1.,1.,0.,1.).unwrap();
            //             self.film.write_pixel(i as usize,j as usize,col);
            //         }
            //     }
            //     switch = !switch;
            // }

            self.canvas.copy(&texture,None,None);

            if s_center.x >= (1. - s_radius) {
                SPEED *= -1.;
            } else if s_center.x <= -(1. - s_radius) {
                SPEED *= -1.;
            }
            s_center.x += SPEED;
            world.primitives[0] = Box::new(Sphere::new(s_radius,s_center));

            // self.update();
            self.render();

            cur = std::time::Instant::now();
            let elapsed = cur - prev;
            prev = cur;
            let ms = elapsed.as_millis();
            println!("cast {} rays at {} ms per frame ~ {} fps",self.width*self.height,ms,f64::round(1000. / (ms as f64)));

            // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }

        Ok(())
    }

    /// Update
    fn update(&mut self) {
        let r = rand::thread_rng().gen();
        let g = rand::thread_rng().gen();
        let b = rand::thread_rng().gen();
        let col = Color::new(r,g,b,1.).unwrap();
        self.film.clear(col);
    }

    /// Render
    fn render(&mut self) {
        self.canvas.present();
    }
}
