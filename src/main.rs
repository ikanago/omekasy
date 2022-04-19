use clap::Parser;
use font::Font;
use prompt::Prompt;

mod convert;
mod font;
mod prompt;

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

fn main() -> crossterm::Result<()> {
    // let cli = Cli::parse();

    // println!("{}", convert(&cli.source, cli.font));

    let fonts = vec![Font::MathBold, Font::Monospace];
    let mut prompt = Prompt::new(fonts);
    prompt.start_prompt()
}
