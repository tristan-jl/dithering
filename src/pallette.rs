use image::Rgb;

use crate::f32_to_u8;
use crate::space::{ColourSpace, EuclideanDistance};

pub struct Palette(Vec<Rgb<u8>>);

impl From<&[[u8; 3]]> for Palette {
    fn from(value: &[[u8; 3]]) -> Self {
        Self(value.iter().map(|&i| image::Rgb(i)).collect())
    }
}
impl From<Vec<[u8; 3]>> for Palette {
    fn from(value: Vec<[u8; 3]>) -> Self {
        value.as_slice().into()
    }
}

impl Palette {
    #[must_use]
    pub fn from_blend(
        desat_pallette: &[[u8; 3]],
        sat_pallette: &[[u8; 3]],
        saturation: f32,
    ) -> Self {
        assert!((0.0..=1.0).contains(&saturation));

        let mut res = desat_pallette.to_vec();
        for (r, (d, s)) in res
            .iter_mut()
            .zip(desat_pallette.iter().zip(sat_pallette.iter()))
        {
            *r = [
                f32_to_u8(f32::from(d[0]) * (1.0 - saturation) + f32::from(s[0]) * saturation),
                f32_to_u8(f32::from(d[1]) * (1.0 - saturation) + f32::from(s[1]) * saturation),
                f32_to_u8(f32::from(d[2]) * (1.0 - saturation) + f32::from(s[2]) * saturation),
            ];
        }
        res.into()
    }

    pub fn from_hex_text(input: &str) -> Self {
        input
            .lines()
            .map(|i| {
                let x = u32::from_str_radix(i.trim(), 16).unwrap().to_be_bytes();
                [x[1], x[2], x[3]]
            })
            .collect::<Vec<[u8; 3]>>()
            .into()
    }

    #[must_use]
    pub fn closest_colour(&self, space: ColourSpace, pixel: &Rgb<u8>) -> Rgb<u8> {
        let mut closest_colour_idx = 0;
        let mut closest_dist: f32 = f32::MAX;

        for (i, palette_colour) in self.0.iter().enumerate() {
            let dist: f32 = space.distance_sq(*palette_colour, *pixel);
            if dist < closest_dist {
                closest_colour_idx = i;
                closest_dist = dist;
            }
        }

        self.0[closest_colour_idx]
    }

    #[must_use]
    pub fn to_idx(&self, pixel: &Rgb<u8>) -> u8 {
        for (i, c) in self.0.iter().enumerate() {
            if *c == *pixel {
                return i as u8;
            }
        }
        0
    }
}
