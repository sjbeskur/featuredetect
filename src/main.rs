//use opencv::highgui::;
use featuredetect::*;


fn main() -> AppResult<> {
    let config = cli::parse_args();
    match config.command{
        cli::Command::Match {path0, path1} => {
            //let file0 = "img/box.png";
            //let file1 = "img/box_in_scene.png";
            orb_matcher::detect_matches(&path0.to_string(), &path1.to_string())?;
                    
        },

        cli::Command::Detect{ show, path } => {

            
            let now = std::time::Instant::now();        
            orb_detector::orb_detect(&path.to_string(), show)?;
            println!("orb - time in millis: {}", now.elapsed().as_millis());

            let now = std::time::Instant::now();        

            akaze_detector::detect(&path.to_string(), show)?;

            println!("akaze - time in millis: {}", now.elapsed().as_millis());
        
        },

        cli::Command::Sparse{ path } => {

            sparsestereo::split_horizontal(&path.to_string());
        
        }
    }

    Ok(())
}
