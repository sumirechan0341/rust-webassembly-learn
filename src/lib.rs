use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    console_error_panic_hook::set_once();

    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    draw_triangle(&context, [(300.0, 0.0), (0.0, 600.0), (600.0, 600.0)]);
    Ok(())
}

fn draw_triangle(context: &web_sys::CanvasRenderingContext2d, vertices: [(f64, f64); 3]) {
    context.begin_path();
    context.line_to(vertices[0].0, vertices[0].1);
    context.line_to(vertices[1].0, vertices[1].1);
    context.line_to(vertices[2].0, vertices[2].1);
    context.close_path();
    context.stroke();
    context.fill();
}
