use clap::Parser;
use clap::ValueEnum;
use font::Font;
#[cfg(feature = "crossterm")]
use prompt::Prompt;
use std::error::Error;
use std::io::stdin;
use std::io::Read;

use crate::convert::Converter;

mod convert;
mod font;
#[cfg(feature = "crossterm")]
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
    #[clap(short, long, value_enum)]
    font: Option<Font>,
    input: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli: Cli = Cli::parse();

    match (cli.input, cli.font) {
        (Some(input), Some(font)) => {
            let converter = Converter::new(&[font]);
            println!(
                "{}",
                converter.convert(&input.chars().collect::<Vec<_>>(), font)
            );
        }
        #[cfg(feature = "crossterm")]
        (None, None) => {
            let mut prompt = Prompt::new(Font::value_variants());
            prompt.start_prompt()?;
        }
        #[cfg(not(feature = "crossterm"))]
        (None, None) => {
            return Err("Compiled without terminal support. Please specify the font as a command line parameter".into());
        }
        (Some(input), None) => {
            let fonts = Font::value_variants();
            let converter = Converter::new(fonts);
            for &font in fonts {
                println!(
                    "{}",
                    converter.convert(&input.chars().collect::<Vec<_>>(), font)
                );
            }
        }
        (None, Some(font)) => {
            let converter = Converter::new(&[font]);
            let mut input = String::new();
            stdin().read_to_string(&mut input)?;
            print!(
                "{}",
                converter.convert(&input.chars().collect::<Vec<_>>(), font)
            );
        }
    }

    Ok(())
}
