use super::color::Color;

#[allow(dead_code)]
pub struct Film {
    pub width: usize,
    pub height: usize,
    pub frame_buffer: Vec<u8>
}

// impl<Vec<u8>> std::ops::DerefMut for Film {
//     type Target = Vec<u8>;
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.frame_buffer
//     }
// }

impl Film {
    /// Construct film
    pub fn new(width: usize,height: usize) -> Film {
        Film {
            width,
            height,
            // frame_buffer: Vec::with_capacity(width*height*4).fill(0)
            frame_buffer: std::iter::repeat(0).take(width*height*4).collect()
        }
    }

    /// Clear the film to given RGBA color
    pub fn clear(&mut self,color: Color) {
        let r: u8 = (color.r * 255.) as u8;
        let g: u8 = (color.g * 255.) as u8;
        let b: u8 = (color.b * 255.) as u8;
        let a: u8 = (color.a * 255.) as u8;

        let test: [u8; 4] = [r,g,b,a];
        let mut index: usize = 0;
        
        for elem in self.frame_buffer.iter_mut() {

            *elem = test[index];
            index = (index + 1) % 4;
        }
    }

    /// Write RGBA color to the framebuffer
    /// (x,y) => x*width + y
    pub fn write_pixel(&mut self,x: usize,y: usize,color: Color) -> Result<(),String> {
        // make sure (x,y) is not out of range
        if x >= self.height || y >= self.width {
            return Err(format!("({},{}) is an invalid range",x,y))
        }

        let index: usize = (x*4*self.width) + (y*4);

        //
        self.frame_buffer[ index ] = (color.r*255.) as u8;
        self.frame_buffer[index+1] = (color.b*255.) as u8;
        self.frame_buffer[index+2] = (color.b*255.) as u8;
        self.frame_buffer[index+3] = (color.a*255.) as u8;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // should construct correctly
    fn test_new() {
        let film : Film = Film::new(800,600);
        assert_eq!(film.width,800);
        assert_eq!(film.height,600);
        assert_eq!(film.frame_buffer.len(),800*600*4);
        let size: usize = film.width * film.height * 4;
        for i in 0..size {
            assert_eq!(film.frame_buffer[i],0);
        }
    }

    #[test]
    // should clear correctly
    fn test_clear() {
        let mut film : Film = Film::new(800,600);
        let color: Color = Color::new(0.0,0.5,0.75,1.0).unwrap();
        film.clear(color);

        let mut index: usize = 0;
        for _ in film.frame_buffer.iter() {

            if index == film.frame_buffer.len() {
                break;
            }

            let r = (color.r * 255.) as u8;
            let g = (color.g * 255.) as u8;
            let b = (color.b * 255.) as u8;
            let a = (color.a * 255.) as u8;
            assert_eq!(film.frame_buffer[ index ],r);
            assert_eq!(film.frame_buffer[index+1],g);
            assert_eq!(film.frame_buffer[index+2],b);
            assert_eq!(film.frame_buffer[index+3],a);
            index += 4;
        }
    }

    #[test]
    // should write pixel to framebuffer correctly
    fn test_write_pixel() {
        //   0   1   2   3 |   4   5   6   7 |   8   9  10  11 |  12  13  14  15 |  16  17  18  19 |  20  21  22  23 |  24  25  26  27 |  28  29  30  31
        //  32  33  34  35 |  36  37  38  39 |  40  41  42  43 |  44  45  46  47 |  48  49  50  51 |  52  53  54  55 |  56  57  58  59 |  60  61  62  63
        //  64  65  66  67 |  68  69  70  71 |  72  73  74  75 |  76  77  78  79 |  80  81  82  83 |  84  85  86  87 |  88  89  90  91 |  92  93  94  95
        //  96  97  98  99 | 100 101 102 103 | 104 105 106 107 | 108 109 110 111 | 112 113 114 115 | 116 117 118 119 | 120 121 122 123 | 124 125 126 127
        // 128 129 130 131 | 132 133 134 135 | 136 137 138 139 | 140 141 142 143 | 144 145 146 147 | 148 149 150 151 | 152 153 154 155 | 156 157 158 159
        // 160 161 162 163 | 164 165 166 167 | 168 169 170 171 | 172 173 174 175 | 176 177 178 179 | 180 181 182 183 | 184 185 186 187 | 188 189 190 191

        let mut film: Film = Film::new(8,6);
        let mut color: Color = Color::new(1.,0.,0.,1.).unwrap();

        film.write_pixel(0,0,color).unwrap();
        assert_eq!(film.frame_buffer[0],255);
        assert_eq!(film.frame_buffer[1],0);
        assert_eq!(film.frame_buffer[2],0);
        assert_eq!(film.frame_buffer[3],255);

        film.write_pixel(0,1,color).unwrap();
        assert_eq!(film.frame_buffer[4],255);
        assert_eq!(film.frame_buffer[5],0);
        assert_eq!(film.frame_buffer[6],0);
        assert_eq!(film.frame_buffer[7],255);

        film.write_pixel(0,2,color).unwrap();
        assert_eq!(film.frame_buffer[8],255);
        assert_eq!(film.frame_buffer[9],0);
        assert_eq!(film.frame_buffer[10],0);
        assert_eq!(film.frame_buffer[11],255);

        film.write_pixel(3,5,color).unwrap();
        assert_eq!(film.frame_buffer[116],255);
        assert_eq!(film.frame_buffer[117],0);
        assert_eq!(film.frame_buffer[118],0);
        assert_eq!(film.frame_buffer[119],255);

        film.write_pixel(5,7,color).unwrap();
        assert_eq!(film.frame_buffer[188],255);
        assert_eq!(film.frame_buffer[189],0);
        assert_eq!(film.frame_buffer[190],0);
        assert_eq!(film.frame_buffer[191],255);

        film.write_pixel(5,2,color).unwrap();
        assert_eq!(film.frame_buffer[168],255);
        assert_eq!(film.frame_buffer[169],0);
        assert_eq!(film.frame_buffer[170],0);
        assert_eq!(film.frame_buffer[171],255);
        //
    }
}
