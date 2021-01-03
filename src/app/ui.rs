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


pub fn draw_main_page<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(90)
            ].as_ref(),
        )
        .split(f.size());
    draw_header(f, app, chunks[0]);
    draw_main_content(f, app, chunks[1]);
}

pub fn draw_header<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(30),
                Constraint::Percentage(35),
                Constraint::Percentage(35)
            ].as_ref()
        ).split(area);


    f.render_widget(Paragraph::new(Span::from("🎵网易云音乐")), chunks[0]);

    f.render_widget(Paragraph::new(Span::from("🔍 摁S开始搜索")), chunks[1]);

    let text = vec![
        Spans::from(
            vec![
                Span::from("👦 "),
                Span::from(app.userinfo.as_ref().map(|a| a.profile.nickname.as_str()).unwrap_or("未登录")),
            ]
        )
    ];

    let username = Paragraph::new(text);
    f.render_widget(username, chunks[2]);

    let block = Block::default().borders(Borders::BOTTOM);
    f.render_widget(block, area);
}

pub fn draw_main_content<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(30),
                Constraint::Percentage(70),
            ].as_ref()
        ).split(area);
    draw_playlists(f, app, chunks[0]);
    draw_tracks(f, app, chunks[1]);
}

pub fn draw_playlists<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let items: Vec<ListItem> = app.playlists_state.items.iter().map(|i| {
        let mut lines = vec![Spans::from(i.name.clone())];
        ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
    })
    .collect();
    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("创建的歌单"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");
    f.render_stateful_widget(items, area, &mut app.playlists_state.state);
}

pub fn draw_tracks<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let items: Vec<ListItem> = app.current_playlist_track_state.items.iter().map(|i| {
        let mut lines = vec![Spans::from(i.name.clone())];
        ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
    })
    .collect();
    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("歌曲列表"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");
    f.render_stateful_widget(items, area, &mut app.current_playlist_track_state.state);
}