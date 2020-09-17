use tui::{
    backend::Backend,
    layout::Rect,
    Frame,
};

pub mod logstab;

pub trait Tab {
    /// all tabs must be drawable
    fn draw<B: Backend>(&self, f: &mut Frame<B>, area: Rect);
}

