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
    sierpinsky(&context, [(300.0, 0.0), (0.0, 600.0), (600.0, 600.0)], 5);
    Ok(())
}
fn draw_triangle(context: &web_sys::CanvasRenderingContext2d, vertices: [(f64, f64); 3]) {
    let [(cx, cy), (lx, ly), (rx, ry)] = vertices;

    context.move_to(cx, cy);
    context.begin_path();
    context.line_to(lx, ly);
    context.line_to(rx, ry);
    context.line_to(cx, cy);
    context.close_path();
    context.stroke();
    return;
}

fn sierpinsky(context: &web_sys::CanvasRenderingContext2d, vertices: [(f64, f64); 3], depth: u32) {
    draw_triangle(context, vertices);
    let depth = depth - 1;
    if depth > 0 {
        let mid = midpoints(vertices);
        draw_triangle(context, mid);
        let triangles = [
            [mid[1], vertices[1], mid[0]],
            [mid[2], mid[0], vertices[2]],
            [vertices[0], mid[1], mid[2]],
        ];
        for triangle in triangles.iter() {
            sierpinsky(context, *triangle, depth);
        }
    }
}
fn midpoints(vertices: [(f64, f64); 3]) -> [(f64, f64); 3] {
    let [(cx, cy), (lx, ly), (rx, ry)] = vertices;
    [
        ((lx + rx) / 2.0, (ly + ry) / 2.0),
        ((cx + lx) / 2.0, (cy + ly) / 2.0),
        ((cx + rx) / 2.0, (cy + ry) / 2.0),
    ]
}
