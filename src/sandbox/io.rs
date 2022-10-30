use std::io::Write;

pub fn test1() {
    std::io::stdout().write("Hello world!\n".as_bytes()).unwrap();

    println!("{}","true".parse::<bool>().unwrap());
    println!("{}","1.23456e7".parse::<f64>().unwrap());
}
