extern crate ndarray;
use self::ndarray::{Array2, IxDyn};

pub struct Mandelbrot {
    width: usize,
    height: usize,
}

impl Mandelbrot {
    const RADIUS: usize = 2;

    pub fn new(width: usize, height: usize) -> Self {
        Mandelbrot {
            width: width,
            height: height,
        }
    }

    // neon JsArrayType only grabs in u8 views, so need to use u8
    pub fn generate(&self, max_iterations: usize, scaling_factor: f32, pan_x: f32, pan_y: f32) -> Array2<u8> {
        let mut result = Array2::zeros([self.width, self.height]);
        for x in 0..self.width {
            for y in 0..self.height {
                let c_re = (x as f32 / scaling_factor) - pan_x;
                let c_im = (y as f32 / scaling_factor) - pan_y;
                if self.belongs(c_re, c_im, max_iterations) {
                    result[[x, y]] = 1 as u8;
                } else {
                    result[[x, y]] = 0 as u8;
                }
            }
        }

        result
    }

    fn belongs(&self, c_re: f32, c_im: f32, max_iterations: usize) -> bool {
        let mut iteration = 0;
        let mut x: f32 = 0.0;
        let mut y: f32 = 0.0;

        while iteration < max_iterations && x * x + y * y <= (Self::RADIUS * Self::RADIUS) as f32 {
            let x_new = (x * x - y * y) as f32 + c_re;
            y = (2.0 * x * y) as f32 + c_im;
            x = x_new;
            iteration += 1;
        }

        iteration < max_iterations
    }
}
