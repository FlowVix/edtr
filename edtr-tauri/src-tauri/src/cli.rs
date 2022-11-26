use clap::Parser;

#[derive(Parser, Debug)]
pub struct CliArgs {
    #[arg(short, long)]
    pub debug: bool,

    #[arg(long)]
    pub devtools: bool,

    #[arg(short, long)]
    pub safe: bool,
}
