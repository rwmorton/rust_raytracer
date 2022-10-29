mod math;

use math::vector::*;
use math::matrix::*;

fn main() {
    let test: Vector = Vector::new(0.,0.,0.);
    test.print();
    let test2: Vector = Default::default();
    test2.print();

    let a = Vector::new(1.,2.,3.);
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

    {
        let mut v: Vector = Vector{x: 2.,y: 8.,z: 0.5};
        v.print();
        normalize(&mut v);
        v.print();
        println!("length = {}",v.len());
    }

    {
        let a = Vector::new(1.,5.,-3.);
        assert!(a.dot(&a) == a.len_sq());
    }

    println!("");
    println!("-------------------------------------");
    println!("");

    {
        let mut a: i32 = 10;
        let mut b: i32 = 20;
        let mut p: &mut i32 = &mut a;
        println!("{} ",*p);
        *p += 1;
        println!("{} ",*p);
        p = &mut b;
        println!("{} ",*p);
        *p += 1;
        println!("{} ",*p);
    }

    {
        let m: Matrix = Matrix::new();
        m.print();
    }
}
