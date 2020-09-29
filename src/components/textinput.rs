use crate::components::Drawable;
use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    Frame,
    widgets::{Paragraph, Block, Borders},
    style::{Style, Color},
};
use crossterm::event::{KeyEvent, KeyCode};
use std::io::Stdout;
use async_trait::async_trait;

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

    pub fn select(&mut self) {
        self.is_active = true;
    }

    pub fn deselect(&mut self) {
        self.is_active = false;
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

    pub fn set_title(&mut self, title: String) {
        self.title = title;
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
}

#[async_trait]
impl Drawable for TextInputComponent {
    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
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
    }

    async fn handle_event(&mut self, event: KeyEvent) -> bool {
        let mut solved = true;
        if self.is_normal_mode() {
            match event.code {
                KeyCode::Enter => self.set_input_mode(EditMode),
                _ => solved = false
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
                _ => solved = false
            }
        }
        solved
    }
}
