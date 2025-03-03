#[derive(Default)]
pub struct TextEntry {
    text: String,
    cursor_pos: usize,
}

impl TextEntry {
    pub fn new() -> Self {
        TextEntry {
            text: String::new(),
            cursor_pos: 0,
        }
    }

    pub fn take(&mut self) -> String {
        std::mem::take(&mut self.text)
    }

    pub fn set_text(&mut self, new_text: String) {
        self.text = new_text;
    }

    pub fn clear(&mut self) {
        self.text.clear();
        self.move_cursor_home();
    }

    pub fn get_str(&self) -> &str {
        self.text.as_str()
    }

    pub fn get_cursor_pos(&self) -> usize {
        self.cursor_pos
    }

    fn byte_index(&self) -> usize {
        self.text
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.cursor_pos)
            .unwrap_or(self.text.len())
    }

    pub fn move_cursor_home(&mut self) {
        self.cursor_pos = 0;
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
        }
    }

    pub fn move_cursor_end(&mut self) {
        self.cursor_pos = self.text.len();
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_pos < self.text.len() {
            self.cursor_pos += 1;
        }
    }

    pub fn insert(&mut self, c: char) {
        self.text.insert(self.byte_index(), c);
        self.move_cursor_right();
    }

    pub fn remove(&mut self) {
        if self.text.len() == 0 {
            return;
        }
        // stops backspace from acting like del when at the beginning of the string
        if self.cursor_pos == 0 {
            return;
        }

        self.move_cursor_left();
        self.text.remove(self.byte_index());
    }
}
