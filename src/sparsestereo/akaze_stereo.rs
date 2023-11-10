#![allow(dead_code)]
use super::traits::StereoDetect;
use super::*;


pub struct AkazeStereo{
    cam0_k: KMat,
    cam1_k: KMat,
}

impl AkazeStereo {
    fn new(cam0_k: KMat, cam1_k: KMat) -> Self {
        Self{ cam0_k, cam1_k,}
    }
}


impl StereoDetect for AkazeStereo{    
    #[rustfmt::skip]
    fn detect_points(&self) {
        let projection_matrix_r = ProjectionMatrix::new(504.384982, 0.0,        418.621510, 4266.51293,
            0.0,        504.384982, 182.304560, 0.0,
            0.0,        0.0,        1.0,        0.0 );
    
        let projection_matrix_l = ProjectionMatrix::new(504.38498235, 0.0,          377.38433337,   0.0,
                    0.0,          504.38498235, 182.30455971,   0.0,        
                    0.0,          0.0,          1.0,            0.0 );

        let rot_r = RMatrix::new(504.38498235,    0.0,        312.85543989,
        0.0,         504.38498235, 185.66272695,
        0.0,           0.0,          1.0);

        let rot_l = RMatrix::new(504.38498235,   0.0,         321.92247118,
        0.0,         504.38498235, 188.63481707,
        0.0,           0.0,           1.0);                                                    

        println!("{}",projection_matrix_r);
        println!("{}",projection_matrix_l);

        println!("{}",rot_r);
        println!("{}",rot_l);


    }
}



#[test]
fn test_detection() {
    let cam0k = KMat::new(1.0, 0.0, 0.0, 1.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0);
    
    let cam1k = KMat::new(1.0, 0.0, 0.0, -1.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0);
    
    let akaze_detector = AkazeStereo::new(cam0k, cam1k);
    akaze_detector.detect_points();
}
