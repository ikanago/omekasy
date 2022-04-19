use clap::Parser;
use font::Font;

use crate::convert::convert;

mod convert;
mod font;

#[derive(Parser)]
#[clap(name = "omekasy")]
#[clap(author = "ikanago <ikanago-dev@protonmail.com>")]
#[clap(version = "0.1.0")]
#[clap(about = "Decorate your input")]
struct Cli {
    #[clap(short, long, arg_enum)]
    font: Font,
    source: String,
}

fn main() {
    let cli = Cli::parse();

    println!("{}", convert(&cli.source, cli.font));
}
