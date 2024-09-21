use crate::utils::to_string;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliOptions {
    /// Shape to process (e.g., "cube", "tetrahedron")
    #[arg(default_value_t = to_string("cube"))]
    pub shape: String, // This is a positional argument

    /// X-axis value for the solid (default is 1, must be between -10 and 10)
    #[arg(default_value_t = 1, value_parser = -10..=10)]
    pub x: i64,

    /// Y-axis value for the solid (default is 1, must be between -10 and 10)
    #[arg(default_value_t = 1, value_parser = -10..=10)]
    pub y: i64,

    /// Z-axis value for the solid (default is 1, must be between -10 and 10)
    #[arg(default_value_t = 1, value_parser = -10..=10)]
    pub z: i64,
}
