mod math;
mod scene;
mod view;
mod image;

use view::window::Window;
use crate::image::film::Film;

fn main() {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 600;
    let mut film: Film = Film::new(WIDTH,HEIGHT);
    let window: Window = Window::new("Rust Raytracing Demo".to_string(),WIDTH as u32,HEIGHT as u32,&mut film);
    window.run().unwrap();
}
