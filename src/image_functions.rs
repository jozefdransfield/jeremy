use image::RgbaImage;

/// Draws the overlay image onto the base image at the given position (with alpha blending).
pub fn overlay_image(base: &mut RgbaImage, overlay: &RgbaImage, x_offset: u32, y_offset: u32) {
    for (x, y, pixel) in overlay.enumerate_pixels() {
        let (ox, oy) = (x + x_offset, y + y_offset);
        if ox >= base.width() || oy >= base.height() {
            continue;
        }

        let base_pixel = base.get_pixel_mut(ox, oy);
        let blended = blend_pixels(*base_pixel, *pixel);
        *base_pixel = blended;
    }
}

pub fn overlay_image_rounded(base: &mut RgbaImage, overlay: &RgbaImage, x_offset: u32, y_offset: u32, corner_radius: u32) {
    for (x, y, pixel) in overlay.enumerate_pixels() {
        let (ox, oy) = (x + x_offset, y + y_offset);
        if ox >= base.width() || oy >= base.height() {
            continue;
        }

        // Check if we're in a corner region
        let is_in_rounded_corner = is_outside_rounded_rect(
            x, y,
            overlay.width(),
            overlay.height(),
            corner_radius,
        );

        // If we're outside the rounded rectangle, skip this pixel
        if is_in_rounded_corner {
            continue;
        }

        let base_pixel = base.get_pixel_mut(ox, oy);
        let blended = blend_pixels(*base_pixel, *pixel);
        *base_pixel = blended;
    }
}

/// Alpha blends two RGBA pixels.
fn blend_pixels(bottom: image::Rgba<u8>, top: image::Rgba<u8>) -> image::Rgba<u8> {
    let alpha_top = top[3] as f32 / 255.0;
    let alpha_bottom = bottom[3] as f32 / 255.0;
    let alpha_out = alpha_top + alpha_bottom * (1.0 - alpha_top);

    let mut out = [0u8; 4];
    for i in 0..3 {
        out[i] = (((top[i] as f32 * alpha_top) + (bottom[i] as f32 * alpha_bottom * (1.0 - alpha_top)))
            / alpha_out)
            .round()
            .min(255.0) as u8;
    }
    out[3] = (alpha_out * 255.0).round().min(255.0) as u8;

    image::Rgba(out)
}

// Helper function to determine if a point is outside the rounded rectangle
fn is_outside_rounded_rect(x: u32, y: u32, width: u32, height: u32, radius: u32) -> bool {
    // Check each corner
    if x < radius && y < radius {
        // Top-left corner
        let dx = radius - x;
        let dy = radius - y;
        return (dx * dx + dy * dy) > radius * radius;
    } else if x >= width - radius && y < radius {
        // Top-right corner
        let dx = x - (width - radius);
        let dy = radius - y;
        return (dx * dx + dy * dy) > radius * radius;
    } else if x < radius && y >= height - radius {
        // Bottom-left corner
        let dx = radius - x;
        let dy = y - (height - radius);
        return (dx * dx + dy * dy) > radius * radius;
    } else if x >= width - radius && y >= height - radius {
        // Bottom-right corner
        let dx = x - (width - radius);
        let dy = y - (height - radius);
        return (dx * dx + dy * dy) > radius * radius;
    }

    false
}