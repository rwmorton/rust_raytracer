mod view;
use view::window::Window;

fn main() {
    let window: Window = Window::new(800,600);
    window.run().unwrap();
}
