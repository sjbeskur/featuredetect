use std::io::Cursor;
use image::{io::Reader as ImageReader, GenericImageView};
use nalgebra::DMatrix;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn test_image_to_na() -> TestResult {

    let img = ImageReader::open("img/box.png")?.decode()?;

    let dims = img.dimensions();
    let rows = dims.1 as usize;
    let cols = dims.0 as usize;
    let img_gray = img.into_luma8(); // grayscale image

    let data: Vec<f64> = img_gray
        .pixels()
        .flat_map(|p| vec![p[0] as f64]) // Assuming grayscale, only one channel
        .collect();

    let img = DMatrix::from_row_slice(rows, cols,&data);

    // faking it for test but this is sort of what we need for NUC
    // y = mx + b  
    let result = img.component_mul(&img);
    let result = result + img;

    Ok(())
}