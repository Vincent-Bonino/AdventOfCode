use clap::{Args, Parser, Subcommand};

#[derive(Clone, Debug, Eq, PartialEq, Parser)]
pub struct Aoc25Args {
    #[command(subcommand)]
    pub command: Aoc25Command,
}

#[derive(Clone, Debug, Eq, PartialEq, Subcommand)]
pub enum Aoc25Command {
    #[clap(visible_aliases = &["d", "day", "s"])]
    SingleDay(SingleDayArgs),

    #[clap(visible_aliases = &["a", "all"])]
    AllDays(AllDaysArgs),
}

#[derive(Args, Clone, Debug, Eq, PartialEq)]
pub struct SingleDayArgs {
    /// Number of the day to run
    pub day: usize,

    #[arg(short('t'), long, default_value_t = false)]
    /// Whether to use regular or test input.
    pub use_test: bool,

    #[arg(long, requires = "use_test")]
    pub test_extra: Option<String>,
}

#[derive(Args, Clone, Debug, Eq, PartialEq)]
pub struct AllDaysArgs;
