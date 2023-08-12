use clap::{Parser};

#[derive(Debug, Parser)]
#[command(author="Sam Beskur <sam.beskur@gmail.com>", version, about="Feature Detector", long_about = "A very basic demonstration of OpenCV Feature Detector(s).")]
pub struct Config{
    pub filename: String,

//    #[arg(short = 'c', long = "connectivity", default_value_t = 4 )]
//    pub connectivity: u8,  // TODO: figure out how to only allow 4 or 8?
}

pub fn parse_args()-> Config{
    let config = Config::parse();
    return config;
}
