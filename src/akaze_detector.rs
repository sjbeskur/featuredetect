use opencv::{
    core::Mat,
    features2d::*,
    types::* ,
    imgcodecs::{imread, IMREAD_GRAYSCALE},    
};



pub fn detect(file0: &str, show_img: bool) -> super::AppResult<> {

    let descriptor_type = AKAZE_DescriptorType::DESCRIPTOR_MLDB;
    let descriptor_size = 0;
    let descriptor_channels = 3;
    let threshold =  0.001_f32;
    let n_octaves =  4;
    let n_octave_layers = 4;    
    let diffusivity = KAZE_DiffusivityType::DIFF_PM_G2;

    let mut akaze = AKAZE::create(
        descriptor_type,
        descriptor_size,
        descriptor_channels,
        threshold,
        n_octaves,
        n_octave_layers,
        diffusivity,
    ).unwrap();

    let img0 = imread(file0, IMREAD_GRAYSCALE)?;
    //let img1 = imread(file1, IMREAD_GRAYSCALE)?;

    let mask = Mat::default();
    let mut kp_a = VectorOfKeyPoint::new();
    let mut des_a = Mat::default();

    let _ = akaze.detect_and_compute(&img0, &mask, &mut kp_a, &mut des_a, false);

    if show_img{    
        super::img_util::show_keypoint(img0, kp_a)?;
    }

    Ok(())
}


