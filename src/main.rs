//use opencv::highgui::;
use featuredetect::*;

fn main() {
    let config = cli::parse_args();
    orb_detector::orb_detect(&config.filename);


    let file0 = "img/box.png";
    let file1 = "img/box_in_scene.png";
    orb_matcher::detect_matches(&file0, &file1);
}
