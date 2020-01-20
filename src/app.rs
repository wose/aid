use std::io;
use std::sync::mpsc::{RecvTimeoutError};

use termion::input::MouseTerminal;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;

use tui::backend::*;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Widget};
use tui::Terminal;

use crate::event::{Event, Events};
use crate::widgets::Clock;

type TermBackend = TermionBackend<AlternateScreen<MouseTerminal<RawTerminal<io::Stdout>>>>;

pub struct App {
    events: Events,
    shutdown: bool,
    terminal: Terminal<TermBackend>,
}

impl App {
    pub fn new() -> Result<Self, failure::Error> {
        let stdout = io::stdout().into_raw_mode()?;
        let stdout = MouseTerminal::from(stdout);
        let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.hide_cursor()?;

        let events = Events::new();

        Ok(App {
            events,
            shutdown: false,
            terminal,
        })
    }

    pub fn run(&mut self) -> Result<(), failure::Error> {
        use std::time::{Duration, Instant};

        self.draw()?;

        while let Ok(event) = self.events.rx.recv() {
            self.handle_event(event);
            let start = Instant::now();
            while let Some(remaining_time) = Duration::from_millis(16).checked_sub(start.elapsed())
            {
                let event = match self.events.rx.recv_timeout(remaining_time) {
                    Ok(event) => event,
                    Err(RecvTimeoutError::Timeout) => break,
                    Err(_) => {
                        self.shutdown = true;
                        break;
                    }
                };

                self.handle_event(event);
            }

            self.draw()?;

            if self.shutdown {
                break;
            }
        }

        Ok(())
    }

    fn draw(&mut self) -> Result<(), failure::Error> {
        self.terminal.draw(|mut f| {
            let size = f.size();
            let rect = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(10), Constraint::Min(0)].as_ref())
                .split(size);

            Clock::default()
                .block(
                    Block::default()
                        .borders(Borders::BOTTOM)
                        .border_style(Style::default().fg(Color::DarkGray)),
                )
                .center(true)
                .style(Style::default().fg(Color::Green))
                .render(&mut f, rect[0]);
        })?;

        Ok(())
    }

    fn handle_event(&mut self, event: Event<termion::event::Event>) {
        match event {
            Event::Input(input_event) => self.handle_input(input_event),
            Event::Tick => self.handle_tick(),
        };
    }

    fn handle_input(&mut self, input: ::termion::event::Event) {
        use termion::event::Event::*;
        use termion::event::Key::*;

        match input {
            Key(Char('q')) => self.shutdown = true,
            _ => {}
        }
    }

    fn handle_tick(&self) {}
}
