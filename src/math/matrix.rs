//
// Force packing in a contiguous array of memory?
//
union DataPacking {
    flat: [f64; 16],
    multi: [[f64; 4]; 4]
}

//
// 4x4 matrix
//
// # Data storage
// TODO!
//
pub struct Matrix {
    pub data: DataPacking
}

pub fn add(a: &Matrix,b: &Matrix) -> Matrix {
    return Matrix {
        flat[0]  = a.flat[0]  + b.flat[0],
        flat[1]  = a.flat[1]  + b.flat[1],
        flat[2]  = a.flat[2]  + b.flat[2],
        flat[3]  = a.flat[3]  + b.flat[3],
        flat[4]  = a.flat[4]  + b.flat[4],
        flat[5]  = a.flat[5]  + b.flat[5],
        flat[6]  = a.flat[6]  + b.flat[6],
        flat[7]  = a.flat[7]  + b.flat[7],
        flat[8]  = a.flat[8]  + b.flat[8],
        flat[9]  = a.flat[9]  + b.flat[9],
        flat[10] = a.flat[10] + b.flat[10],
        flat[11] = a.flat[11] + b.flat[11],
        flat[12] = a.flat[12] + b.flat[12],
        flat[13] = a.flat[13] + b.flat[13],
        flat[14] = a.flat[14] + b.flat[14],
        flat[15] = a.flat[15] + b.flat[15],
    }
}
