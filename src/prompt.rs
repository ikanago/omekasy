use std::{
    io::{self, Stderr, Stdout, Write},
    marker::PhantomData,
    time::Duration,
};

use crate::convert::Converter;
use crate::font::Font;

use crossterm::{
    cursor::{MoveLeft, MoveRight, MoveToNextLine, MoveToPreviousLine},
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    style::{Print, Stylize},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    ExecutableCommand, QueueableCommand,
};

/// Trait that abstructs collecting input key event.
pub trait IoEnvironment<Intermediate: Write, Output: Write> {
    fn feed(&mut self) -> crossterm::Result<Option<KeyEvent>>;

    /// Returns a channel to output intermediate prompt.
    fn intermediate_output(&self) -> Intermediate;

    /// Returns a channel to write final result on.
    fn output(&self) -> Output;
}

/// Marker struct to collect input key event from stdin by polling and output to stderr/stdout.
pub struct StdIo;

impl StdIo {
    const POLL_DURATION_MS: u64 = 50;
}

impl IoEnvironment<Stderr, Stdout> for StdIo {
    fn feed(&mut self) -> crossterm::Result<Option<KeyEvent>> {
        if poll(Duration::from_millis(Self::POLL_DURATION_MS))? {
            if let Event::Key(event) = read()? {
                return Ok(Some(event));
            }
        }
        Ok(None)
    }

    fn intermediate_output(&self) -> Stderr {
        io::stderr()
    }

    fn output(&self) -> Stdout {
        io::stdout()
    }
}

pub enum Action {
    Confirm,
    Quit,
    Update,
    None,
}

pub struct Prompt<Environment, Intermediate, Output>
where
    Environment: IoEnvironment<Intermediate, Output>,
    Intermediate: Write,
    Output: Write,
{
    input: Vec<char>,
    fonts: &'static [Font],
    converter: Converter,
    environment: Environment,
    current_font: usize,
    num_whole_lines: usize,

    _intermediate: PhantomData<Intermediate>,
    _output: PhantomData<Output>,
}

impl<Environment, Intermediate, Output> Prompt<Environment, Intermediate, Output>
where
    Environment: IoEnvironment<Intermediate, Output>,
    Intermediate: Write,
    Output: Write,
{
    const PROMPT_SYMBOL: &'static str = "> ";

    pub fn new(fonts: &'static [Font], environment: Environment) -> Self {
        let converter = Converter::new(fonts);
        let num_whole_lines = fonts.len() + 1;

        Self {
            input: Vec::new(),
            fonts,
            converter,
            environment,
            current_font: 0,
            num_whole_lines,
            _intermediate: PhantomData,
            _output: PhantomData,
        }
    }

    /// Start event loop to wait for user input and render output.
    pub fn start_prompt(&mut self) -> crossterm::Result<()> {
        enable_raw_mode()?;

        let mut intermediate_output = self.environment.intermediate_output();
        self.initialize_prompt(&mut intermediate_output)?;
        self.render_input(&mut intermediate_output)?;

        self.start_event_loop(&mut intermediate_output)?;

        self.environment
            .output()
            .execute(MoveLeft(self.input_line_len()))?
            .execute(Clear(ClearType::CurrentLine))?
            .execute(Print(format!(
                "{}\r\n",
                self.converter
                    .convert(&self.input, self.fonts[self.current_font])
            )))?
            .execute(Clear(ClearType::FromCursorDown))?;

        disable_raw_mode()?;

        Ok(())
    }

    /// Ahead of event loop, reserve lines to render candidate outputs;
    /// `SavePosition` and `RestorePosition` does not work because the saved position is not
    /// intended one after rendering a new line.
    fn initialize_prompt<W>(&mut self, w: &mut W) -> crossterm::Result<()>
    where
        W: Write,
    {
        for _ in 0..self.num_whole_lines {
            w.execute(Print("\r\n"))?;
        }
        w.execute(MoveToPreviousLine(self.num_whole_lines as u16))?;
        Ok(())
    }

    fn start_event_loop<W>(&mut self, w: &mut W) -> crossterm::Result<()>
    where
        W: Write,
    {
        loop {
            match self.handle_key_event(w)? {
                Action::Confirm => {
                    break;
                }
                Action::Quit => {
                    self.input = Vec::new();
                    break;
                }
                Action::Update => {
                    self.render_input(w)?;

                    self.render_candidates(w)?;

                    w.execute(MoveToPreviousLine((self.num_whole_lines - 1) as u16))?
                        .execute(MoveRight(self.input_line_len()))?;
                }
                Action::None => {}
            }
        }

        Ok(())
    }

    fn input_line_len(&self) -> u16 {
        (Self::PROMPT_SYMBOL.len() + self.input.len()) as u16
    }

    fn handle_key_event<W>(&mut self, w: &mut W) -> crossterm::Result<Action>
    where
        W: Write,
    {
        if let Some(event) = self.environment.feed()? {
            let KeyEvent { code, modifiers } = event;
            let action = match code {
                KeyCode::Enter => Action::Confirm,
                KeyCode::Esc => Action::Quit,
                KeyCode::Char('c') if modifiers == KeyModifiers::CONTROL => Action::Quit,
                KeyCode::Backspace => {
                    w.execute(MoveLeft(1))?;
                    self.input.pop();
                    Action::Update
                }
                KeyCode::Up => self.move_up_cursor(),
                KeyCode::Char('k') if modifiers == KeyModifiers::CONTROL => self.move_up_cursor(),
                KeyCode::Down => self.move_down_cursor(),
                KeyCode::Char('j') if modifiers == KeyModifiers::CONTROL => self.move_down_cursor(),
                KeyCode::Char(c) => {
                    self.input.push(c);
                    Action::Update
                }
                _ => Action::None,
            };
            return Ok(action);
        }

        Ok(Action::None)
    }

    fn move_up_cursor(&mut self) -> Action {
        if self.current_font > 0 {
            self.current_font -= 1;
            Action::Update
        } else {
            Action::None
        }
    }

    fn move_down_cursor(&mut self) -> Action {
        if self.current_font + 1 < self.fonts.len() {
            self.current_font += 1;
            Action::Update
        } else {
            Action::None
        }
    }

    fn render_input<W>(&mut self, w: &mut W) -> crossterm::Result<()>
    where
        W: Write,
    {
        w.execute(MoveLeft(self.input_line_len()))?
            .execute(Clear(ClearType::CurrentLine))?
            .execute(Print(format!(
                "{}{}",
                Self::PROMPT_SYMBOL.blue(),
                self.input.iter().collect::<String>()
            )))?;
        Ok(())
    }

    fn render_candidates<W>(&mut self, w: &mut W) -> crossterm::Result<()>
    where
        W: Write,
    {
        for i in 0..self.fonts.len() {
            let selection = if i == self.current_font { "> " } else { "  " };

            w.queue(MoveToNextLine(1))?
                .queue(Clear(ClearType::CurrentLine))?
                .queue(Print(format!(
                    "{}{}",
                    selection.red(),
                    self.converter.convert(&self.input, self.fonts[i])
                )))?;
        }
        w.flush()?;

        Ok(())
    }
}
