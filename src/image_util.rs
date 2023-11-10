use opencv::{
    core::{Scalar, Mat, Vector, KeyPoint },
    features2d::*,
    highgui::{imshow, named_window, WindowFlags},
};


pub fn show_keypoint(img: Mat, keypoints: Vector<KeyPoint>) -> super::AppResult<> {
    let red_color = Scalar::new(0.0, 0.0, 255.0, 0.0);
    let mut out_img = img.clone();

    draw_keypoints(&img, &keypoints, &mut out_img, red_color, DrawMatchesFlags::DEFAULT)?;
    named_window("Display Window", WindowFlags::WINDOW_NORMAL as i32)?;
    imshow("Display Window", &out_img)?;
    opencv::highgui::wait_key(0)?;
    Ok(())
}
