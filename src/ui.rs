use super::app::App;
use tui::{
    backend::Backend,
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

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
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

// struct Tab<'a, B: Backend> {
//     f: &'a mut Frame<'a, B>,
//     area: Rect,
// }

trait Tab {
    fn draw<B: Backend>(&self, f: &mut Frame<B>, area: Rect);
}

struct LogsTab {}

impl LogsTab {
    pub fn new() -> LogsTab {
        LogsTab {}
    }
}

impl Tab for LogsTab {
    fn draw<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let chunks = Layout::default()
            .constraints([
                Constraint::Percentage(100)
            ].as_ref())
            .split(area);
        let block = Block::default().borders(Borders::ALL).title("Log Tab Area");
        f.render_widget(block, chunks[0]);
    }
}
