use image::{
    GenericImage,
    GenericImageView,
    ImageBuffer,
    RgbImage
};

mod math;
mod _image;

use math::vector::*;
use math::matrix::*;
use _image::color::*;

fn print_row(r: (f64,f64,f64,f64)) {
    println!("({},{},{},{})",r.0,r.1,r.2,r.3);
}

fn main() {
    // test image
    let imgx = 800;
    let imgy = 800;

    let scalex = 3.0 / imgx as f32;
    let scaley = 3.0 / imgy as f32;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    // A redundant loop to demonstrate reading image data
    for x in 0..imgx {
        for y in 0..imgy {
            let cx = y as f32 * scalex - 1.5;
            let cy = x as f32 * scaley - 1.5;

            let c = num_complex::Complex::new(-0.4, 0.6);
            let mut z = num_complex::Complex::new(cx, cy);

            let mut i = 0;
            while i < 255 && z.norm() <= 2.0 {
                z = z * z + c;
                i += 1;
            }

            let pixel = imgbuf.get_pixel_mut(x, y);
            let image::Rgb(data) = *pixel;
            *pixel = image::Rgb([data[0], i as u8, data[2]]);
        }
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("./temp/fractal.png").unwrap();

    // {
    //     // test Color
    //     let c: Color = Color::new(0.1,0.2,0.3,10.5);
    // }
    // let test: Vector = Vector::new(0.,0.,0.);
    // test.print();
    // let test2: Vector = Default::default();
    // test2.print();

    // let a = Vector::new(1.,2.,3.);
    // a.print();
    // let a_norm = a.normalize();
    // a_norm.print();
    // println!("a_norm length squared = {}",a_norm.len_sq());
    // println!("a_norm length = {}",a_norm.len());

    // let x = X_AXIS;
    // let y = Y_AXIS;
    // let cross = x.cross(&y);
    // x.print();
    // print!(" x ");
    // y.print();
    // print!(" = ");
    // cross.print();
    // println!("");

    // {
    //     let v: Vector = Vector{x: 2.,y: 8.,z: 0.5};
    //     v.print();
    //     println!("length = {}",v.len());
    // }

    // {
    //     let a = Vector::new(1.,5.,-3.);
    //     assert!(a.dot(&a) == a.len_sq());
    // }

    // println!("");
    // println!("-------------------------------------");
    // println!("");

    // {
    //     let mut a: i32 = 10;
    //     let mut b: i32 = 20;
    //     let mut p: &mut i32 = &mut a;
    //     println!("{} ",*p);
    //     *p += 1;
    //     println!("{} ",*p);
    //     p = &mut b;
    //     println!("{} ",*p);
    //     *p += 1;
    //     println!("{} ",*p);
    // }

    // {
    //     let mut m: Matrix = Matrix::new();
    //     m.print();
    //     println!("");
    //     let mut n: Matrix = m.scale(3.);
    //     n.print();
    //     let mut mxp: Matrix = m.mul(&n);
    //     println!("");
    //     mxp.print();

    //     println!("");
    //     m = Matrix::new().scale(5.0);
    //     n = Matrix::new().scale(1.0/5.0);
    //     mxp = m.mul(&n);
    //     m.print();
    //     println!("");
    //     n.print();
    //     println!("");
    //     mxp.print();
    //     println!("");

    //     // test transpose
    //     m.set
    //     (
    //         (1., 2., 3., 4.),
    //         (5., 6., 7., 8.),
    //         (9., 10., 11., 12.),
    //         (13., 14., 15., 16.)
    //     );
    //     m.print();
    //     println!("");
    //     m = m.transpose();
    //     m.print();

    //     // test get rows
    //     m = m.transpose();
    //     println!("printing rows:");
    //     let row1 = m.get_row(0);
    //     print_row(row1);
    //     let row2 = m.get_row(1);
    //     print_row(row2);
    //     let row3 = m.get_row(2);
    //     print_row(row3);
    //     let row4 = m.get_row(3);
    //     print_row(row4);

    //     // test get rows as vectors
    // }
}
