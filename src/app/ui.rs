use crate::app::App;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::canvas::{Canvas, Line, Map, MapResolution, Rectangle},
    widgets::{
        Axis, BarChart, Block, Borders, Cell, Chart, Dataset, Gauge, LineGauge, List, ListItem,
        Paragraph, Row, Sparkline, Table, Tabs, Wrap,
    },
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());
    let ui = Block::default().borders(Borders::ALL).title(app.title);
    f.render_widget(ui, chunks[0]);
}

pub fn draw_login<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [Constraint::Length(3), Constraint::Min(0)]
            .as_ref(),
        )
        .split(f.size());

    let block_username = ListItem::new("username");
    let block_password = ListItem::new("password");
    let list = List::new(vec![
        block_username,
        block_password
    ]).block(Block::default().title("Login").borders(Borders::ALL))
    .highlight_style(
        Style::default()
            .bg(Color::LightGreen)
            .add_modifier(Modifier::BOLD),
    ).highlight_symbol(">> ");




    f.render_widget(list, chunks[0]);

}
