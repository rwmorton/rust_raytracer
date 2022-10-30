fn min(arr: &[i8]) -> Result<i8,&str> {
    if arr.len() == 0 {
        Err("Array is empty!")
    } else {
        let mut min: i8 = arr[0];
        for i in 1..arr.len() {
            if arr[i] < min {
                min = arr[i];
            }
        }
        Ok(min)
    }
}

pub fn test1() {
    let arr: [i8; 8] = [9,8,3,10,12,-4,-7,0];
    println!("{:?}, smallest = {}",arr,min(&arr).unwrap());
    // println!("{:?}, smallest = {}",arr,min(&[]).unwrap());
}
