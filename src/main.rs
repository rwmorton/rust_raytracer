mod math;

use math::vector::*;

fn main() {
    let test: Vector = Vector::new();
    test.print();
    let test2: Vector = Default::default();
    test2.print();

    let a = Vector::xyz(1.,2.,3.);
    a.print();
    let a_norm = a.normalize();
    a_norm.print();
    println!("a_norm length squared = {}",a_norm.len_sq());
    println!("a_norm length = {}",a_norm.len());

    let x = X_AXIS;
    let y = Y_AXIS;
    let cross = x.cross(&y);
    x.print();
    print!(" x ");
    y.print();
    print!(" = ");
    cross.print();
    println!("");
}
