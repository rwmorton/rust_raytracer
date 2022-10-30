use std::cmp::Ordering;

pub fn test1() {
    let mut arr = [1,99,2,33,0,-33,100,108,-19];
    println!("{:?}",arr);

    fn desc(a: &i32,b: &i32) -> Ordering {
        if a < b {
            Ordering::Greater
        } else if a > b {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }

    arr.sort_by(desc);

    println!("{:?}",arr);
}

pub fn test2() {
    let mut arr = [-44,-33,-22,-11,1,11,22,33,44,55,66,77,88,99,109];

    println!("{:?}",arr);

    arr.sort_by(|a: &i32,b: &i32| -> Ordering
    {
        if a < b { Ordering::Greater }
        else if a > b { Ordering::Less }
        else { Ordering::Equal }
    });

    println!("{:?}",arr);
}

pub fn test3() {
    let mut arr = [-23,232,-23,4535,91243,112,-33,0,33];
    println!("{:?}",arr);
    arr.sort_by(|a,b| b.cmp(a));
    println!("{:?}",arr);
}
