fn print_nth_char(s: &str,mut n: u32) {
    let mut iter: std::str::Chars = s.chars();
    loop {
        let item: Option<char> = iter.next();
        match item {
            Some(c) => if n == 0 {
                println!("{}",c);
                break;
            },
            None => {
                break;
            }
        }
        n -= 1;
    }
}

fn print_codes(s: &str) {
    // let mut iter = s.chars();
    // loop {
    //     match iter.next() {
    //         Some(c) => {
    //             println!("{}: {}",c,c as u32);
    //         },
    //         None => {
    //             break;
    //         }
    //     }
    // }

    for c in s.chars() {
        println!("{}: {}",c,c as u32);
    }
}

pub fn test1() {
    print_nth_char("€èe", 0);
    print_nth_char("€èe", 2);
    //
    print_codes("abcABC€èe");
}
