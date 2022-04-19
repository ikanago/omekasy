use std::io::{self, Write};

use clap::Parser;
use font::Font;
use termion::{cursor, event::Key, input::TermRead, raw::IntoRawMode, clear};

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
    // let cli = Cli::parse();

    // println!("{}", convert(&cli.source, cli.font));

    let stdin = io::stdin();
    let mut stdin = stdin.lock().keys();
    let stdout = io::stdout();
    let stdout = stdout.lock();
    let mut stdout = stdout.into_raw_mode().unwrap();

    let mut source = Vec::new();

    loop {
        let b = stdin.next().unwrap().unwrap();
        match b {
            Key::Backspace => {
                write!(stdout, "{}{}{}", cursor::Left(1), clear::AfterCursor, cursor::Save).unwrap();
                source.pop();
            }
            Key::Char('q') => {
                break;
            }
            Key::Char(c) => {
                write!(stdout, "{}{}", c, cursor::Save).unwrap();
                source.push(c);
            }
            _ => {}
        }
        write!(stdout, "\r\n{}", clear::AfterCursor).unwrap();
        write!(stdout, "{}", convert(&source, Font::MathBold)).unwrap();
        write!(stdout, "{}", cursor::Restore).unwrap();
        stdout.flush().unwrap();
    }
}
