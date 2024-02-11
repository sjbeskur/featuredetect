//use opencv::highgui::;
use featuredetect::*;
mod image_splitter;

fn main() -> AppResult<> {
    let config = cli::parse_args();
    match config.command{
        cli::Command::Match {path0, path1, matcher, top} => {
            match matcher {
                cli::MatchStrategy::Orb => {
                    orb_matcher::detect_matches(&path0.to_string(), &path1.to_string())?;
                },
                cli::MatchStrategy::Akaze => {
                    akaze_detector::match_features(&path0.to_string(), &path1.to_string(), top)?;
                }
            }                    
        },

        cli::Command::Detect{ show, path, matcher} => {        
            let now = std::time::Instant::now();        
            match matcher {
                cli::MatchStrategy::Orb => {
                    orb_detector::orb_detect(&path.to_string(), show)?;
                },
                cli::MatchStrategy::Akaze => {
                    akaze_detector::detect(&path.to_string(), show)?;
                }
            }
            println!("{:?}: time in millis: {}",matcher, now.elapsed().as_millis());
        },

        cli::Command::Sparse{ path } => {
            image_splitter::split_horizontal(&path.to_string());        
        },
        cli::Command::MonoRange{ show, path} => { 
            //show, path   
            ranging::preproccess(&path.to_string());
        },
    }

    Ok(())
}
