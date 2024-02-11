use opencv::{
    core::*, gapi::threshold, highgui::{imshow, named_window, WindowFlags}, imgcodecs::{imread, IMREAD_GRAYSCALE}, imgproc::{self, canny, find_contours, gaussian_blur} 
};

use crate::AppResult;

/*
https://avisingh599.github.io/vision/monocular-vo/
Capture images: It, It+1
,
Undistort the above images.
Use FAST algorithm to detect features in It
, and track those features to It+1
. A new detection is triggered if the number of features drop below a certain threshold.
Use Nister’s 5-point alogirthm with RANSAC to compute the essential matrix.
Estimate R,t
 from the essential matrix that was computed in the previous step.
Take scale information from some external source (like a speedometer), and concatenate the translation vectors, and rotation matrices.

*/


/*
def find_marker(image):
	# convert the image to grayscale, blur it, and detect edges
	gray = cv2.cvtColor(image, cv2.COLOR_BGR2GRAY)
	gray = cv2.GaussianBlur(gray, (5, 5), 0)
	edged = cv2.Canny(gray, 35, 125)
	# find the contours in the edged image and keep the largest one;
	# we'll assume that this is our piece of paper in the image
	cnts = cv2.findContours(edged.copy(), cv2.RETR_LIST, cv2.CHAIN_APPROX_SIMPLE)
	cnts = imutils.grab_contours(cnts)
	c = max(cnts, key = cv2.contourArea)
	# compute the bounding box of the of the paper region and return it
	return cv2.minAreaRect(c)
*/
pub fn preproccess(image_file: &str) -> Result<(), Box<dyn std::error::Error>>{
    let src = imread(image_file, IMREAD_GRAYSCALE)?;
    let mut blurred = Mat::default();
    let blur_kernel =Size{ width: 5, height: 5};
    let _ = gaussian_blur(&src,&mut blurred, blur_kernel, 1.0,1.0, BORDER_DEFAULT )?;

    println!("[canny] Running canny Edge Detection.");
    let mut canny_output: Mat = Mat::default(); 
    canny(&blurred, &mut canny_output , 50.0, 150.0, 3, false)?;
    
    println!("[contours] Finding contours...");
    let mut contours = opencv::types::VectorOfMat::new(); 
    //let none = imgproc::CHAIN_APPROX_NONE;
    //let tc89_kcos = imgproc::CHAIN_APPROX_TC89_KCOS;
    //let l_one = imgproc::CHAIN_APPROX_TC89_L1;    
    imgproc::find_contours(&canny_output, &mut contours, imgproc::RETR_TREE, imgproc::CHAIN_APPROX_SIMPLE, Point::new(0,0))?;
    println!("Contours found detected: {}.", contours.to_vec().len());

    let mut thresh = Mat::default();
    imgproc::threshold(&src, &mut thresh, 0.0, 65535.0, imgproc::THRESH_BINARY + imgproc::THRESH_OTSU)?;


    let mut drawing = Mat::default(); 
    let color = Scalar::new(64.0, 64.0, 0.0, 0.0);
    let thickness: i32 = 4;    
    let maxresult = 0;
    let zero_offset = Point::new(0, 0);
    for (i, _) in contours.iter().enumerate() {
        let contour_idx = i as i32;
        let  _ = imgproc::draw_contours(&mut drawing, &contours, contour_idx , color, thickness, imgproc::LINE_AA, &opencv::core::no_array(), maxresult, zero_offset)?;
        //let _ = imgproc::circle(&mut drawing, mc[i], 40, color, 3, 8, 0 )?;
    }    

    println!("Opening Display");
    //named_window("Display Window", WindowFlags::WINDOW_NORMAL as i32).unwrap();
    imshow("Display Window", &thresh).unwrap();
    opencv::highgui::wait_key(0).unwrap();

    Ok(())
}

/*
def distance_to_camera(knownWidth, focalLength, perWidth):
	# compute and return the distance from the maker to the camera
	return (knownWidth * focalLength) / perWidth
*/