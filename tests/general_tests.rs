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

    let data: Vec<f32> = img_gray
        .pixels()
        .flat_map(|p| vec![p[0] as f32]) // Assuming grayscale, only one channel
        .collect();

    let mut img = DMatrix::from_row_slice(rows, cols,&data);

    // faking it for test but this is sort of what we need for NUC
    // y = mx + b  
    let corrected = non_uniform_correct(&mut img);
    Ok(())
}

fn non_uniform_correct(img: &mut DMatrix<f32>) -> DMatrix<f32>{
    let result = img.component_mul(&img);  // fake gain 
    let offset = img.clone(); // fake the offset this should be loaded from config;
    result + offset
}