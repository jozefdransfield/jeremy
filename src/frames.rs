use std::collections::HashMap;
use std::path::PathBuf;
use image::{ImageBuffer, Rgba, RgbaImage};
use crate::image_functions::{overlay_image, overlay_image_rounded};

pub struct Frame {
    pub device: &'static str,
    pub variant: &'static str,
    pub dimensions: (u32, u32),
    pub corner_radius: u32,
    pub path: &'static str,
}

pub fn frame(screen_shot_path: PathBuf, variant: Option<String>, output_path: PathBuf) {
    let screen_shot = image::open(screen_shot_path)
        .expect("Failed to open background image")
        .to_rgba8();

    // this will filter based on variant
    let frames = all_frames();
    let device = frames
        .iter()
        .filter(|frame| frame.dimensions == screen_shot.dimensions())
        .filter(|frame| {
            if variant.is_some() {
                frame.variant == variant.as_ref().unwrap()
            } else {
                true
            }
        })
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
        .save(&output_path)
        .expect("Failed to save output image");

    println!("Image saved to {}", output_path.to_str().unwrap());
}

pub fn all_frames() -> Vec<Frame> {
    vec![
        Frame {
            device: "iPhone 16 Pro Max",
            variant: "Black Titanium",
            dimensions: (2868, 1320),
            corner_radius: 100,
            path: "frames/iPhone 16 Pro Max/iPhone 16 Pro Max - Black Titanium - Landscape.png",
        },
        Frame {
            device: "iPhone 16 Pro Max",
            variant: "Black Titanium",
            dimensions: (1320, 2868),
            corner_radius: 100,
            path: "frames/iPhone 16 Pro Max/iPhone 16 Pro Max - Black Titanium - Portrait.png",
        },
        Frame {
            device: "iPhone 16 Pro Max",
            variant: "Desert Titanium",
            dimensions: (2868, 1320),
            corner_radius: 100,
            path: "frames/iPhone 16 Pro Max/iPhone 16 Pro Max - Desert Titanium - Landscape.png",
        },
        Frame {
            device: "iPhone 16 Pro Max",
            variant: "Desert Titanium",
            dimensions: (1320, 2868),
            corner_radius: 100,
            path: "frames/iPhone 16 Pro Max/iPhone 16 Pro Max - Desert Titanium - Portrait.png",
        },
        Frame {
            device: "iPhone 16 Pro Max",
            variant: "Natural Titanium",
            dimensions: (2868, 1320),
            corner_radius: 100,
            path: "frames/iPhone 16 Pro Max/iPhone 16 Pro Max - Natural Titanium - Landscape.png",
        },
        Frame {
            device: "iPhone 16 Pro Max",
            variant: "Natural Titanium",
            dimensions: (1320, 2868),
            corner_radius: 100,
            path: "frames/iPhone 16 Pro Max/iPhone 16 Pro Max - Natural Titanium - Portrait.png",
        },
        Frame {
            device: "iPhone 16 Pro Max",
            variant: "White Titanium",
            dimensions: (2868, 1320),
            corner_radius: 100,
            path: "frames/iPhone 16 Pro Max/iPhone 16 Pro Max - White Titanium - Landscape.png",
        },
        Frame {
            device: "iPhone 16 Pro Max",
            variant: "White Titanium",
            dimensions: (1320, 2868),
            corner_radius: 100,
            path: "frames/iPhone 16 Pro Max/iPhone 16 Pro Max - White Titanium - Portrait.png",
        },






        Frame {
            device: "iPhone 16 Pro",
            variant: "Black Titanium",
            dimensions: (2622, 1206),
            corner_radius: 100,
            path: "frames/iPhone 16 Pro/iPhone 16 Pro - Black Titanium - Landscape.png",
        },
        Frame {
            device: "iPhone 16 Pro",
            variant: "Black Titanium",
            dimensions: (1206, 2622),
            corner_radius: 100,
            path: "frames/iPhone 16 Pro/iPhone 16 Pro - Black Titanium - Portrait.png",
        },
        Frame {
            device: "iPhone 16 Pro",
            variant: "Desert Titanium",
            dimensions: (2622, 1206),
            corner_radius: 100,
            path: "frames/iPhone 16 Pro/iPhone 16 Pro - Desert Titanium - Landscape.png",
        },
        Frame {
            device: "iPhone 16 Pro",
            variant: "Desert Titanium",
            dimensions: (1206, 2622),
            corner_radius: 100,
            path: "frames/iPhone 16 Pro/iPhone 16 Pro - Desert Titanium - Portrait.png",
        },
        Frame {
            device: "iPhone 16 Pro",
            variant: "Natural Titanium",
            dimensions: (2622, 1206),
            corner_radius: 100,
            path: "frames/iPhone 16 Pro/iPhone 16 Pro - Natural Titanium - Landscape.png",
        },
        Frame {
            device: "iPhone 16 Pro",
            variant: "Natural Titanium",
            dimensions: (1206, 2622),
            corner_radius: 100,
            path: "frames/iPhone 16 Pro/iPhone 16 Pro - Natural Titanium - Portrait.png",
        },
        Frame {
            device: "iPhone 16 Pro",
            variant: "White Titanium",
            dimensions: (2622, 1206),
            corner_radius: 100,
            path: "frames/iPhone 16 Pro/iPhone 16 Pro - White Titanium - Landscape.png",
        },
        Frame {
            device: "iPhone 16 Pro",
            variant: "White Titanium",
            dimensions: (1206, 2622),
            corner_radius: 100,
            path: "frames/iPhone 16 Pro Max/iPhone 16 Pro - White Titanium - Portrait.png",
        },







        Frame {
            device: "iPhone 16 Plus",
            variant: "Black",
            dimensions: (2796, 1290),
            corner_radius: 100,
            path: "frames/iPhone 16 Plus/iPhone 16 Plus - Black - Landscape.png",
        },
        Frame {
            device: "iPhone 16 Plus",
            variant: "Black",
            dimensions: (1290, 2796),
            corner_radius: 100,
            path: "frames/iPhone 16 Plus/iPhone 16 Plus - Black - Portrait.png",
        },
        Frame {
            device: "iPhone 16 Plus",
            variant: "Pink",
            dimensions: (2796, 1290),
            corner_radius: 100,
            path: "frames/iPhone 16 Plus/iPhone 16 Plus - Pink - Landscape.png",
        },
        Frame {
            device: "iPhone 16 Plus",
            variant: "Pink",
            dimensions: (1290, 2796),
            corner_radius: 100,
            path: "frames/iPhone 16 Plus/iPhone 16 Plus - Pink - Portrait.png",
        },
        Frame {
            device: "iPhone 16 Plus",
            variant: "Teal",
            dimensions: (2796, 1290),
            corner_radius: 100,
            path: "frames/iPhone 16 Plus/iPhone 16 Plus - Teal - Landscape.png",
        },
        Frame {
            device: "iPhone 16 Plus",
            variant: "Teal",
            dimensions: (1290, 2796),
            corner_radius: 100,
            path: "frames/iPhone 16 Plus/iPhone 16 Plus - Teal - Portrait.png",
        },
        Frame {
            device: "iPhone 16 Plus",
            variant: "Ultramarine",
            dimensions: (2796, 1290),
            corner_radius: 100,
            path: "frames/iPhone 16 Plus/iPhone 16 Plus - Ultramarine - Landscape.png",
        },
        Frame {
            device: "iPhone 16 Plus",
            variant: "Ultramarine",
            dimensions: (1290, 2796),
            corner_radius: 100,
            path: "frames/iPhone 16 Plus/iPhone 16 Plus - Ultramarine - Portrait.png",
        },
        Frame {
            device: "iPhone 16 Plus",
            variant: "White",
            dimensions: (2796, 1290),
            corner_radius: 100,
            path: "frames/iPhone 16 Plus/iPhone 16 Plus - White - Landscape.png",
        },
        Frame {
            device: "iPhone 16 Plus",
            variant: "White",
            dimensions: (1290, 2796),
            corner_radius: 100,
            path: "frames/iPhone 16 Plus/iPhone 16 Plus - White - Portrait.png",
        },




        

        Frame {
            device: "iPhone 16",
            variant: "Black",
            dimensions: (2556, 1179),
            corner_radius: 100,
            path: "frames/iPhone 16/iPhone 16 - Black - Landscape.png",
        },
        Frame {
            device: "iPhone 16",
            variant: "Black",
            dimensions: (1179, 2556),
            corner_radius: 100,
            path: "frames/iPhone 16/iPhone 16 - Black - Portrait.png",
        },
        Frame {
            device: "iPhone 16",
            variant: "Pink",
            dimensions: (2556, 1179),
            corner_radius: 100,
            path: "frames/iPhone 16/iPhone 16 - Pink - Landscape.png",
        },
        Frame {
            device: "iPhone 16 Plus",
            variant: "Pink",
            dimensions: (1179, 2556),
            corner_radius: 100,
            path: "frames/iPhone 16/iPhone 16 - Pink - Portrait.png",
        },
        Frame {
            device: "iPhone 16",
            variant: "Teal",
            dimensions: (2556, 1179),
            corner_radius: 100,
            path: "frames/iPhone 16/iPhone 16 - Teal - Landscape.png",
        },
        Frame {
            device: "iPhone 16",
            variant: "Teal",
            dimensions: (1179, 2556),
            corner_radius: 100,
            path: "frames/iPhone 16/iPhone 16 - Teal - Portrait.png",
        },
        Frame {
            device: "iPhone 16",
            variant: "Ultramarine",
            dimensions: (2556, 1179),
            corner_radius: 100,
            path: "frames/iPhone 16/iPhone 16 - Ultramarine - Landscape.png",
        },
        Frame {
            device: "iPhone 16",
            variant: "Ultramarine",
            dimensions: (1179, 2556),
            corner_radius: 100,
            path: "frames/iPhone 16/iPhone 16 - Ultramarine - Portrait.png",
        },
        Frame {
            device: "iPhone 16",
            variant: "White",
            dimensions: (2556, 1179),
            corner_radius: 100,
            path: "frames/iPhone 16/iPhone 16 - White - Landscape.png",
        },
        Frame {
            device: "iPhone 16",
            variant: "White",
            dimensions: (1179, 2556),
            corner_radius: 100,
            path: "frames/iPhone 16/iPhone 16 - White - Portrait.png",
        },





    ]

    //
    // const I_PHONE_16_LANDSCAPE: Device = Device {
    //     dimensions: ,
    //     style: "frames/iPhone 16/iPhone 16 - Black - Landscape.png",
    //     corner_radius: 100,
    // };
    //
    // const I_PHONE_16_PORTRAIT: Device = Device {
    //     dimensions: ,
    //     style: "frames/iPhone 16/iPhone 16 - Black - Portrait.png",
    //     corner_radius: 100,
    // };
}
