use crossterm::event::KeyCode;
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

#[derive(Debug, Clone)]
pub struct Input {
    pub block: bool,
    pub val: String,
    pub title: String,
    pub style: Style,
    pub focus: bool,
    pub is_password: bool,
    pub placeholder: String,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            block: true,
            val: "".to_string(),
            placeholder: "".to_string(),
            title: "".to_string(),
            style: Style::default(),
            focus: false,
            is_password: false,
        }
    }
}

impl Input {
    pub fn on_key(&mut self, key: KeyCode) {
        if self.focus {
            match key {
                KeyCode::Enter => {
                    self.focus = false;
                },
                KeyCode::Backspace => {
                    self.val.pop();
                },
                KeyCode::Char(c) => {
                    self.val.push(c);
                }
                _ => {},
            }
        }
    }

    pub fn block(mut self, block: bool) -> Self {
        self.block = block;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn placeholder(mut self, placeholder: String) -> Self {
        self.placeholder = placeholder;
        self
    }

    pub fn title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn val(mut self, val: String) -> Self {
        self.val = val;
        self
    }

    pub fn is_password(mut self, is_password: bool) -> Self {
        self.is_password = is_password;
        self
    }

    pub fn draw<B: Backend>(&self, f: &mut Frame<B>, area: Rect, system_tick: &u64) {
        let val_len = self.val.len();
        let show_text =
            if val_len == 0 {
                self.placeholder.clone()
            } else {
                if self.is_password {
                    "*".repeat(val_len)
                } else {
                    self.val.clone()
                }
            };
        let text = Paragraph::new(if self.focus {
            Spans::from(vec![
                Span::from(show_text),
                Span::styled(
                    " ",
                    Style::default().bg(if system_tick % 2 == 0 {
                        Color::Black
                    } else {
                        Color::White
                    }),
                ),
            ])
        } else {
            Spans::from(show_text)
        });
        if self.block {
            f.render_widget(
                text.block(Block::default().title(self.title.as_ref()).borders(Borders::ALL)),
                area,
            );
        } else {
            f.render_widget(text, area);
        }
    }
}
