use image::Rgb;

pub trait EuclideanDistance {
    fn distance_sq<T>(&self, c1: Rgb<T>, c2: Rgb<T>) -> f32
    where
        f32: std::convert::From<T>,
        T: Copy;
}

#[derive(Copy, Clone, Debug)]
pub enum ColourSpace {
    RGB,
    CIELAB,
}

impl EuclideanDistance for ColourSpace {
    fn distance_sq<T>(&self, c1: Rgb<T>, c2: Rgb<T>) -> f32
    where
        f32: std::convert::From<T>,
        T: Copy,
    {
        match self {
            ColourSpace::RGB => {
                c1.0.iter()
                    .zip(c2.0)
                    .map(|(&c1_i, c2_i)| (f32::from(c1_i) - f32::from(c2_i)).powi(2))
                    .sum()
            }
            ColourSpace::CIELAB => todo!(),
        }
    }
}
