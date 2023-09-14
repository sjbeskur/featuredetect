use opencv::{
    core::*,
    features2d::*,
    highgui::{imshow, named_window, WindowFlags},
    imgcodecs::{imread, IMREAD_GRAYSCALE},
};


pub fn orb_detect(image_file: &str) -> Result<(), Box<dyn std::error::Error>> {
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

    let mut mask = Mat::default();
    my_orb.detect(&img, &mut keypoints, &mask)?;

    let count = keypoints.len();
    println!("Number of keypoints: {}", count);

    // compute the descriptors with ORB
    my_orb.compute(&img, &mut keypoints, &mut mask)?;

    let red_color = Scalar::new(0.0, 0.0, 255.0, 0.0);
    let mut out_img = img.clone();
    draw_keypoints(&img, &keypoints, &mut out_img, red_color, DrawMatchesFlags::DEFAULT)?;

    // Create a named window to display the image
    named_window("Display Window", WindowFlags::WINDOW_NORMAL as i32)?;

    // Display the image
    imshow("Display Window", &out_img)?;

    // Wait for a key press (0 means wait indefinitely)
    opencv::highgui::wait_key(0)?;
    Ok(())
}
