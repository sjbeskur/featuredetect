use opencv::{
    core::{Scalar, Mat, NORM_HAMMING, no_array, Vector, Ptr, KeyPoint},
    features2d::*,
    types::* ,
    highgui::imshow,  //{named_window, WindowFlags},
    imgcodecs::{imread, IMREAD_GRAYSCALE},  
};


pub struct AKAZESparseStereo{

    detector: Ptr<AKAZE>,    
    
}

impl AKAZESparseStereo{

    pub fn create() -> AKAZESparseStereo{ 
        let descriptor_type = AKAZE_DescriptorType::DESCRIPTOR_MLDB;
        let descriptor_size = 0;
        let descriptor_channels = 3;
        let threshold =  0.001_f32;
        let n_octaves =  4;
        let n_octave_layers = 4;    
        let diffusivity = KAZE_DiffusivityType::DIFF_PM_G2;

        Self{
            detector: AKAZE::create(
                descriptor_type,
                descriptor_size,
                descriptor_channels,
                threshold,
                n_octaves,
                n_octave_layers,
                diffusivity,
            ).unwrap()
        }
    }

    pub fn detect_and_compute(&mut self, image: &Mat, kp: &mut Vector<KeyPoint>, desc: &mut Mat){
        let mask = Mat::default();
        let _ = self.detector.detect_and_compute(&image, &mask, kp, desc, false);
    
    }
}



pub fn detect(file0: &str, show_img: bool) -> super::AppResult<> {

    let mut akaze = AKAZESparseStereo::create();
    let img0 = imread(file0, IMREAD_GRAYSCALE)?;
    //let img1 = imread(file1, IMREAD_GRAYSCALE)?;
    let mut kp_a = VectorOfKeyPoint::new();
    let mut des_a = Mat::default();

    let _ = akaze.detect_and_compute(&img0, &mut kp_a, &mut des_a);

    if show_img{    
        super::img_util::show_keypoint(img0, kp_a)?;
    }

    Ok(())
}


pub fn match_features(file0: &str, file1: &str) -> Result<(), Box<dyn std::error::Error>>{
    let img0 = imread(file0, IMREAD_GRAYSCALE)?;
    let img1 = imread(file1, IMREAD_GRAYSCALE)?;

    let mut akaze = AKAZESparseStereo::create();


    let mut kp0 = VectorOfKeyPoint::new();
    let mut des0 = Mat::default();
    let _ = akaze.detect_and_compute(&img0, &mut kp0, &mut des0);

    let mut kp1 = VectorOfKeyPoint::new();
    let mut des1 = Mat::default();
    let _ = akaze.detect_and_compute(&img1, &mut kp1, &mut des1);


    let bf_matcher = BFMatcher::new(NORM_HAMMING, true)?;
    
    let mut matches = VectorOfDMatch::new();
    bf_matcher.train_match(&des0, &des1, &mut matches, &no_array())?;

    let mut v = matches.to_vec();
    v.sort_by(|a,b| a.distance.partial_cmp(&b.distance).unwrap());
    let top_n = Vector::from_slice(&v[0..10]);


    let mut out_image = Mat::default();
    draw_matches(&img0,
        &kp0,
        &img1, 
        &kp1,
        &top_n, 
        &mut out_image,
        Scalar::all(-1.0),
        Scalar::all(-1.0),
        &Vector::<i8>::new(),
        DrawMatchesFlags::DEFAULT,
    )?;

    imshow("Matches", &out_image)?;
    opencv::highgui::wait_key(0)?;

    Ok(())

}