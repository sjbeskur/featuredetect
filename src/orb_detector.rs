use opencv::{
    core::*,
    features2d::*,
    imgcodecs::{imread, IMREAD_GRAYSCALE},
};


pub fn orb_detect(image_file: &str, show_img: bool) -> Result<(), Box<dyn std::error::Error>> {
    let img = imread(image_file, IMREAD_GRAYSCALE).unwrap();

    let nfeatures = 500;
    let scale_factor: f32 = 1.2;
    let nlevels = 8;
    let edge_threshold = 31;
    let first_level = 0;
    let wta_k = 2;
    let score_type = ORB_ScoreType::HARRIS_SCORE;
    let patch_size = 31;
    let fast_threshold = 20;
    let mut my_orb = ORB::create(
        nfeatures,
        scale_factor,
        nlevels,
        edge_threshold,
        first_level,
        wta_k,
        score_type,
        patch_size,
        fast_threshold,
    )?;

    let mut keypoints = opencv::core::Vector::new();

    let mut desc = Mat::default();
    let mask = Mat::default();
    //my_orb.detect(&img, &mut keypoints, &mask)?;

    //let count = keypoints.len();
    //println!("Number of keypoints: {}", count);

    // compute the descriptors with ORB
    my_orb.compute(&img, &mut keypoints, &mut desc)?;
    my_orb.detect_and_compute(&img, &mask, &mut keypoints, &mut desc, false)?;

    
    // Uncomment this to show the image with keypoints
    if show_img{    
        super::image_util::show_keypoint(img, keypoints)?;
    }

    Ok(())
}
