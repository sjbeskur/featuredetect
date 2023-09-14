use super::*;

use opencv::{
    core::{Scalar, Mat, NORM_HAMMING, no_array, Vector},
    features2d::*,
    types::{VectorOfDMatch, VectorOfKeyPoint},
    highgui::imshow,  //{named_window, WindowFlags},
    imgcodecs::{imread, IMREAD_GRAYSCALE},
};


pub fn detect_matches(file0: &str, file1: &str ) -> Result<(), Box<dyn std::error::Error>> {
    
    let img0 = imread(file0, IMREAD_GRAYSCALE)?;
    let img1 = imread(file1, IMREAD_GRAYSCALE)?;

    let mut orb = ORB::default()?;
    let mask = Mat::default(); // or no_array(); ??

    let mut kp_a = VectorOfKeyPoint::new();
    let mut des_a = Mat::default();

    //let _ = orb.compute(&img0, &mut kp_a, &mut des_a);
    orb.detect_and_compute(&img0, &mask, &mut kp_a, &mut des_a, false)?;

    let mut kp_b = VectorOfKeyPoint::new();
    let mut des_b = Mat::default();
    orb.detect_and_compute(&img1, &mask, &mut kp_b, &mut des_b, false)?;

    // let mut bf_matcher = DescriptorMatcher::create("BruteForce-Hamming")?;
    let bf_matcher = BFMatcher::new(NORM_HAMMING, true)?;
    
    let mut matches = VectorOfDMatch::new();
    bf_matcher.train_match(&des_a, &des_b, &mut matches, &no_array())?;

    println!("\n MATHES : {} --------------------" , matches.len());

    let mut v = matches.to_vec();
    v.sort_by(|a,b| a.distance.partial_cmp(&b.distance).unwrap());
    let top_n = Vector::from_slice(&v[0..10]);

    let mut out_image = Mat::default();
    draw_matches(&img0,
        &kp_a,
        &img1, 
        &kp_b,
        &top_n, 
        &mut out_image,
        Scalar::all(-1.0),
        Scalar::all(-1.0),
        &Vector::<i8>::new(),
        DrawMatchesFlags::DEFAULT,
    )?;

    // // Show matches
    imshow("Matches", &out_image)?;
    opencv::highgui::wait_key(0)?;


    Ok(())
}
