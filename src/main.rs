//use opencv::highgui::;
use featuredetect::*;

fn main() {

    let config = cli::parse_args();
    orb_detect(&config.filename)

}
