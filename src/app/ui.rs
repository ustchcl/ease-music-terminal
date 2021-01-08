use crate::app::{input::Input, App, Focus, Route};
use crate::util::utils::{pre_format, show_duration};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect, Margin},
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
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());

    let block_username = ListItem::new("username");
    let block_password = ListItem::new("password");
    let list = List::new(vec![block_username, block_password])
        .block(Block::default().title("Login").borders(Borders::ALL))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_widget(list, chunks[0]);
}

pub fn draw_ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    match app.route {
        Route::Login => draw_login_page(f, app),
        Route::Loading => draw_loading_page(f, app),
        Route::Home => draw_main_page(f, app),
        Route::Search => draw_search_page(f, app),
        Route::MusicAnalysis => draw_music_analysis(f, app),
    }
}

pub fn draw_main_page<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Min(2),
                Constraint::Percentage(80),
                Constraint::Min(2),
                Constraint::Length(1),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(f.size());
    draw_header(f, app, chunks[0]);
    draw_main_content(f, app, chunks[1]);
    draw_control_bar(f, app, chunks[2]);
    draw_percent(f, app, chunks[3]);
    draw_lyric(f, app, chunks[4]);
}

pub fn draw_header<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(30),
                Constraint::Percentage(35),
                Constraint::Percentage(35),
            ]
            .as_ref(),
        )
        .split(area);

    f.render_widget(Paragraph::new(Span::from("🎵 网易云音乐")), chunks[0]);

    f.render_widget(Paragraph::new(Span::from("🔍 搜索")), chunks[1]);

    let text = vec![Spans::from(vec![
        Span::from("👦 "),
        Span::from(
            app.userinfo
                .as_ref()
                .map(|a| a.profile.nickname.as_str())
                .unwrap_or("未登录"),
        ),
    ])];

    let username = Paragraph::new(text);
    f.render_widget(username, chunks[2]);

    let block = Block::default().borders(Borders::BOTTOM);
    f.render_widget(block, area);
}

pub fn draw_main_content<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(area);
    draw_playlists(f, app, chunks[0]);
    draw_tracks(f, app, chunks[1]);
}

/// 音乐播放列表
pub fn draw_playlists<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let len = app.playlists_state.items.len();
    let items: Vec<ListItem> = (0..len)
        .into_iter()
        .map(|i| {
            let lines = vec![Spans::from(format!(
                "{}. ({}首){}",
                pre_format(i + 1, (len + 1).to_string().len(), '0'),
                app.playlists_state.items[i].track_count,
                app.playlists_state.items[i].name.clone()
            ))];
            ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
        })
        .collect();
    let is_focus = app.focus == Focus::Playlist;
    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("创建的歌单"))
        .highlight_style(
            Style::default()
                .bg(if is_focus {
                    Color::LightBlue
                } else {
                    Color::White
                })
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("👉 ");
    f.render_stateful_widget(items, area, &mut app.playlists_state.state);
}

// pub fn draw_playlist_detail<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
//     let p = app.current_playlist();
//     let paragraph = Paragraph::new(
//         vec![
//             Spans::from(
//                 vec![
//                     Span::styled("歌单", Style::default().fg(Color::Cyan).bg(Color::Black)),
//                     Span::from(p.name.clone()),
//                     Span::from(format!("{} 创建", p.create_time))

//                 ]
//             )
//         ]

//     );
// }

/// 绘制播放列表的音乐列表
pub fn draw_tracks<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let len = app.current_playlist_track_state.items.len();
    let items: Vec<ListItem> = (0..len)
        .into_iter()
        .map(|i| {
            let item = &app.current_playlist_track_state.items[i];
            let lines = vec![Spans::from(format!(
                "{}. {} - {} - [{}] - <<{}>>",
                pre_format(i + 1, (len + 1).to_string().len(), '0'),
                item.name.clone(),
                show_duration(item.dt),
                if item.ar.is_empty() {
                    "匿名".to_string()
                } else {
                    item.ar[0].name.clone()
                },
                item.al.name.clone()
            ))];
            ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
        })
        .collect();
    let is_focus = app.focus == Focus::Track;
    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("歌曲列表"))
        .highlight_style(
            Style::default()
                .bg(if is_focus {
                    Color::LightBlue
                } else {
                    Color::White
                })
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("👉 ");
    f.render_stateful_widget(items, area, &mut app.current_playlist_track_state.state);
}

