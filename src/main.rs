extern crate image;

use image::{GenericImageView, ImageBuffer, RgbaImage, Rgba};

const MAX_DELTA: f64 = 35215.0;

fn as_list(rgba: &Rgba<u8>) -> (f64, f64, f64, f64) {
    (rgba[0] as f64, rgba[1] as f64, rgba[2] as f64, rgba[3] as f64)
}

fn rgb2y(r: &f64, g: &f64, b: &f64) -> f64 {
    r * 0.29889531 + g * 0.58662247 + b * 0.11448223
}

fn rgb2i(r: &f64, g: &f64, b: &f64) -> f64 {
    r * 0.59597799 - g * 0.27417610 - b * 0.32180189
}

fn  rgb2q(r: &f64, g: &f64, b: &f64) -> f64 {
    r * 0.21147017 - g * 0.52261711 + b * 0.31114694
}

// delta between two pixels.
fn delta(i1: &Rgba<u8>, i2: &Rgba<u8>) -> f64 {
    let (r1, g1, b1, _) = as_list(i1);
    let (r2, g2, b2, _) = as_list(i2);

    let y: f64 = rgb2y(&r1, &g1, &b1) - rgb2y(&r2, &g2, &b2);
    let i: f64 = rgb2i(&r1, &g1, &b1) - rgb2i(&r2, &g2, &b2);
    let q: f64 = rgb2q(&r1, &g1, &b1) - rgb2q(&r2, &g2, &b2);

    0.5053 * y * &y + 0.299 * i * &i + 0.1957 *q * &q
}

fn main() {
    let max_delta: f64 = MAX_DELTA * 0.1 * 0.1;

    let i1 = image::open("examples/www.cypress.io.png").unwrap();
    let i2 = image::open("examples/www.cypress.io-1.png").unwrap();

    let (width, height) = i1.dimensions();
    let mut diff: RgbaImage = ImageBuffer::new(width, height);

    let liza_1 = i1.to_rgba8();
    let liza_2 = i2.to_rgba8();

    let mut diff_pixels_counter: i64 = 0;

    for (x, y, pixel) in diff.enumerate_pixels_mut() {
        let pixel_1 = liza_1.get_pixel(x, y);
        let pixel_2 = liza_2.get_pixel(x, y);

        if pixel_1 != pixel_2 {
            let delta: f64 = delta(pixel_1, pixel_2);

            if delta > max_delta {
                *pixel = Rgba([255, 0, 0, 255]);
                diff_pixels_counter += 1
            }
        } else if true == false {
            *pixel = *pixel_1
        }
    }

    diff.save("examples/donkey_diff.png").unwrap();

    println!("Total diff count: {}", diff_pixels_counter);
    println!("Target image dimensions {:?}", liza_1.dimensions());
    println!("Compare image dimensions {:?}", liza_2.dimensions());
}
