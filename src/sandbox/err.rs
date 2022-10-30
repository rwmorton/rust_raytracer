fn f1(x: u8) -> Result<u8,String> {
    if x == 1 {
        Err(format!("Err: 1"))
    } else {
        Ok(x)
    }
}
fn f2(x: u8) -> Result<u8,String> {
    if x == 2 {
        Err(format!("Err: 2"))
    } else {
        Ok(x)
    }
}
fn f3(x: u8) -> Result<u8,String> {
    if x == 3 {
        Err(format!("Err: 3"))
    } else {
        Ok(x)
    }
}
fn f4(x: u8) -> Result<u8,String> {
    if x == 4 {
        Err(format!("Err: 4"))
    } else {
        Ok(x)
    }
}

fn f(x: u8) -> Result<u8,String> {
    let result1 = f1(x);
    if result1.is_err() {
        return result1;
    }
    let result2 = f2(result1.unwrap());
    if result2.is_err() {
        return result2;
    }
    let result3 = f3(result2.unwrap());
    if result3.is_err() {
        return result3;
    }
    f4(result3.unwrap())
}

fn f_eq(x: u8) -> Result<u8,String> {
    f4(f3(f2(f1(x)?)?)?)
}

pub fn test1() {
    println!("{}",f(11).unwrap());
    println!("{}",f_eq(21).unwrap());
}

fn g1(n: i32) -> Result<i64,String> {
    Ok(g2(n * 2)? as i64 * 3)
}

fn g2(n: i32) -> Result<i32,String> {
    if n >= 0 {
        Ok(n * 5)
    } else {
        Err("Negative argument".to_string())
    }
}

pub fn test2() {
    println!("{:?}",g1(10));
    println!("{:?}",g1(7));
    println!("{:?}",g1(-1));
}
