mod math;
use math::vector::Vector;
use math::point::Point;
use math::ray::Ray;

mod film;
mod sandbox;

fn main() {
    // sandbox::img::gen_fractal();

    sandbox::mem::test1(&3.4);
    sandbox::mem::test2();
    println!("\n-----------------\n");
    sandbox::data::test1();
    println!("\n-----------------\n");
    sandbox::closure::test1();
    sandbox::closure::test2();
    sandbox::closure::test3();
    println!("\n-----------------\n");
    sandbox::str::test1();
    sandbox::str::test2();
    println!("\n-----------------\n");
    sandbox::ranges::test1();
    println!("\n-----------------\n");
    sandbox::iter::test1();
    println!("\n-----------------\n");
    sandbox::err::test1();
    sandbox::err::test2();
    println!("\n-----------------\n");
    sandbox::io::test1();

    // test display trait
    let v = math::vector::Vector::new(1.,2.,3.);
    println!("vector = {}",v);
    let o = Point::new(1.,2.,3.);
    let d = Vector::new(-3.,-1.,2.);
    let r = Ray::new(&o,&d);
    println!("ray = {}",r);
}
