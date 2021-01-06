use std::fmt;
use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    symbols,
    widgets::{Block, Widget},
};

#[derive(Debug, Clone)]
pub struct Input<'a> {
    pub block: Option<Block<'a>>,
    pub val: String,
    pub title: String,
    pub style: Style,
}

impl<'a> Default for Input<'a> {
    fn default() -> Self {
        Self {
            block: None,
            val: "".to_string(),
            title: "".to_string(),
            style: Style::default(),
        }
    }
}

impl<'a> Input<'a> {
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn val(mut self, val: String) -> Self {
        self.val = val;
        self
    }

    pub fn title(mut self, title: String) -> Self {
        self.title = title;
        self
    }
}

impl<'a> Widget for Input<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.style);
        let area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };
        if area.height > 3 {
            return;
        }
        let style = self.style;
        buf.set_string(0, 0, &self.val, style);
    }
}
