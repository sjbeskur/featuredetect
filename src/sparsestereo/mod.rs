
pub mod triangulate;


use image::{GenericImageView, ImageFormat, open, save_buffer_with_format}; //DynamicImage, 
 

pub fn split_horizontal(img_file: &str){
    let img = open(img_file).unwrap();

    // Get the dimensions of the image
    let (width, height) = img.dimensions();
    println!("w: {} h: {}", width, height);

    // Split the image in half horizontally
    let left_half = img.view(0, 0, width / 2, height);
    let right_half = img.view(width / 2, 0, width / 2, height);

    let left_half_raw = left_half.to_image();
    let (w,h) = left_half_raw.dimensions();
    println!("w: {} h: {}", w, h);
    let left_half_raw = left_half_raw.as_raw();
    let right_half_raw = right_half.to_image();
    let right_half_raw = right_half_raw.as_raw();

    // Save the two halves as separate images
    save_buffer_with_format("left_half.png", left_half_raw, w, h,  image::ColorType::La16, ImageFormat::Png).unwrap();
    save_buffer_with_format("right_half.png", right_half_raw, w, h, image::ColorType::La16, ImageFormat::Png).unwrap();

}

#[test]
fn test_split_horizontal(){
    assert_eq!(true, true);
}