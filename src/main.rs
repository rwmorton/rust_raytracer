mod math;

use math::vector::*;

fn main() {
    let mut a: Vector = Vector {x: 1.,y: 2.,z: 3.,w: 0.};
    let mut b: Vector = Vector {x: 0.,y: 1.,z: 0.,w: 0.};
    println!("dot product = {}",dot(&a,&b));
    let sum: Vector = add(&a,&b);
    println!("(x,y,z,w) = ({},{},{},{})",sum.x,sum.y,sum.z,sum.w);

    a = Vector{x: 1.,y: 0.,z: 0.,w: 0.};
    b = Vector{x: 0.,y: 1.,z: 0.,w: 0.};

    let c: Vector = cross(&a,&b);

    println!("cross = ({}, {}, {}, {})",c.x,c.y,c.z,c.w);

    let d: Vector = Vector{x: 2.,y: 2.,z: 2.,w: 0.};
    let d_norm: Vector = normalize(&d);

    print_vector(&d_norm);

    println!("length squared of d_norm = {}",len_sq(&d_norm));
    println!("length of d_norm = {}",len(&d_norm));
}
