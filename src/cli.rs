use clap::{Parser, Subcommand};
use camino::Utf8PathBuf;


#[derive(Debug, Parser)]
#[command(
    author = "Sam Beskur <sam.beskur@gmail.com>",
    version,
    about = "Feature Detector",
    long_about = "A very basic demonstration of OpenCV Feature Detector(s)."
)]
pub struct Config {

    #[clap(subcommand)]
    pub command: Command,

    //pub filename: String,
    //    #[arg(short = 'c', long = "connectivity", default_value_t = 4 )]
    //    pub connectivity: u8,  // TODO: figure out how to only allow 4 or 8?
}

pub fn parse_args() -> Config {
    let config = Config::parse();
    return config;
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Match features between two input images
    Match {
        #[clap(long, short = 'm', value_enum)]
        matcher: MatchStrategy,

        /// The path to read from
        path0: Utf8PathBuf,

        path1: Utf8PathBuf,
        // (can #[clap(flatten)] other argument structs here)

        #[clap(long, short = 't', default_value_t=10)]
        top: i32,
    },
    /// Detect features using ORB and Akaze Feature Detectors.
    Detect{

        #[clap(long, short = 'm', value_enum)]
        matcher: MatchStrategy,
        
        #[clap(long, short = 's', default_value_t = false)]
        show: bool,
        /// The path to read from
        path: Utf8PathBuf,

    },
    
    Sparse{
        /// The path to read from
        path: Utf8PathBuf,

    },

    MonoRange{        
        #[clap(long, short = 's', default_value_t = false)]
        show: bool,
        /// The path to read from
        path: Utf8PathBuf,

    }
    // ...other commands (can #[clap(flatten)] other enum variants here)

}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum MatchStrategy{
    Orb,
    Akaze,
}
