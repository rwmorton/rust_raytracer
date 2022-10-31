// enums
pub fn test1() {
    //
    enum Shape {
        Rectangle { width: f32, height: f32 },
        Triangle { side: f32 },
        Circle { radius: f32 }
    }

    impl Shape {
        pub fn perimeter(&self) -> f32 {
            match self {
                Shape::Rectangle { width, height } => width * 2.0 + height * 2.0,
                Shape::Triangle { side } => side * 3.0,
                Shape::Circle { radius } => radius * 2.0 * std::f32::consts::PI
            }
        }

        pub fn area(&self) -> f32 {
            match self {
                Shape::Rectangle { width, height } => width * height,
                Shape::Triangle { side } => 0.5 * 3.0_f32.sqrt() / 2.0 * side,
                Shape::Circle { radius } => radius * radius * std::f32::consts::PI
            }
        }
    }

    fn print_area(shape: Shape) {
        println!("{}",shape.area());
    }

    fn print_perimeters(shapes: Vec<Shape>) {
        for shape in shapes.iter() {
            println!("{}",shape.perimeter());
        }
    }

    // test
    let c: Shape = Shape::Circle{radius: 1.0 };
    print_area(c);

    let mut shapes: Vec<Shape> = Vec::new();
    shapes.push(Shape::Circle{radius: 2.0});
    shapes.push(Shape::Triangle{side: 1.0});
    shapes.push(Shape::Rectangle{width: 2.0,height: 3.0});
    print_perimeters(shapes);
}

// traits
pub fn test2() {
    trait Animal {
        fn talk(&self);
    }

    struct Dog;
    struct Cat;

    impl Animal for Dog {
        fn talk(&self) {
            println!("I am a dog");
        }
    }

    impl Animal for Cat {
        fn talk(&self) {
            println!("I am a cat");
        }
    }

    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Dog{}),Box::new(Cat{})];
    for animal in animals.iter() {
        animal.talk();
    }

    //
}
