use clap::Parser;

#[derive(Parser)]
#[command(name = "example")]
#[command(about="An example of using clap with a struct", long_about = None)]
pub(crate) struct Cli {
    #[arg(long)]
    pub long: bool,
    #[arg(long)]
    pub debug: bool,

    #[arg(short, long, default_value = ".")]
    pub path: String,
}
