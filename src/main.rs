//use opencv::highgui::;
use featuredetect::*;


fn main() -> AppResult<> {
    let config = cli::parse_args();
    match config.command{
        cli::Command::Match {path0, path1, matcher} => {

            match matcher {
                cli::MatchStrategy::Orb => {
                    orb_matcher::detect_matches(&path0.to_string(), &path1.to_string())?;
                },
                cli::MatchStrategy::Akaze => {
                    akaze_detector::match_features(&path0.to_string(), &path1.to_string())?;
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

            sparsestereo::split_horizontal(&path.to_string());
        
        }
    }

    Ok(())
}
