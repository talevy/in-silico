#![feature(phase)]
extern crate cairo;
extern crate num;
#[phase(plugin)]
extern crate lazy_static;

use std::io::File;
use std::num::Float;

use num::complex::{Complex, Complex32};
use cairo::Cairo;
use cairo::surface::Surface;
use cairo::surface::format::Format;

lazy_static! {
    static ref GOLDEN_RATIO: f32 = (1.0 + 5f32.sqrt()) / 2.0;
}

type Triangle = (uint, Complex32, Complex32, Complex32);

fn subdivide(triangles: Vec<Triangle>) {
    for &(color, a, b, c) in triangles.iter() {
        if color == 1 {
            let p:Complex32 = a + (b - a).unscale(*GOLDEN_RATIO);
        }
    }
}

fn main() {
    let (img_width, img_height): (f64, f64) = (1000.0, 1000.0);
    let mut surface: Surface = Surface::create_similar_image(Format::ARGB32, img_width as i32, img_height as i32);
    let mut context: Cairo = Cairo::create(&mut surface);
    context.translate(img_width / 2.0, img_height / 2.0);
    let wheelRadius = 1.2 * ((img_width / 2.0).powi(2) + (img_height / 2.0).powi(2)).sqrt();
    context.scale(wheelRadius, wheelRadius);
    context.set_source_rgb(1.0, 0.35, 0.35);
    context.fill();
    surface.write_to_png("penrose.png");
}
