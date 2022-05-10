use clap::ArgEnum;
use clap::Parser;
use font::Font;
use prompt::Prompt;

use crate::convert::Converter;

mod convert;
mod font;
mod prompt;

#[derive(Parser)]
#[clap(author)]
#[clap(version)]
#[clap(about)]
/// Decorate latin alphabet and numbers in your input with various font; special characters in
/// Unicode.
///
/// If you provide neither font type nor input, interactive prompt is displayed.
struct Cli {
    #[clap(short, long, arg_enum)]
    font: Option<Font>,
    input: Option<String>,
}

fn main() -> crossterm::Result<()> {
    let cli: Cli = Cli::parse();

    match (cli.input, cli.font) {
        (Some(input), Some(font)) => {
            let converter = Converter::new(&[font]);
            println!(
                "{}",
                converter.convert(&input.chars().collect::<Vec<_>>(), font)
            );
        }
        (None, None) => {
            let mut prompt = Prompt::new(Font::value_variants());
            prompt.start_prompt()?;
        }
        _ => {
            eprintln!("Specifying only one of font and input is not allowed.");
        }
    }

    Ok(())
}
