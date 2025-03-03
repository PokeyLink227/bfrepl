use crate::{theme::THEME, widgets::TextEntry};
use crossterm::event::KeyCode;
use ratatui::{
    layout::{Flex, Offset},
    prelude::*,
    widgets::{Block, BorderType, Clear, Paragraph, Wrap},
};

#[derive(Default, PartialEq)]
pub enum PopupStatus {
    InUse,
    Canceled,
    Confirmed,
    #[default]
    Closed,
}

#[derive(Default)]
pub struct TextEntryPopup {
    pub text_field: TextEntry,
    pub title: String,
    pub status: PopupStatus,
    pub max_lines: u16,
}

impl TextEntryPopup {
    pub fn handle_input(&mut self, key: KeyCode) -> bool {
        let mut input_captured = true;

        match key {
            KeyCode::Enter => self.confirm(),
            KeyCode::Esc => self.cancel(),
            KeyCode::Char(c) => self.text_field.insert(c),
            KeyCode::Backspace => self.text_field.remove(),
            KeyCode::Left => self.text_field.move_cursor_left(),
            KeyCode::Right => self.text_field.move_cursor_right(),
            _ => input_captured = false,
        }

        input_captured
    }

    pub fn new(title: String, max_lines: u16) -> Self {
        TextEntryPopup {
            text_field: TextEntry::new(),
            title,
            status: PopupStatus::Closed,
            max_lines,
        }
    }

    fn confirm(&mut self) {
        self.status = PopupStatus::Confirmed;
    }

    fn cancel(&mut self) {
        self.status = PopupStatus::Canceled;
    }

    pub fn close(&mut self) {
        self.status = PopupStatus::Closed;
    }

    pub fn show(&mut self) {
        self.status = PopupStatus::InUse;
    }

    pub fn reset(&mut self) {
        self.close();
        self.text_field.clear();
    }

    pub fn take(&mut self) -> String {
        std::mem::take(&mut self.text_field.take())
    }
}

impl Widget for &TextEntryPopup {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([self.max_lines + 2]).flex(Flex::Center);
        let horizontal = Layout::horizontal([60]).flex(Flex::Center);
        let [area] = vertical.areas(area);
        let [area] = horizontal.areas(area);

        let window = Block::bordered()
            .style(THEME.popup)
            .border_style(THEME.popup)
            .border_type(BorderType::Rounded)
            .title(self.title.as_str())
            .title_bottom(
                Line::raw(format!(" [Esc] to Cancel [Enter] to Confirm "))
                    .alignment(Alignment::Right),
            );

        let win_area = window.inner(area);
        Clear.render(win_area, buf);
        window.render(area, buf);

        Paragraph::new(self.text_field.get_str())
            .wrap(Wrap { trim: true })
            .style(THEME.popup_selected)
            .render(win_area, buf);

        let cursor_pos = self.text_field.get_cursor_pos() as i32;
        Span::from("█").style(THEME.popup_selected).render(
            win_area.offset(Offset {
                x: cursor_pos % 58,
                y: cursor_pos / 58,
            }),
            buf,
        );
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum ConfirmationField {
    Yes,
    #[default]
    No,
}

impl ConfirmationField {
    pub fn cycle_next(&mut self) {
        *self = match self {
            ConfirmationField::No => ConfirmationField::Yes,
            ConfirmationField::Yes => ConfirmationField::No,
        }
    }
}

pub struct ConfirmationPopup {
    pub title: String,
    pub body: String,
    pub status: PopupStatus,

    selected_field: ConfirmationField,
}

impl ConfirmationPopup {
    pub fn handle_input(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Tab => {
                self.selected_field.cycle_next();
                true
            }
            KeyCode::BackTab => {
                self.selected_field.cycle_next();
                true
            }
            KeyCode::Char('y') => {
                self.selected_field = ConfirmationField::Yes;
                self.status = PopupStatus::Confirmed;
                true
            }
            KeyCode::Char('n') => {
                self.selected_field = ConfirmationField::No;
                self.status = PopupStatus::Confirmed;
                true
            }
            KeyCode::Esc => {
                self.selected_field = ConfirmationField::No;
                self.status = PopupStatus::Canceled;
                true
            }
            KeyCode::Enter => {
                self.status = PopupStatus::Confirmed;
                true
            }
            KeyCode::Char('q') => true,
            _ => false,
        }
    }

    pub fn new(new_title: String, new_body: String) -> ConfirmationPopup {
        ConfirmationPopup {
            selected_field: ConfirmationField::No,
            title: new_title,
            body: new_body,
            status: PopupStatus::Closed,
        }
    }

    pub fn show(&mut self) {
        self.selected_field = ConfirmationField::No;
        self.status = PopupStatus::InUse;
    }

    pub fn close(&mut self) {
        self.status = PopupStatus::Closed;
    }

    pub fn decision(&self) -> bool {
        match self.selected_field {
            ConfirmationField::No => false,
            ConfirmationField::Yes => true,
        }
    }
}

impl Widget for &ConfirmationPopup {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([5]).flex(Flex::Center);
        let horizontal = Layout::horizontal([45]).flex(Flex::Center);
        let [area] = vertical.areas(area);
        let [area] = horizontal.areas(area);

        let window = Block::bordered()
            .style(THEME.popup)
            .border_style(THEME.popup)
            .border_type(BorderType::Rounded)
            .title(Span::from(&self.title));
        /*
        .title(
            Title::from(format!(" [Esc] to Cancel [Enter] to Confirm "))
                .alignment(Alignment::Right)
                .position(Position::Bottom),
        );
        */

        let win_area = window.inner(area);
        Clear.render(win_area, buf);
        window.render(area, buf);

        let vertical = Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(1),
            Constraint::Length(1),
        ]);
        let [body_area, _gap, button_area] = vertical.areas(win_area);

        Paragraph::new(self.body.as_str())
            .style(THEME.popup)
            .alignment(Alignment::Center)
            .render(body_area, buf);

        Line::from(vec![
            Span::from("[No]").style(if self.selected_field == ConfirmationField::No {
                THEME.popup_selected
            } else {
                THEME.popup
            }),
            Span::from("               "),
            Span::from("[Yes]").style(if self.selected_field == ConfirmationField::Yes {
                THEME.popup_selected
            } else {
                THEME.popup
            }),
        ])
        .centered()
        .render(button_area, buf);
    }
}
