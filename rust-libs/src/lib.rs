mod utils;

use std::{convert::TryInto, fmt};
use wasm_bindgen::prelude::*;

use crate::hexagon::{
    draw_hex, hex_pointy_canvas_tiling, hex_polygon_corners, HexLayout, HexOrientation,
};

pub mod hexagon;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, rust-libs!");
}

#[wasm_bindgen]
pub struct Canvas {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
}

#[wasm_bindgen]
impl Canvas {
    pub fn new(width: u32, height: u32) -> Canvas {
        let pixels = (0..width * height * 4).map(|_| 255).collect();

        Canvas {
            width,
            height,
            pixels,
        }
    }

    pub fn do_stuff(&mut self) {
        /*self.draw_pixel(&Pixel::new(0, 0), [0, 0, 0, 255]);
        self.draw_pixel(&Pixel::new(1, 0), [0, 0, 0, 255]);
        self.draw_pixel(&Pixel::new(2, 6), [0, 0, 0, 255]);
        self.draw_pixel(&Pixel::new(3, 4), [0, 0, 0, 255]);
        log("First line");
        self.draw_line(&Pixel::new(8, 2), &Pixel::new(1, 9), [255, 255, 0, 255]);
        log("Second line");
        self.draw_line(&Pixel::new(8, 2), &Pixel::new(7, 9), [255, 0, 255, 255]);*/
        let orientation = HexOrientation::new_layout_pointy();
        let size = Pixel::new(10, 10);
        let origin = Pixel::new(0, 0);
        let layout = HexLayout::new(orientation, size, origin);
        let tiling = hex_pointy_canvas_tiling(self.width, self.height, &layout);
        for tile in &tiling {
            draw_hex(self, &layout, &tile, [145, 57, 233, 255]);
        }
        for tile in &tiling {
            let corners = hex_polygon_corners(&layout, &tile);
            for i in 0..6 {
                log(format!("Hex {:?}, Vertex {:?}", tile, corners[i]).as_str());
                let p: Pixel = shift_point_to_vertex(&corners[i]);
                self.draw_pixel(&p, [255, 0, 0, 255]);
            }
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn pixels(&self) -> *const u8 {
        self.pixels.as_ptr()
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

#[derive(fmt::Debug)]
pub struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }
}

pub type Pixel = Point<i32>;

fn shift_point_to_vertex(point: &Point<f32>) -> Pixel {
    let x = (point.x + 0.5f32).floor() as i32;
    let y = (point.y + 0.5f32).floor() as i32;
    Pixel::new(x, y)
}

impl Canvas {
    fn get_index(&self, pixel: &Pixel) -> usize {
        4 * (pixel.x * self.width as i32 + pixel.y) as usize
    }

    fn draw_pixel(&mut self, pixel: &Pixel, rgba: [u8; 4]) {
        //log(format!("Pixel {}, {}", pixel.x, pixel.y).as_str());
        if pixel.x < 0
            || pixel.x >= self.height.try_into().unwrap()
            || pixel.y < 0
            || pixel.y >= self.width.try_into().unwrap()
        {
            //log("Out of bounds");
            return;
        }
        let index = self.get_index(pixel);
        self.pixels[index..index + 4].copy_from_slice(&rgba);
    }

    fn draw_line(&mut self, start: &Pixel, end: &Pixel, rgba: [u8; 4]) {
        let drow = if start.x < end.x {
            end.x - start.x
        } else {
            start.x - end.x
        };

        let dcolumn = if start.y < end.y {
            end.y - start.y
        } else {
            start.y - end.y
        };

        if dcolumn < drow {
            if start.x > end.x {
                self.draw_line_low(end, start, rgba)
            } else {
                self.draw_line_low(start, end, rgba)
            }
        } else {
            if start.y > end.y {
                self.draw_line_high(end, start, rgba)
            } else {
                self.draw_line_high(start, end, rgba)
            }
        }
    }

    fn draw_line_low(&mut self, a: &Pixel, b: &Pixel, rgba: [u8; 4]) {
        let dx: i64 = i64::from(b.x) - i64::from(a.x);
        let mut dy: i64 = i64::from(b.y) - i64::from(a.y);

        let mut yi: i64 = 1;

        if dy < 0 {
            yi = -1;
            dy = -dy;
        }
        let mut D = 2 * dy - dx;
        let mut y = i64::from(a.y);

        for x in a.x..=b.x {
            let row: Result<i32, _> = x.try_into();
            let column: Result<i32, _> = y.try_into();
            if row.is_ok() && column.is_ok() {
                self.draw_pixel(&Pixel::new(row.unwrap(), column.unwrap()), rgba);
            }
            if D > 0 {
                y = y + yi;
                D = D + (2 * (dy - dx));
            } else {
                D = D + 2 * dy;
            }
        }
    }

    fn draw_line_high(&mut self, a: &Pixel, b: &Pixel, rgba: [u8; 4]) {
        let mut dx: i64 = i64::from(b.x) - i64::from(a.x);
        let dy: i64 = i64::from(b.y) - i64::from(a.y);

        let mut xi: i64 = 1;

        if dx < 0 {
            xi = -1;
            dx = -dx;
        }
        let mut D = 2 * dx - dy;
        let mut x = i64::from(a.x);

        for y in a.y..=b.y {
            let row: Result<i32, _> = x.try_into();
            let column: Result<i32, _> = y.try_into();
            if row.is_ok() && column.is_ok() {
                self.draw_pixel(&Pixel::new(row.unwrap(), column.unwrap()), rgba);
            }
            if D > 0 {
                x = x + xi;
                D = D + (2 * (dx - dy));
            } else {
                D = D + 2 * dx;
            }
        }
    }
}

fn rgba_to_rgb(rgba: [u8; 4]) -> [u8; 3] {
    let r = rgba[0] as f32;
    let g = rgba[1] as f32;
    let b = rgba[2] as f32;
    let a = rgba[3] as f32 / 255.0;

    let r2 = (r * a + 255.0 * (1.0 - a)) as u8;
    let g2 = (g * a + 255.0 * (1.0 - a)) as u8;
    let b2 = (b * a + 255.0 * (1.0 - a)) as u8;

    [r2, g2, b2]
}

fn find_closest_heart(color_rgba: [u8; 4]) -> &'static str {
    let hearts = [
        ("❤️", [255, 0, 0]),
        ("💛", [255, 255, 0]),
        ("💚", [0, 255, 0]),
        ("💙", [0, 0, 255]),
        ("💜", [128, 0, 128]),
        ("🖤", [0, 0, 0]),
        ("🤍", [255, 255, 255]),
        ("💔", [255, 105, 180]),
        ("❣️", [255, 192, 203]),
        ("💕", [255, 20, 147]),
        ("💞", [255, 182, 193]),
        ("💓", [255, 160, 122]),
        ("💗", [255, 192, 203]),
        ("💖", [255, 182, 193]),
        ("💘", [255, 105, 180]),
        ("🧡", [255, 165, 0]),
        ("🤎", [139, 69, 19]),
        ("💖", [255, 192, 203]),
    ];

    let rgb = rgba_to_rgb(color_rgba);

    let mut closest_heart = "";
    let mut closest_distance = f32::INFINITY;

    for (heart, color) in &hearts {
        let distance = ((color[0] as f32 - rgb[0] as f32).powi(2)
            + (color[1] as f32 - rgb[1] as f32).powi(2)
            + (color[2] as f32 - rgb[2] as f32).powi(2))
        .sqrt();
        if distance < closest_distance {
            closest_distance = distance;
            closest_heart = heart;
        }
    }

    closest_heart
}

impl fmt::Display for Canvas {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.pixels.as_slice().chunks(4 * self.width as usize) {
            for pixel in line.chunks(4) {
                write!(
                    f,
                    "{}",
                    find_closest_heart([pixel[0], pixel[1], pixel[2], pixel[3]])
                )?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
