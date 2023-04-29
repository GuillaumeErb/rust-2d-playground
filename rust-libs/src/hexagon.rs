use std::{f32::consts::PI, fmt};

use crate::{log, shift_point_to_vertex, Canvas, Pixel, Point};

#[derive(fmt::Debug)]
pub struct Hex {
    q: i32,
    r: i32,
    s: i32,
}

impl Hex {
    pub fn new(q: i32, r: i32, s: i32) -> Hex {
        Hex { q, r, s }
    }
    pub fn new_with_axial(q: i32, r: i32) -> Hex {
        Hex { q, r, s: -q - r }
    }
}

pub struct HexLayout {
    orientation: HexOrientation,
    size: Pixel,
    origin: Pixel,
}

impl HexLayout {
    pub fn new(orientation: HexOrientation, size: Pixel, origin: Pixel) -> HexLayout {
        HexLayout {
            orientation,
            size,
            origin,
        }
    }
}

pub fn hex_to_pixel(layout: &HexLayout, h: &Hex) -> Point<f32> {
    let x: f32 = (layout.orientation.f[0] * (h.q as f32) + layout.orientation.f[1] * h.r as f32)
        * layout.size.x as f32
        + layout.origin.x as f32;
    let y: f32 = (layout.orientation.f[2] * (h.q as f32) + layout.orientation.f[3] * h.r as f32)
        * layout.size.y as f32
        + layout.origin.y as f32;

    Point::<f32>::new(x, y)
}

fn pixel_to_hex(layout: &HexLayout, pixel: Pixel) -> Hex {
    let ptx = (pixel.x as f32 - layout.origin.x as f32) as f32 / layout.size.x as f32;
    let pty = (pixel.x as f32 - layout.origin.x as f32) as f32 / layout.size.x as f32;

    let q = layout.orientation.b[0] * ptx + layout.orientation.b[1] * pty;
    let r = layout.orientation.b[2] * ptx + layout.orientation.b[3] * pty;

    let q_round = q.round() as i32;
    let r_round = r.round() as i32;

    Hex::new_with_axial(q_round, r_round)
}

fn hex_corner_offset(layout: &HexLayout, corner: u8) -> Point<f32> {
    let angle = PI * (layout.orientation.start_angle + corner as f32) / 3_f32;
    let x = layout.size.x as f32 * angle.cos();
    let y = layout.size.y as f32 * angle.sin();

    Point::<f32>::new(x, y)
}

pub fn hex_polygon_corners(layout: &HexLayout, h: &Hex) -> Vec<Point<f32>> {
    let mut corners: Vec<Point<f32>> = vec![];
    let center = hex_to_pixel(&layout, &h);
    for i in 0..6 {
        let offset = hex_corner_offset(&layout, i);
        let x = center.x + offset.x;
        let y = center.y + offset.y;
        corners.push(Point::<f32>::new(x, y))
    }
    corners
}

pub fn draw_hex(canvas: &mut Canvas, layout: &HexLayout, h: &Hex, rgba: [u8; 4]) {
    let corners = hex_polygon_corners(layout, h);
    for i in 0..6 {
        //log(format!("Hex {} -> {}", corners[i], corners[(i + 1) % 6]).as_str());
        let start: Pixel = shift_point_to_vertex(&corners[i]);
        let end: Pixel = shift_point_to_vertex(&corners[(i + 1) % 6]);
        canvas.draw_line(&start, &end, rgba);
    }
}

pub struct HexOrientation {
    f: [f32; 4],
    b: [f32; 4],
    start_angle: f32,
}

impl HexOrientation {
    pub fn new_layout_pointy() -> HexOrientation {
        HexOrientation {
            f: [3_f32.sqrt(), 3_f32.sqrt() / 2_f32, 0_f32, 3_f32 / 2_f32],
            b: [3_f32.sqrt() / 3_f32, -1_f32 / 3_f32, 0_f32, 2_f32 / 3_f32],
            start_angle: 0.5_f32,
        }
    }

    pub fn new_layout_flat() -> HexOrientation {
        HexOrientation {
            f: [3_f32 / 2_f32, 0_f32, 3_f32.sqrt() / 2_f32, 3_f32.sqrt()],
            b: [2_f32 / 3_f32, 0_f32, -1_f32 / 3_f32, 3_f32.sqrt() / 3_f32],
            start_angle: 0_f32,
        }
    }
}

pub fn hex_pointy_canvas_tiling(width: u32, height: u32, layout: &HexLayout) -> Vec<Hex> {
    let mut result = vec![];
    let height_hex: i32 = (height as f32 / layout.size.x as f32).ceil() as i32;
    let width_hex: i32 = (width as f32 / layout.size.y as f32).ceil() as i32;
    for r in 0..=height_hex {
        let r_offset = (r as f32 / 2f32).floor() as i32;
        for q in -r_offset..=width_hex - r_offset {
            let hex = Hex::new_with_axial(q, r);
            log(format!("Hex {:?}", hex).as_str());
            result.push(hex);
        }
    }
    result
}
