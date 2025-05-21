mod frames;
mod tests;
mod image_functions;

use crate::frames::all_frames;
use crate::image_functions::overlay_image;
use crate::image_functions::overlay_image_rounded;
use image::{ImageBuffer, Rgba, RgbaImage};

pub fn frame(screen_shot_path: &str, variant: Option<&str>, output_path: &str) {
    let screen_shot = image::open(screen_shot_path)
        .expect("Failed to open background image")
        .to_rgba8();

    // this will filter based on variant
    let frames = all_frames();
    let device = frames
        .iter()
        .filter(|frame| frame.dimensions == screen_shot.dimensions())
        .filter(|frame| if variant.is_some() { frame.variant == variant.unwrap() } else { true })
        .next()
        .expect("Failed to find device");

    let device_frame = image::open(device.path)
        .expect("Failed to open overlay image")
        .to_rgba8();

    // draw a canvas the size of the frame
    let mut canvas: RgbaImage = ImageBuffer::from_pixel(
        device_frame.width(),
        device_frame.height(),
        Rgba([0, 0, 0, 0]),
    );

    let offset_x = (device_frame.width() - screen_shot.width()) / 2;
    let offset_y = (device_frame.height() - screen_shot.height()) / 2;

    // Composite: paste the overlay onto the background
    overlay_image_rounded(
        &mut canvas,
        &screen_shot,
        offset_x,
        offset_y,
        device.corner_radius,
    );
    overlay_image(&mut canvas, &device_frame, 0, 0);

    // Save the result
    canvas
        .save(output_path)
        .expect("Failed to save output image");

    println!("Image saved to {}", output_path);
}

fn main() {

    // frame --variant = "Black Titanium" --output target/iPhone 16 Pro Max - Portrait.png


    
}
