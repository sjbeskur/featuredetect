use std::error::Error;
use ndarray::ArrayView2;
use opencv as cv;
use cv::prelude::*;

pub mod cli;
pub mod orb_matcher;
pub mod orb_detector;



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