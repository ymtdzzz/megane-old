use super::app::App;
use super::components::tab::logstab::LogsTab;
use super::components::tab::Tab;
use tui::{
    backend::CrosstermBackend,
    widgets::{
        Block,
        Borders,
        Tabs,
    },
    layout::{
        Layout,
        Direction,
        Constraint,
        Rect,
    },
    text::Spans,
    style::{Style, Color},
    Frame,
};
use std::io::Stdout;

pub fn draw(f: &mut Frame<CrosstermBackend<Stdout>>, app: &mut App) {
    // layout
    // call current tab's draw()
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());
    let titles = ["LOGS", "METRICS"].iter().cloned().map(Spans::from).collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("aaa"))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(0);
    f.render_widget(tabs, chunks[0]);

    let logsTab = LogsTab::new();
    logsTab.draw(f, chunks[1]);
}

