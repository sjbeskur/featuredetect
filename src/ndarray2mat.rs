use std::error::Error;
use ndarray::ArrayView2;
use opencv as cv;
use cv::prelude::*;


pub trait ToNDArray {
    fn try_as_array(&self) -> Result<ArrayView2<u8>, Box<dyn Error>>;
}

impl ToNDArray for cv::core::Mat {
    fn try_as_array(&self) -> Result<ArrayView2<u8>, Box<dyn Error>> {
        if !self.is_continuous() {
            return Err("Mat is not continuous".into());  // sanity check we hope never happens
        }
        let bytes = self.data_bytes()?;
        let size = self.size()?;
        let a = ArrayView2::from_shape((size.height as usize, size.width as usize), bytes)?;
        Ok(a)
    }
}

pub trait ToNAlgebra{
    //fn try_as_array(&self) -> Result<ArrayView2<u8>, Box<dyn Error>>;
}


#[test]
fn test_to_ndarray() -> Result<(), Box<dyn std::error::Error>>{

    use opencv::imgcodecs::{imread, IMREAD_GRAYSCALE};

    let file = "img/sjb-aerial.png";
    let img = imread(file, IMREAD_GRAYSCALE)?;
    let img_nd = img.try_as_array()?;
    println!("dimensions: {:?}", img_nd.shape());
    println!("dimensions: {:?}", img_nd);
    assert_eq!(img_nd.shape(), [749,502]);
    assert_eq!(img.rows(), 749);
    assert_eq!(img.cols(), 502);
    Ok(())
}