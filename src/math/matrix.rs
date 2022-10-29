// Force packing in a contiguous array of memory?
union DataPacking {
    flat: [f64; 16],
    multi: [[f64; 4]; 4]
}

// 4x4 matrix
//
// # Data storage
// Stored in row major order.
pub struct Matrix {
    //pub data: DataPacking
    pub m: [f64; 16] // = [0.0; 16]
}

// helpful constants
const IDENTITY: Matrix = Matrix {
    m: [
        1.0,0.0,0.0,0.0,
        0.0,1.0,0.0,0.0,
        0.0,0.0,1.0,0.0,
        0.0,0.0,0.0,1.0
    ]
};

impl Matrix {
    pub fn new() -> Matrix {
        let mut m: Matrix = Matrix {m: [0.0; 16]};
        // row 1
        m.m[0] = 1.0;
        m.m[1] = 0.0;
        m.m[2] = 0.0;
        m.m[3] = 0.0;
        // row 2
        m.m[4] = 0.0;
        m.m[5] = 1.0;
        m.m[6] = 0.0;
        m.m[7] = 0.0;
        // row 3
        m.m[8] = 0.0;
        m.m[9] = 0.0;
        m.m[10] = 1.0;
        m.m[11] = 0.0;
        // row 4
        m.m[12] = 0.0;
        m.m[13] = 0.0;
        m.m[14] = 0.0;
        m.m[15] = 0.0;

        m // return
}

    // TODO: Add formatting.
    pub fn print(&self) {
        println!("| {} {} {} {} |",self.m[0],self.m[1],self.m[2],self.m[3]);
        println!("| {} {} {} {} |",self.m[4],self.m[5],self.m[6],self.m[7]);
        println!("| {} {} {} {} |",self.m[8],self.m[9],self.m[10],self.m[11]);
        println!("| {} {} {} {} |",self.m[12],self.m[13],self.m[14],self.m[15]);
    }
}
