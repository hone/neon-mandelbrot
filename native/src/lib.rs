#[macro_use]
extern crate neon;

mod mandelbrot;

use neon::vm::{Call, JsResult, Lock};
use neon::js::{JsInteger, JsNumber, Value};
use neon::js::binary::JsArrayBuffer;
use mandelbrot::Mandelbrot;

fn fetch_arg<'a, T: Value>(call: &mut Call<'a>, index: i32) -> JsResult<'a, T> {
    call.arguments.require(call.scope, index)?.check::<T>()
}

fn mandelbrot(mut call: Call) -> JsResult<JsArrayBuffer> {
    let width = fetch_arg::<JsInteger>(&mut call, 0)?.value() as usize;
    let height = fetch_arg::<JsInteger>(&mut call, 1)?.value() as usize;
    let max_iterations = fetch_arg::<JsInteger>(&mut call, 2)?.value() as usize;
    let scaling_factor = fetch_arg::<JsNumber>(&mut call, 3)?.value() as f32;
    let pan_x = fetch_arg::<JsNumber>(&mut call, 4)?.value() as f32;
    let pan_y = fetch_arg::<JsNumber>(&mut call, 5)?.value() as f32;

    let buffer_size = (2 * width * height) as u32;
    let mut image = JsArrayBuffer::new(call.scope, buffer_size)?;
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
