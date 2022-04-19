use std::{
    io::{self, Write},
    time::Duration,
};

use clap::Parser;
use crossterm::{
    cursor::{MoveLeft, MoveRight, MoveToNextLine, MoveToPreviousLine},
    event::{poll, read, Event, KeyCode, KeyEvent},
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    ExecutableCommand, QueueableCommand,
};
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

fn main() -> crossterm::Result<()> {
    // let cli = Cli::parse();

    // println!("{}", convert(&cli.source, cli.font));

    let mut stdout = io::stdout();

    let mut source = Vec::new();
    let fonts = vec![Font::MathBold, Font::Monospace];
    let mut current_font = 0;

    enable_raw_mode()?;

    let num_whole_lines = fonts.len() + 1;
    for _ in 0..num_whole_lines {
        stdout.queue(Print("\r\n"))?;
    }
    stdout.queue(MoveToPreviousLine(num_whole_lines as u16))?;
    stdout.flush()?;

    loop {
        let mut has_updated = false;

        if poll(Duration::from_millis(50))? {
            if let Event::Key(KeyEvent { code, .. }) = read()? {
                match code {
                    KeyCode::Backspace => {
                        stdout.execute(MoveLeft(1))?;
                        source.pop();
                    }
                    KeyCode::Up => {
                        if current_font > 0 {
                            current_font -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if current_font < fonts.len() - 1 {
                            current_font += 1;
                        }
                    }
                    KeyCode::Enter => {
                        break;
                    }
                    KeyCode::Char(c) => {
                        source.push(c);
                    }
                    _ => {}
                }

                has_updated = true;
            }
        }

        if has_updated {
            stdout
                .execute(MoveLeft(source.len() as u16))?
                .execute(Clear(ClearType::CurrentLine))?
                .execute(Print(source.iter().collect::<String>()))?;

            for (i, &font) in fonts.iter().enumerate() {
                let selection = if i == current_font { 'x' } else { ' ' };
                stdout
                    .queue(MoveToNextLine(1))?
                    .queue(Clear(ClearType::CurrentLine))?
                    .queue(Print(format!("[{}]{}", selection, convert(&source, font))))?;
            }
            stdout.flush()?;
            stdout
                .execute(MoveToPreviousLine((num_whole_lines - 1) as u16))?
                .execute(MoveRight(source.len() as u16))?;
        }
    }

    stdout
        .execute(MoveLeft(source.len() as u16))?
        .execute(Clear(ClearType::CurrentLine))?
        .execute(Print(format!(
            "{}\r\n",
            convert(&source, fonts[current_font])
        )))?
        .execute(Clear(ClearType::FromCursorDown))?;

    disable_raw_mode()?;

    Ok(())
}
