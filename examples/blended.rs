use image::GenericImageView;
use image::imageops::FilterType;
use inky::ColourSpace;
use inky::Palette;
use inky::quantise_and_dither_image;

fn main() {
    let palette = Palette::from_blend(
        [
            [0, 0, 0],
            [255, 255, 255],
            [255, 255, 0],
            [255, 0, 0],
            [0, 0, 255],
            [0, 255, 0],
        ]
        .as_slice(),
        [
            [0, 0, 0],
            [161, 164, 165],
            [208, 190, 71],
            [156, 72, 75],
            [61, 59, 94],
            [58, 91, 70],
        ]
        .as_slice(),
        0.7,
    );
    let mut args = std::env::args();
    args.next(); // throw away program name
    let input_path = args.next().expect("usage: inky <input_path> <output_path>");
    let output_path = args.next().expect("usage: inky <input_path> <output_path>");

    let img = image::open(&input_path).unwrap();
    println!(
        "Using image '{}' with dimensions: {:?}",
        &input_path,
        img.dimensions()
    );
    let img = img.resize(800, 400, FilterType::Nearest);
    let buf = img.to_rgb8();
    println!("Dithering...");
    let res = quantise_and_dither_image(&buf, &palette, ColourSpace::RGB);
    println!("Done");

    res.save(&output_path).unwrap();
    println!("Wrote output to '{}'", &output_path);
}
