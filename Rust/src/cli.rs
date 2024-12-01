use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    // Number of the day to run
    #[arg(help = "Number of the day to run")]
    pub day: usize,

    #[arg(
        short('t'),
        long,
        default_value_t = false,
        help = "Whether to use regular or test input"
    )]
    pub use_test: bool,
}
