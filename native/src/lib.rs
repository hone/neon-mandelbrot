#[macro_use]
extern crate neon;

mod mandelbrot;

use neon::vm::{Call, JsResult, Lock};
use neon::js::JsString;
use neon::js::binary::JsArrayBuffer;
use mandelbrot::Mandelbrot;

fn mandelbrot(call: Call) -> JsResult<JsArrayBuffer> {
    let scope = call.scope;
    let width = 600 as usize;
    let height = 600 as usize;
    let max_iterations = 10;
    let scaling_factor = 200.0;
    let pan_x = 2.0;
    let pan_y = 1.5;
    let buffer_size = (2 * width * height) as u32;
    let mut image = JsArrayBuffer::new(scope, buffer_size)?;
    let mandelbrot = Mandelbrot::new(width, height);
    let rust_image = mandelbrot.generate(max_iterations, scaling_factor, pan_x, pan_y);
    for x in 0..width {
        for y in 0..height {
            let index = x * width + y;
            image.grab(|mut slice| {
                slice[index] = rust_image[[x, y]];
            });
        }
    }

    Ok(image)
}

register_module!(m, {
    m.export("mandelbrot", mandelbrot)
});
