use tui::{
    backend::Backend,
    layout::Rect,
    style::Modifier,
    text::Text,
    Frame,
    widgets::{Paragraph, Block, Borders},
    style::{Style, Color},
};
use crossterm::event::{KeyEvent, KeyCode};
use anyhow::Result;

pub enum InputMode {
    NormalMode,
    EditMode,
}

use InputMode::*;

/// text input component.
/// implementation is referenced to gitui (https://github.com/extrawurst/gitui/blob/master/src/components/textinput.rs)
pub struct TextInputComponent {
    title: String,
    default_msg: String,
    msg: String,
    is_active: bool,
    input_mode: InputMode,
    cursor_position: usize,
}

impl TextInputComponent {
    pub fn new(title: &str, default_msg: &str) -> Self {
        Self {
            msg: String::default(),
            is_active: false,
            title: title.to_string(),
            default_msg: default_msg.to_string(),
            input_mode: NormalMode,
            cursor_position: 0,
        }
    }

    pub fn get_text(&self) -> &String {
        &self.msg
    }

    pub fn toggle_active(&mut self) {
        self.is_active = !self.is_active;
    }

    pub fn set_input_mode(&mut self, mode: InputMode) {
        self.input_mode = mode;
    }

    pub fn is_normal_mode(&self) -> bool {
        match self.input_mode {
            NormalMode => true,
            EditMode => false,
        }
    }

    fn next_cursor(&mut self) {
        if let Some(pos) = self.next_char_position() {
            self.cursor_position = pos;
        }
    }

    fn prev_cursor(&mut self) {
        let mut index = self.cursor_position.saturating_sub(1);
        while index > 0 && !self.msg.is_char_boundary(index) {
            index -= 1;
        }
        self.cursor_position = index;
    }

    fn next_char_position(&self) -> Option<usize> {
        if self.cursor_position >= self.msg.len() {
            return None;
        }
        let mut index = self.cursor_position.saturating_add(1);
        while index < self.msg.len()
            && !self.msg.is_char_boundary(index)
        {
            index += 1;
        }
        Some(index)
    }

    fn delete(&mut self) {
        if self.cursor_position > 0 {
            self.prev_cursor();
            self.msg.remove(self.cursor_position);
        }
    }

    fn insert_cursor<'a>(msg: &'a str, at: usize, ch: &'a str) -> String {
        let (first, last) = msg.split_at(at);
        [first, ch, last].concat()
    }

    // TODO: export these functions to trait 'Component'? ------------
    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) -> Result<()> {
        let mut msg = if self.msg.is_empty() {
            self.default_msg.as_ref()
        } else {
            self.msg.as_ref()
        };
        let msg_with_cursor = Self::insert_cursor(msg, self.cursor_position, "|");
        if !self.is_normal_mode() {
            msg = &msg_with_cursor;
        }
        let paragraph = Paragraph::new(msg)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(
                        if self.is_active {
                            Style::default().fg(Color::Yellow)
                        } else {
                            Style::default().fg(Color::White)
                        }
                    )
                    .title(self.title.as_str())
            );
        f.render_widget(paragraph, area);
        Ok(())
    }

    pub fn handle_event(&mut self, event: KeyEvent) {
        if self.is_normal_mode() {
            match event.code {
                KeyCode::Enter => self.set_input_mode(EditMode),
                _ => {}
            }
        } else {
            match event.code {
                KeyCode::Char(c) => {
                    self.msg.insert(self.cursor_position, c);
                    self.next_cursor();
                },
                KeyCode::Backspace => self.delete(),
                KeyCode::Right => self.next_cursor(),
                KeyCode::Left => self.prev_cursor(),
                KeyCode::Esc => self.set_input_mode(NormalMode),
                _ => {}
            }
        }
    }
}