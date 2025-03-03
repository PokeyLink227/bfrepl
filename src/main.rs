use crate::{popup::*, theme::THEME, widgets::TextEntry};
use crossterm::event::{self, KeyCode};
use ratatui::{
    layout::Offset,
    prelude::*,
    widgets::{Block, BorderType, Paragraph, Widget},
};
use std::io::{self};

mod popup;
mod theme;
mod tui;
mod widgets;

pub enum CommandRequest {
    None,
    SetActive,
}

#[derive(Clone, Copy, PartialEq)]
enum RunningMode {
    Running,
    Exiting,
    Command,
}

enum Mode {
    Normal,
    Editing,
    Command,
}

enum Dialogue {
    None,
    Save,
    NewTask,
}

struct Options {
    error_display_time: u32,
    refresh_rate: u32,
}

pub struct App {
    mode: RunningMode,
    options: Options,

    command_field: TextEntry,
    error_str: String,
    frames_since_error: Option<u32>,
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ]);
        let [title_bar, canvas, bottom_bar] = vertical.areas(area);

        //Block::new().style(THEME.root).render(area, buf);

        self.render_title_bar(title_bar, buf);

        Block::bordered()
            .border_style(THEME.root)
            .title("REPL")
            .title_style(THEME.root)
            .style(THEME.root)
            .border_type(BorderType::Rounded)
            .render(canvas, buf);

        if self.mode == RunningMode::Command {
            Line::from(vec![
                Span::from(":"),
                Span::from(self.command_field.get_str()),
            ])
            .render(bottom_bar, buf);
            Span::from("█").render(
                bottom_bar.offset(Offset {
                    x: 1 + self.command_field.get_cursor_pos() as i32,
                    y: 0,
                }),
                buf,
            );
        } else if let Some(_) = self.frames_since_error {
            Span::from(format!("Error: {}", self.error_str))
                .style(THEME.command_error)
                .render(bottom_bar, buf);
        } else {
            self.render_bottom_bar(bottom_bar, buf);
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        // initialization
        self.command_field.set_text("t load".to_string());
        self.process_command();

        // main loop
        while self.mode != RunningMode::Exiting {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;

            // command error timer update
            if let Some(frames) = self.frames_since_error {
                if frames >= self.options.error_display_time * self.options.refresh_rate {
                    self.frames_since_error = None;
                } else {
                    self.frames_since_error = Some(frames + 1);
                }
            }

            // popup handler
        }

        // clean up

        // report no errors
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                // key holds info about modifiers (shitf, ctrl, alt)
                if key.kind == event::KeyEventKind::Press {
                    if !self.dispatch_input(key.code) {
                        match key.code {
                            KeyCode::Char('q') => self.try_quit(),
                            KeyCode::Char(':') => {
                                self.mode = RunningMode::Command;
                                self.frames_since_error = None;
                                self.command_field.clear();
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn dispatch_input(&mut self, key: KeyCode) -> bool {
        if self.mode == RunningMode::Command {
            match key {
                KeyCode::Char(c) => self.command_field.insert(c),
                KeyCode::Backspace => self.command_field.remove(),
                KeyCode::Enter => {
                    self.mode = RunningMode::Running;
                    self.process_command();
                    self.command_field.move_cursor_home();
                }
                KeyCode::Esc => {
                    self.mode = RunningMode::Running;
                    self.command_field.move_cursor_home();
                }
                KeyCode::Left => self.command_field.move_cursor_left(),
                KeyCode::Right => self.command_field.move_cursor_right(),
                _ => {}
            }
            true
        } else {
            false
        }
    }

    // currently doesnt support arguments with spaces included
    fn process_command(&mut self) {
        let mut parsed_command = self.command_field.get_str().split(' ');
        match parsed_command.next().unwrap() {
            "quit" | "q" => self.try_quit(),
            "quit!" | "q!" => self.force_quit(),
            _ => self.post_error(format!("Unknown Command: {}", self.command_field.get_str())),
        }
    }

    fn post_error(&mut self, err_str: String) {
        self.frames_since_error = Some(0);
        self.error_str = err_str;
    }

    fn force_quit(&mut self) {
        self.mode = RunningMode::Exiting;
    }

    fn try_quit(&mut self) {
        self.force_quit();
    }

    fn render_title_bar(&self, area: Rect, buf: &mut Buffer) {
        let horizontal = Layout::horizontal([
            Constraint::Min(0),
            Constraint::Length(7),
            Constraint::Length(10),
            Constraint::Length(9),
            Constraint::Length(9),
        ]);
        let [app_name, list_tab, calendar_tab, options_tab, profile_tab] = horizontal.areas(area);

        Block::new().style(THEME.root).render(area, buf);
        Paragraph::new("FrogPad").render(app_name, buf);
        Paragraph::new(" Tasks ").render(list_tab, buf);
        Paragraph::new(" Calendar ").render(calendar_tab, buf);
        Paragraph::new(" Options ").render(options_tab, buf);
        Paragraph::new(" Profile ").render(profile_tab, buf);
    }

    /*
        need to only render useable controls for currently selected tab.
        so render common followed by specific controls.
    */
    fn render_bottom_bar(&self, area: Rect, buf: &mut Buffer) {
        let common_keys: [(&'static str, &'static str); 2] = [("Q", "Quit"), ("n", "Next Tab")];

        let spans: Vec<Span> = common_keys
            .iter()
            .flat_map(|(key, desc)| {
                let key = Span::from(format!(" {key} ")).style(THEME.key_bind);
                let desc = Span::from(format!(" {desc} ")).style(THEME.key_desc);
                [key, desc]
            })
            .collect();

        Line::from(spans).centered().render(area, buf);
    }
}

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let mut app = App {
        mode: RunningMode::Running,
        options: Options {
            error_display_time: 2,
            refresh_rate: 60,
        },
        command_field: TextEntry::default(),
        error_str: String::new(),
        frames_since_error: None,
    };
    app.run(&mut terminal)?;
    tui::restore()
}
