use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    Frame,
};
use std::io::Stdout;

pub mod logstab;

pub trait Tab {
    /// all tabs must be drawable
    // fn draw<B: Backend>(&self, f: &mut Frame<B>, area: Rect);
    fn draw(&self, f: &mut Frame<CrosstermBackend<Stdout>>, area: Rect);
}

