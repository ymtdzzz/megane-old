use super::app::App;
use tui::{
    backend::CrosstermBackend,
    widgets::{
        Block,
        Borders,
        Tabs,
    },
    layout::{
        Layout,
        Constraint,
    },
    text::Spans,
    style::{Style, Color},
    Frame,
};
use std::io::Stdout;

pub fn draw(f: &mut Frame<CrosstermBackend<Stdout>>, app: &mut App) {
    // layout
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());
    let titles = ["LOGS", "METRICS"].iter().cloned().map(Spans::from).collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(""))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.current_tab_idx);
    f.render_widget(tabs, chunks[0]);

    // draw main area
    if let Some(tab) = app.tabs.get(app.current_tab_idx) {
        tab.draw(f, chunks[1]);
    }
}

