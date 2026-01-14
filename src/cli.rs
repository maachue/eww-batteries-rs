use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[clap(short, long, default_value_t = 1000)]
    pub sleep: u64,
}
