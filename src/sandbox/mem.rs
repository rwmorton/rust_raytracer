pub fn test1(p: &f64) {
    let a = Box::new(*p);
    {
        let b = Box::new([1,2,3]);
        println!("{} {:?}",*a,*b);
    }
    let c = Box::new(true);
    println!("{} {}",a,c);
}

pub fn test2() {
    let a = 7;
    let a_box: Box<i32>;
    let mut a_ref: &i32 = &a;
    print!("{} {};",a,*a_ref);
    a_box = Box::new(a + 2);
    a_ref = &*a_box;
    println!(" {} {} {}",a,*a_ref,*a_box);
}
