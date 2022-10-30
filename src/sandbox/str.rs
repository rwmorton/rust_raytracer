use std::mem::*;

pub fn test1() {
    let s1: &str = "";
    let s2: &str = "Hello";
    let s3: &str = "Hèllo";
    println!("{} {} {}",size_of_val(s1),size_of_val(s2),size_of_val(s3));
    println!("{} {} {}",size_of_val(&s1),size_of_val(&s2),size_of_val(&s3));
    println!("{} {} {}",size_of_val(&&s1),size_of_val(&&s2),size_of_val(&&s3));
}

pub fn test2() {
    let mut s1 = "".to_string();
    s1.push('e');
    let mut s2 = "".to_string();
    s2.push('è');
    let mut s3 = "".to_string();
    s3.push('€');
    println!("s1: {} {}",s1.capacity(),s1.len());
    println!("s2: {} {}",s2.capacity(),s2.len());
    println!("s3: {} {}",s3.capacity(),s3.len());

    // s1 = format!("{}{}",s1,"aaaaaaa");
    s1.push_str("aaaaaaa");
    println!("s1: {} {}",s1.capacity(),s1.len());
    s1.push('x');
    println!("s1: {} {}",s1.capacity(),s1.len());
}
