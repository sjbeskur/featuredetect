//use opencv::highgui::;
use featuredetect::*;

type AppResult = Result<(), Box<dyn std::error::Error>>;

fn main() -> AppResult<> {
    let config = cli::parse_args();
    match config.command{
        cli::Command::Match {path0, path1} => {
            //let file0 = "img/box.png";
            //let file1 = "img/box_in_scene.png";
            orb_matcher::detect_matches(&path0.to_string(), &path1.to_string())?;
                    
        },

        cli::Command::Detect{ path } => {
            orb_detector::orb_detect(&path.to_string())?;
        
        }
    }

    Ok(())
}
