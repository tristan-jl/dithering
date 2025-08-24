pub mod pallette;
pub mod space;

pub use pallette::Palette;
pub use space::ColourSpace;

use image::RgbImage;
use itertools::Itertools;

fn f32_to_u8(input: f32) -> u8 {
    input.round().clamp(0.0, 255.0) as u8
}

pub fn quantise_image(buf: &mut RgbImage, palette: &Palette, space: ColourSpace) {
    buf.pixels_mut().for_each(|pixel| {
        *pixel = palette.closest_colour(space, pixel);
    });
}

#[must_use]
pub fn quantise_and_dither_image(
    buf: &RgbImage,
    palette: &Palette,
    space: ColourSpace,
) -> RgbImage {
    let mut res = buf.clone();
    let (max_width, max_height) = buf.dimensions();

    buf.enumerate_pixels().for_each(|(x, y, old_pixel)| {
        fn helper(input: f32, quant_err: f32, factor: f32) -> u8 {
            f32_to_u8(input + quant_err * factor)
        }

        res[(x, y)] = palette.closest_colour(space, old_pixel);

        if x > 0 && x < max_width - 1 && y < max_height - 1 {
            let mut right_pixel = res[(x + 1, y)];
            let mut bottom_left_pixel = res[(x - 1, y + 1)];
            let mut bottom_pixel = res[(x, y + 1)];
            let mut bottom_right_pixel = res[(x + 1, y + 1)];

            for ((i, &old), &new) in old_pixel.0.iter().enumerate().zip(res[(x, y)].0.iter()) {
                let quant_err = f32::from(old) - f32::from(new);
                right_pixel.0[i] = helper(f32::from(right_pixel.0[i]), quant_err, 7.0 / 16.0);
                bottom_left_pixel.0[i] =
                    helper(f32::from(bottom_left_pixel.0[i]), quant_err, 3.0 / 16.0);
                bottom_pixel.0[i] = helper(f32::from(bottom_pixel.0[i]), quant_err, 5.0 / 16.0);
                bottom_right_pixel.0[i] =
                    helper(f32::from(bottom_right_pixel.0[i]), quant_err, 1.0 / 16.0);
            }
            res[(x + 1, y)] = right_pixel;
            res[(x - 1, y + 1)] = bottom_left_pixel;
            res[(x, y + 1)] = bottom_pixel;
            res[(x + 1, y + 1)] = bottom_right_pixel;
        }
    });

    res
}

#[must_use]
pub fn image_to_bytes(buf: &RgbImage, pallette: &Palette) -> Vec<u8> {
    let (w, h) = buf.dimensions();
    let mut res = Vec::with_capacity((w * h) as usize);
    for mut i in &buf.pixels().chunks(2) {
        let l = i.next().unwrap();
        let r = i.next().unwrap();
        res.push(pallette.to_idx(l) << 4 | pallette.to_idx(r));
    }

    res
}
