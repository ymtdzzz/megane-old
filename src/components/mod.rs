use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    Frame,
};
use crossterm::event::KeyEvent;
use std::io::Stdout;
use async_trait::async_trait;

pub mod tab;
pub mod textinput;
pub mod logs;
pub mod spinner;

#[async_trait]
pub trait Drawable {
    /// all tabs must be drawable
    fn draw(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>, area: Rect);

    /// handle event
    /// return data means whether parent component should handle event or not
    async fn handle_event(&mut self, event: KeyEvent) -> bool;
}


