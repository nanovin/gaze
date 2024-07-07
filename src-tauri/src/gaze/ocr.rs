use image::DynamicImage;
use rusty_tesseract::{Args, Image};

pub fn image_to_text(img: &DynamicImage) -> String {
    rusty_tesseract::image_to_string(
        &Image::from_dynamic_image(&img).unwrap(),
        &Args {
            lang: "eng".into(),
            psm: Some(6),
            oem: Some(8),
            ..Default::default()
        },
    )
    .unwrap_or("".to_string())
}
