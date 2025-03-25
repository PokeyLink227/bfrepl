use crate::{interpreter::BFInt, popup::*, theme::THEME, widgets::TextEntry};
use crossterm::event::{self, KeyCode};
use ratatui::{
    layout::Offset,
    prelude::*,
    widgets::{Block, BorderType, Paragraph, Widget},
};
use std::{
    fmt,
    io::{self},
};

mod interpreter;
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
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Mode {
    Normal,
    Editing,
    Command,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ReplMode {
    Running,
    Paused,
}

enum Dialogue {
    None,
    Save,
    NewTask,
}

#[derive(Clone, Copy, Debug)]
enum ReplType {
    Code,
    Output,
    Input,
}

impl fmt::Display for ReplType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Code => "   >",
                Self::Output => "out>",
                Self::Input => "in >",
            }
        )
    }
}

impl ReplType {
    fn as_str(self) -> &'static str {
        match self {
            Self::Code => "   >",
            Self::Output => "out>",
            Self::Input => "in >",
        }
    }
}

struct Options {
    error_display_time: u32,
    refresh_rate: u32,
}

pub struct App {
    mode: Mode,
    running_mode: RunningMode,
    repl_mode: ReplMode,
    options: Options,
    lines: Vec<ReplType>,
    interp: BFInt,

    command_field: TextEntry,
    error_str: String,
    frames_since_error: Option<u32>,
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(3),
            Constraint::Length(1),
        ]);
        let [title_bar_area, canvas_area, program_area, bottom_bar_area] = vertical.areas(area);
        let horizontal =
            Layout::horizontal([Constraint::Percentage(70), Constraint::Percentage(30)]);
        let [repl_area, mem_info_area] = horizontal.areas(canvas_area);
        let vertical = Layout::vertical([Constraint::Percentage(75), Constraint::Percentage(25)]);
        let [mem_area, info_area] = vertical.areas(mem_info_area);

        //Block::new().style(THEME.root).render(area, buf);

        self.render_title_bar(title_bar_area, buf);

        Paragraph::new(
            self.lines
                .iter()
                .map(|l| Line::from(l.as_str()))
                .collect::<Vec<Line>>(),
        )
        .block(
            Block::bordered()
                .border_style(THEME.root)
                .title("REPL")
                .title_style(THEME.root)
                .style(THEME.root)
                .border_type(BorderType::Rounded),
        )
        .render(repl_area, buf);

        // change to be slice of current program sized ot fit
        Paragraph::new(unsafe { String::from_utf8_unchecked(self.interp.prog.clone()) })
            .block(
                Block::bordered()
                    .border_style(THEME.root)
                    .title("Program view")
                    .title_style(THEME.root)
                    .style(THEME.root)
                    .border_type(BorderType::Rounded),
            )
            .render(program_area, buf);
        Span::from("^").render(
            program_area.offset(Offset {
                x: self.interp.prog_ptr as i32 + 1,
                y: 2,
            }),
            buf,
        );

        Paragraph::new(format!("{:?}", self.interp.mem))
            .block(
                Block::bordered()
                    .border_style(THEME.root)
                    .title("Memory")
                    .title_style(THEME.root)
                    .style(THEME.root)
                    .border_type(BorderType::Rounded),
            )
            .render(mem_area, buf);

        Paragraph::new("memory usage: 17 bytes (2 pages)")
            .block(
                Block::bordered()
                    .border_style(THEME.root)
                    .title("Info")
                    .title_style(THEME.root)
                    .style(THEME.root)
                    .border_type(BorderType::Rounded),
            )
            .render(info_area, buf);

        if self.mode == Mode::Command {
            Line::from(vec![
                Span::from(":"),
                Span::from(self.command_field.get_str()),
            ])
            .render(bottom_bar_area, buf);
            Span::from("█").render(
                bottom_bar_area.offset(Offset {
                    x: 1 + self.command_field.get_cursor_pos() as i32,
                    y: 0,
                }),
                buf,
            );
        } else if let Some(_) = self.frames_since_error {
            Span::from(format!("Error: {}", self.error_str))
                .style(THEME.command_error)
                .render(bottom_bar_area, buf);
        } else {
            self.render_bottom_bar(bottom_bar_area, buf);
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        // initialization
        self.command_field.set_text("t load".to_string());
        self.process_command();

        self.interp.mem[0] = 7;
        self.interp.extend_prog(b"[->+<]");

        // main loop
        while self.running_mode != RunningMode::Exiting {
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
                            KeyCode::Char('n') => self.interp.step(),
                            KeyCode::Char(':') => {
                                self.mode = Mode::Command;
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
        if self.mode == Mode::Command {
            match key {
                KeyCode::Char(c) => self.command_field.insert(c),
                KeyCode::Backspace => self.command_field.remove(),
                KeyCode::Enter => {
                    self.mode = Mode::Normal;
                    self.process_command();
                    self.command_field.move_cursor_home();
                }
                KeyCode::Esc => {
                    self.mode = Mode::Normal;
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
        self.running_mode = RunningMode::Exiting;
    }

    fn try_quit(&mut self) {
        self.force_quit();
    }

    fn render_title_bar(&self, area: Rect, buf: &mut Buffer) {
        let horizontal = Layout::horizontal([
            Constraint::Min(0),
            Constraint::Length(9),
            Constraint::Length(9),
        ]);
        let [app_name, editing_mode_area, repl_mode_area] = horizontal.areas(area);

        //Block::new().style(THEME.root).render(area, buf);
        Paragraph::new("BFRepl").render(app_name, buf);
        match self.mode {
            Mode::Normal => Span::from(" Normal ").style(THEME.mode.normal),
            Mode::Editing => Span::from(" Editing ").style(THEME.mode.editing),
            Mode::Command => Span::from(" Command ").style(THEME.mode.command),
        }
        .render(editing_mode_area, buf);
        match self.repl_mode {
            ReplMode::Running => Span::from(" Running ").style(THEME.mode.editing),
            ReplMode::Paused => Span::from(" Paused ").style(THEME.mode.normal),
        }
        .render(repl_mode_area, buf);
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
        mode: Mode::Normal,
        running_mode: RunningMode::Running,
        repl_mode: ReplMode::Paused,
        options: Options {
            error_display_time: 2,
            refresh_rate: 60,
        },
        lines: vec![ReplType::Code, ReplType::Code, ReplType::Output],
        interp: BFInt::new(),
        command_field: TextEntry::default(),
        error_str: String::new(),
        frames_since_error: None,
    };
    app.run(&mut terminal)?;
    tui::restore()
}