/// 绘制音乐控制器
fn draw_control_bar<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let current_track = app.current_playing_track();
    let current_track_name = current_track
        .as_ref()
        .map(|x| x.name.as_str())
        .unwrap_or("--");
    let loved = current_track
        .as_ref()
        .map(|x| app.is_liked(&x.id))
        .unwrap_or(false);
    let current_track_artist_name = current_track
        .as_ref()
        .map(|t| {
            t.ar.iter()
                .map(|a| a.name.clone())
                .collect::<Vec<_>>()
                .join(",")
        })
        .unwrap_or("--".to_string());
    let is_pause = app.player_controller.is_pause;
    let volume = (app.sink.volume() * 100.0) as u16;

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(area);

    f.render_widget(
        Paragraph::new(vec![
            Spans::from(format!(
                "🎶 {} {}",
                current_track_name,
                if loved { "🧡" } else { "🤍" }
            )),
            Spans::from(format!("🎤 {}", current_track_artist_name)),
        ]),
        chunks[0],
    );
    let pause_play_text = if !is_pause {
        "播放状态: ⏸️"
    } else {
        "播放状态: ▶️"
    };
    f.render_widget(
        Paragraph::new(vec![
            Spans::from(pause_play_text),
            Spans::from("上一首: Ctrl+←  下一首: Ctrl+→"),
        ]),
        chunks[1],
    );

    let chunks_volume = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Percentage(100)].as_ref())
        .split(chunks[2]);
    let volume_icon = if volume > 50 {
        "🔊"
    } else if volume > 0 {
        "🔉"
    } else {
        "🔈"
    };
    let gauge = Gauge::default()
        .gauge_style(
            Style::default()
                .fg(Color::Yellow)
                .bg(Color::Black)
                .add_modifier(Modifier::ITALIC),
        )
        .label(format!("{}: {}%", volume_icon, volume))
        .percent(volume);
    f.render_widget(gauge, chunks_volume[0]);
    f.render_widget(
        Paragraph::new(
            Spans::from("音量+: Ctrl+↑  音量-: Ctrl+↓"),
        ),
        chunks_volume[1],
    );
}

fn draw_percent<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let duration = app
        .current_playing_track()
        .as_ref()
        .map(|x| x.dt)
        .unwrap_or(100000);
    let played = app.player_controller.seek * 1000;
    let percent = (((played as f32) * 100.0) / (duration as f32)) as u16;
    let gauge_play_duration = Gauge::default()
        .gauge_style(
            Style::default()
                .fg(Color::Yellow)
                .bg(Color::Black)
                .add_modifier(Modifier::ITALIC),
        )
        .label(format!(
            "⌛: {}/{}",
            show_duration(played),
            show_duration(duration)
        ))
        .percent(percent.min(100));
    f.render_widget(gauge_play_duration, area);
}

fn draw_lyric<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let lyric_row = app.get_avaiable_lrc_row();
    let text = Gauge::default()
        .gauge_style(
            Style::default()
                .fg(Color::Yellow)
                .bg(Color::Black)
                .add_modifier(Modifier::ITALIC),
        ).label(lyric_row)
        .percent(0);

    f.render_widget(text, area);
}

/// 绘制登录页
fn draw_login_page<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(38),
                Constraint::Min(3),
                Constraint::Min(3),
                Constraint::Percentage(62),
            ]
            .as_ref(),
        )
        .split(f.size());

    let app_title = vec![
        Spans::from("███████╗ █████╗ ███████╗███████╗    ███╗   ███╗██╗   ██╗███████╗██╗ ██████╗"),
        Spans::from("██╔════╝██╔══██╗██╔════╝██╔════╝    ████╗ ████║██║   ██║██╔════╝██║██╔════╝"),
        Spans::from("█████╗  ███████║███████╗█████╗      ██╔████╔██║██║   ██║███████╗██║██║     "),
        Spans::from("██╔══╝  ██╔══██║╚════██║██╔══╝      ██║╚██╔╝██║██║   ██║╚════██║██║██║     "),
        Spans::from("███████╗██║  ██║███████║███████╗    ██║ ╚═╝ ██║╚██████╔╝███████║██║╚██████╗"),
        Spans::from("╚══════╝╚═╝  ╚═╝╚══════╝╚══════╝    ╚═╝     ╚═╝ ╚═════╝ ╚══════╝╚═╝ ╚═════╝"),
        Spans::from("                                                            by ustchcl     "),
    ];

    let login_helper = vec![
        Spans::from("切换和激活输入框\t Ctrl + i"),
        Spans::from("登录 \t\t\t Ctrl + Enter"),
    ];
    f.render_widget(Paragraph::new(app_title), chunks[0]);
    app.inputs[0].draw(f, chunks[1], &app.system_tick);
    app.inputs[1].draw(f, chunks[2], &app.system_tick);
    f.render_widget(Paragraph::new(login_helper), chunks[3]);
}

fn draw_input<B: Backend>(
    f: &mut Frame<B>,
    focus: bool,
    title: &str,
    val: &str,
    area: Rect,
    app: &App,
) {
    let text = Paragraph::new(if focus {
        Spans::from(vec![
            Span::from(val),
            Span::styled(
                " ",
                Style::default().bg(if app.system_tick % 2 == 0 {
                    Color::Black
                } else {
                    Color::White
                }),
            ),
        ])
    } else {
        Spans::from(val)
    })
    .block(Block::default().title(title).borders(Borders::ALL));
    f.render_widget(text, area);
}

/// 绘制搜索页面
fn draw_search_page<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let block = Block::default().title("搜索").borders(Borders::ALL);
    f.render_widget(block, f.size());
}

/// 绘制音乐播放详情
fn draw_music_analysis<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let block = Block::default().title("音乐").borders(Borders::ALL);
    f.render_widget(block, f.size());
}

/// 绘制加载页面
fn draw_loading_page<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let system_tick = &app.system_tick;
    let mut text = String::from("加载中...");
    let c = match system_tick % 4 {
        0 => '|',
        1 => '/',
        2 => '-',
        3 => '\\',
        _ => '|',
    };
    text.push(c);
    let p = Paragraph::new(
        Spans::from(text)
    ).block(Block::default().borders(Borders::ALL));

    f.render_widget(p, f.size().inner(&Margin { vertical: 20, horizontal: 20 }));
    
}