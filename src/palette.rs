use crate::f32_to_u8;
use crate::space::{ColourSpace, EuclideanDistance};
use anyhow::anyhow;
use anyhow::{Context, Result};
use image::Rgb;
use std::path::Path;

/// Palette
/// Represents a color palette as a collection of RGB colors.
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
    /// Creates a new Palette by blending 2 palettes together based on the given saturation.
    ///
    /// # Errors
    ///
    /// Returns an error if saturation is not 0 <= s <= 1.
    pub fn from_blend(
        desat_palette: &[[u8; 3]],
        sat_palette: &[[u8; 3]],
        saturation: f32,
    ) -> Result<Self> {
        if !(0.0..=1.0).contains(&saturation) {
            return Err(anyhow!(
                "Saturation should be between 0 and 1, got: {}",
                saturation
            ));
        }

        let mut res = desat_palette.to_vec();
        for (r, (d, s)) in res
            .iter_mut()
            .zip(desat_palette.iter().zip(sat_palette.iter()))
        {
            *r = [
                f32_to_u8(f32::from(d[0]) * (1.0 - saturation) + f32::from(s[0]) * saturation),
                f32_to_u8(f32::from(d[1]) * (1.0 - saturation) + f32::from(s[1]) * saturation),
                f32_to_u8(f32::from(d[2]) * (1.0 - saturation) + f32::from(s[2]) * saturation),
            ];
        }
        Ok(res.into())
    }

    /// Creates a new Palette by parsing a string of hexadecimal color values.
    ///
    /// The text is split by lines, with one colour per line.
    ///
    /// # Examples
    /// ```
    /// let palette = Palette::from_hex_text("
    /// 2f321b
    /// 64471e
    /// 1c3abf
    /// fdafa0
    /// dec69c
    /// b4c9de",
    /// ).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an Error if it is unable to parse the string.
    pub fn from_hex_text(input: &str) -> Result<Self> {
        let x = input
            .trim()
            .lines()
            .map(|i| {
                let x = u32::from_str_radix(i.trim(), 16)?.to_be_bytes();
                Ok([x[1], x[2], x[3]])
            })
            .collect::<Result<Vec<[u8; 3]>>>()?;

        Ok(x.into())
    }

    /// Loads a palette from a Tinted Scheme
    /// <https://github.com/tinted-theming/schemes>
    ///
    /// # Errors
    ///
    /// This function will return an error if is unable to parse the yaml file.
    pub fn from_tinted_scheme_yaml<P: AsRef<Path> + ToString>(path: &P) -> Result<Self> {
        let fc = std::fs::read_to_string(path)
            .context(format!("Unable to find file '{}'", &path.to_string()))?;
        let lines = fc.lines();
        let mut res = Vec::new();
        for p_line in lines {
            if p_line.trim().starts_with("base") {
                let mut a = p_line.split(':');
                a.next()
                    .context(format!("Bad format, no colon: {p_line:?}"))?;
                let hex_str = a
                    .next()
                    .context(format!("Bad format, after colon: {p_line:?}"))?;
                let hex_str_2 = hex_str
                    .trim()
                    .strip_prefix("\"#")
                    .context(format!("Bad format, no #: {hex_str:?}"))?
                    .strip_suffix("\"")
                    .context("Bad format, missing quote at end")?;
                let x = u32::from_str_radix(hex_str_2.trim(), 16)?.to_be_bytes();
                res.push([x[1], x[2], x[3]]);
            }
        }

        Ok(res.into())
    }

    /// Returns the palette colours.
    #[must_use]
    pub fn get_colours(&self) -> &[Rgb<u8>] {
        &self.0
    }

    /// Finds the closest color in the palette to a given pixel using the specified color space.
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

    /// Returns the index of the palette colour of the pixel provided.
    ///
    /// If the pixel isn't a palette colour, returns 0.
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
