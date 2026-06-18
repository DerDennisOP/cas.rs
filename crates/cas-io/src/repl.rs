use std::io::{self, Write};

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::layout::Position;
use ratatui::style::Stylize;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Widget};
use ratatui::{DefaultTerminal, Frame, TerminalOptions, Viewport};

use crate::parser::{Echo, Parser};

const PROMPT: &str = "> ";
const VIEWPORT_HEIGHT: u16 = 2;

enum Outcome {
    Value(String),
    Error(String),
}

pub struct Repl<P: Parser> {
    parser: P,
    input: String,
}

impl Default for Repl<Echo> {
    fn default() -> Self {
        Self::new(Echo)
    }
}

impl<P: Parser> Repl<P> {
    pub fn new(parser: P) -> Self {
        Self {
            parser,
            input: String::new(),
        }
    }

    pub fn run(mut self) -> io::Result<()> {
        let mut terminal = ratatui::init_with_options(TerminalOptions {
            viewport: Viewport::Inline(VIEWPORT_HEIGHT),
        });
        let outcome = self.event_loop(&mut terminal);
        ratatui::restore();
        outcome?;
        let mut stdout = io::stdout();
        for _ in 0..VIEWPORT_HEIGHT {
            writeln!(stdout)?;
        }
        Ok(())
    }

    fn event_loop(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;
            let Event::Key(key) = event::read()? else {
                continue;
            };
            if key.kind != KeyEventKind::Press {
                continue;
            }
            match key.code {
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => break,
                KeyCode::Esc => break,
                KeyCode::Enter => self.submit(terminal)?,
                KeyCode::Backspace => {
                    self.input.pop();
                }
                KeyCode::Char(c) => self.input.push(c),
                _ => {}
            }
        }
        Ok(())
    }

    fn submit(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let input = std::mem::take(&mut self.input);
        if input.is_empty() {
            return Ok(());
        }
        let outcome = match self.parser.commit(&input) {
            Ok(value) => Outcome::Value(value.to_string()),
            Err(error) => Outcome::Error(error.to_string()),
        };
        let lines = vec![
            Line::from(vec![Span::raw(PROMPT).dim(), Span::raw(input)]),
            outcome.into_line(),
        ];
        let height = lines.len() as u16;
        terminal.insert_before(height, move |buf| {
            let area = buf.area;
            Paragraph::new(lines).render(area, buf);
        })
    }

    fn draw(&self, frame: &mut Frame) {
        let area = frame.area();
        let prompt = Line::from(vec![
            Span::raw(PROMPT).cyan(),
            Span::raw(self.input.as_str()),
        ]);
        frame.render_widget(Paragraph::new(vec![prompt, self.preview()]), area);

        let cursor_x = area.x + (PROMPT.len() + self.input.chars().count()) as u16;
        frame.set_cursor_position(Position::new(cursor_x, area.y));
    }

    fn preview(&self) -> Line<'static> {
        if self.input.is_empty() {
            return Line::default();
        }
        match self.parser.parse(&self.input) {
            Ok(value) => Line::raw(value.to_string()).dim(),
            Err(error) => Line::raw(error.to_string()).red().dim(),
        }
    }
}

impl Outcome {
    fn into_line(self) -> Line<'static> {
        match self {
            Outcome::Value(value) => Line::raw(value),
            Outcome::Error(error) => Line::raw(error).red(),
        }
    }
}
