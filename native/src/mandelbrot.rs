extern crate time;
use self::time::PreciseTime;

pub struct Mandelbrot {
    width: usize,
    height: usize,
}

impl Mandelbrot {
    const RADIUS: f32 = (2 * 2) as f32;

    pub fn new(width: usize, height: usize) -> Self {
        Mandelbrot {
            width: width,
            height: height,
        }
    }

    pub fn generate(&self, max_iterations: usize, scaling_factor: f32, pan_x: f32, pan_y: f32) -> Vec<u8> {
        let start_time = PreciseTime::now();
        let result = (0..self.width * self.height).map(|index| {
            let x = index / self.width;
            let y = index - x * self.width;
            let c_re = (x as f32 / scaling_factor) - pan_x;
            let c_im = (y as f32 / scaling_factor) - pan_y;
            self.belongs(c_re, c_im, max_iterations)
        }).collect();
        let end_time = PreciseTime::now();
        eprintln!("Rust timing: {}", start_time.to(end_time));
        result
    }

    fn belongs(&self, c_re: f32, c_im: f32, max_iterations: usize) -> u8 {
        let mut x: f32 = 0.0;
        let mut y: f32 = 0.0;

        for iteration in 0..max_iterations {
            let x_new = (x * x - y * y) as f32 + c_re;
            y = (2.0 * x * y) as f32 + c_im;
            x = x_new;

            if x * x + y * y > Self::RADIUS {
                return ((iteration as f32 / max_iterations as f32) * 100.0) as u8
            }
        }

        0 as u8
    }
}
