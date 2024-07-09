use image::DynamicImage;
use ndarray::{s, Array2};
use std::f64::consts::PI;

// perform 2D DCT on an array
fn dct2(input: &Array2<f64>) -> Array2<f64> {
    let (rows, cols) = input.dim();
    let mut output = Array2::<f64>::zeros((rows, cols));

    for u in 0..rows {
        for v in 0..cols {
            let mut sum = 0.0;
            for x in 0..rows {
                for y in 0..cols {
                    sum += input[[x, y]]
                        * ((PI * (2 * x + 1) as f64 * u as f64) / (2.0 * rows as f64)).cos()
                        * ((PI * (2 * y + 1) as f64 * v as f64) / (2.0 * cols as f64)).cos();
                }
            }
            let coef = if u == 0 {
                1.0 / (rows as f64).sqrt()
            } else {
                (2.0 / rows as f64).sqrt()
            } * if v == 0 {
                1.0 / (cols as f64).sqrt()
            } else {
                (2.0 / cols as f64).sqrt()
            };
            output[[u, v]] = coef * sum;
        }
    }

    output
}

// perceptual hash as implemented in
// https://www.hackerfactor.com/blog/index.php?/archives/432-Looks-Like-It.html
pub fn phash(img: &DynamicImage, hs: usize, hff: usize) -> Result<Vec<u8>, &str> {
    if hs < 2 {
        return Err("Hash size must be greater than or equal to 2");
    }

    let img_size = hs * hff;

    let img = img.to_luma8();
    let img = image::imageops::resize(
        &img,
        img_size as u32,
        img_size as u32,
        image::imageops::FilterType::Lanczos3,
    );

    let pixels = Array2::from_shape_fn((img_size, img_size), |(x, y)| {
        img.get_pixel(x as u32, y as u32)[0] as f64
    });

    let dctlowfreq = dct2(&pixels).slice(s![..hs, ..hs]).to_owned();
    let med = dctlowfreq.mean().unwrap_or(0.0);
    let mut hash = Vec::with_capacity(hs * hs);
    for &value in dctlowfreq.iter() {
        hash.push(if value > med { 1 } else { 0 });
    }

    Ok(hash)
}

// distance between two hashes
pub fn hdist(hash1: &[u8], hash2: &[u8]) -> usize {
    hash1
        .iter()
        .zip(hash2.iter())
        .filter(|(&a, &b)| a != b)
        .count()
}
