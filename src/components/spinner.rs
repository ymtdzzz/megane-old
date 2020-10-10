use super::Drawable;
use async_trait::async_trait;
use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    Frame,
    widgets::{Paragraph, Block, Borders},
};
use crossterm::event::{KeyEvent, KeyCode, KeyModifiers};
use std::io::Stdout;

pub struct Spinner {
    description: String,
    spinner: Vec<char>,
    current_pos: usize,
}

impl Spinner {
    pub fn new(description: &str, spinner: Option<Vec<char>>) -> Self {
        let spinner = if let Some(s) = spinner {
            s
        } else {
            vec![
                '⣾',
                '⣽',
                '⣻',
                '⢿',
                '⡿',
                '⣟',
                '⣯',
                '⣷',
            ]
        };
        Self {
            description: description.to_string(),
            spinner,
            current_pos: 0,
        }
    }

    fn next_pos(&mut self) {
        if self.current_pos == (self.spinner.len() - 1) {
            self.current_pos = 0;
        } else {
            self.current_pos = self.current_pos.saturating_add(1);
        }
    }

    fn get_spinner_char(&mut self) -> char {
        let spinner_char = self.spinner.get_mut(self.current_pos).unwrap().clone();
        self.next_pos();
        spinner_char
    }

    pub fn get_message(&mut self) -> String {
        let spinner = self.get_spinner_char();
        format!("{}{}", self.description, spinner)
    }

    pub fn reset(&mut self) {
        self.current_pos = 0;
    }
}

#[async_trait]
impl Drawable for Spinner {
    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
        let msg = self.get_message();
        let paragraph = Paragraph::new(msg.as_ref())
            .block(
                Block::default()
                    .borders(Borders::NONE)
            );
        f.render_widget(paragraph, area);
    }

    async fn handle_event(&mut self, event: KeyEvent) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_default_spinner() -> Spinner {
        Spinner::new("Now Loading", None)
    }
    
    fn get_custom_spinner() -> Spinner {
        // current_pos is from 0 to 4
        Spinner::new("Now Loading", Some(vec!['.', 'o', 'O', '@', '*']))
    }

    #[test]
    fn it_can_increase_current_pos() {
        let mut spinner = get_custom_spinner();
        assert_eq!(spinner.current_pos, 0);
        spinner.next_pos(); // pos 0 to 1
        assert_eq!(spinner.current_pos, 1);
        spinner.next_pos(); // pos 1 to 2
        spinner.next_pos(); // pos 2 to 3
        spinner.next_pos(); // pos 3 to 4
        assert_eq!(spinner.current_pos, 4);
        spinner.next_pos(); // pos 4 to 0
        assert_eq!(spinner.current_pos, 0);
    }

    #[test]
    fn it_can_get_spinner_char() {
        let mut spinner = get_custom_spinner();
        let actual = spinner.get_spinner_char();
        let expected = '.';
        assert_eq!(actual, expected);
        let actual = spinner.get_spinner_char();
        let expected = 'o';
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_can_get_message() {
        let mut default_spinner = get_default_spinner();
        let actual = default_spinner.get_message();
        let expected = String::from("Now Loading⣾");
        assert_eq!(actual, expected);
        let actual = default_spinner.get_message();
        let expected = String::from("Now Loading⣽");
        assert_eq!(actual, expected);
        let actual = default_spinner.get_message();
        let expected = String::from("Now Loading⣻");
        assert_eq!(actual, expected);
        let actual = default_spinner.get_message();
        let expected = String::from("Now Loading⢿");
        assert_eq!(actual, expected);
        let actual = default_spinner.get_message();
        let expected = String::from("Now Loading⡿");
        assert_eq!(actual, expected);
        let actual = default_spinner.get_message();
        let expected = String::from("Now Loading⣟");
        assert_eq!(actual, expected);
        let actual = default_spinner.get_message();
        let expected = String::from("Now Loading⣯");
        assert_eq!(actual, expected);
        let actual = default_spinner.get_message();
        let expected = String::from("Now Loading⣷");
        assert_eq!(actual, expected);
        
        let mut custom_spinner = get_custom_spinner();
        let actual = custom_spinner.get_message();
        let expected = String::from("Now Loading.");
        assert_eq!(actual, expected);
        let actual = custom_spinner.get_message();
        let expected = String::from("Now Loadingo");
        assert_eq!(actual, expected);
        let actual = custom_spinner.get_message();
        let expected = String::from("Now LoadingO");
        assert_eq!(actual, expected);
        
        custom_spinner.reset();
        let actual = custom_spinner.get_message();
        let expected = String::from("Now Loading.");
        assert_eq!(actual, expected);
        let actual = custom_spinner.get_message();
        let expected = String::from("Now Loadingo");
        assert_eq!(actual, expected);
        let actual = custom_spinner.get_message();
        let expected = String::from("Now LoadingO");
        assert_eq!(actual, expected);
        let actual = custom_spinner.get_message();
        let expected = String::from("Now Loading@");
        assert_eq!(actual, expected);
        let actual = custom_spinner.get_message();
        let expected = String::from("Now Loading*");
        assert_eq!(actual, expected);
        let actual = custom_spinner.get_message();
        let expected = String::from("Now Loading.");
        assert_eq!(actual, expected);
    }

    #[tokio::test(basic_scheduler)]
    async fn it_can_handle_event() {
        let mut default_spinner = get_default_spinner();
        let resolved = default_spinner.handle_event(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE)).await;
        assert!(!resolved);
    }
}
