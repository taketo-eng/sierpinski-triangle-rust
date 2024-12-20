use rand::thread_rng;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use rand::prelude::*;


#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();


    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    // get client width and height
    let client_width = canvas.client_width();
    let client_height = canvas.client_height();
    web_sys::console::log_1(&format!("client_width: {}", client_width).into());
    web_sys::console::log_1(&format!("client_height: {}", client_height).into());
    // canvas width and height, client width and height
    canvas.set_width(canvas.client_width().try_into().unwrap());
    canvas.set_height(canvas.client_height().try_into().unwrap());

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();


        sierpinski(
            &context, 
            [(300.0, 0.0), (0.0, 600.0), (600.0, 600.0)], 
            (0, 255, 0),
            5);

    Ok(())
}

fn sierpinski(context: &web_sys::CanvasRenderingContext2d, points: [(f64, f64); 3], color: (u8, u8, u8), depth: u8) {
    draw_triangle(&context, points, color);
    let depth = depth - 1;
    let [top, left, right] = points;
    if depth > 0 {
        let mut rng = thread_rng();

        let next_color = (
            rng.gen_range(0..255),
            rng.gen_range(0..255),
            rng.gen_range(0..255),
        );


        let left_middle = midpoint(top, left);
        let right_middle = midpoint(top, right);
        let bottom_middle = midpoint(left, right);

        sierpinski(&context, [top, left_middle, right_middle], next_color, depth);
        sierpinski(&context, [left_middle, left, bottom_middle], next_color, depth);
        sierpinski(&context, [right_middle, bottom_middle, right], next_color, depth);
    }
}

fn midpoint(point_1: (f64, f64), point_2: (f64, f64)) -> (f64, f64) {
    ((point_1.0 + point_2.0) / 2.0, (point_1.1 + point_2.1) / 2.0)
}

fn draw_triangle(
    context: &web_sys::CanvasRenderingContext2d, 
    points:[(f64, f64); 3],
    color: (u8, u8, u8)
) {
    let color_str = format!("rgb({}, {}, {})", color.0, color.1, color.2);
    context.set_fill_style_str(&color_str);

    let [top, left, right] = points;
    context.move_to(top.0, top.1);
    context.begin_path();
    context.line_to(left.0, left.1);
    context.line_to(right.0, right.1);
    context.line_to(top.0, top.1);
    context.close_path();
    context.stroke();
    context.fill();
}