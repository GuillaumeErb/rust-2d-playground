mod utils;

use std::fmt;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
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
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    pub fn new(width: u32, height: u32) -> Canvas {
        let pixels = (0..width * height * 4)
            .map(|i| if i % 7 == 0 || i % 3 == 0 { 255 } else { 0 })
            .collect();

        Canvas {
            width,
            height,
            pixels,
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
