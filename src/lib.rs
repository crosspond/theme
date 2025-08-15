mod everforest;

use edtui::EditorStatusLine;
use ratatui::{
    style::{Color, Style, Stylize},
    widgets::{Block, Padding},
};

#[allow(unused)]
pub struct Theme {
    pub bg_dim: Color,
    pub bg0: Color,
    pub bg1: Color,
    pub bg2: Color,
    pub bg3: Color,
    pub bg4: Color,
    pub bg5: Color,
    pub bg_red: Color,
    pub bg_yellow: Color,
    pub bg_visual: Color,
    pub bg_green: Color,
    pub bg_blue: Color,
    pub fg: Color,
    pub red: Color,
    pub orange: Color,
    pub yellow: Color,
    pub green: Color,
    pub cyan: Color,
    pub blue: Color,
    pub magenta: Color,
    pub grey0: Color,
    pub grey1: Color,
    pub grey2: Color,
    pub statusline_1: Color,
    pub statusline_2: Color,
    pub statusline_3: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self::everforest()
    }
}

impl Theme {
    pub fn accents(&self) -> [Color; 7] {
        [
            self.red,
            self.green,
            self.yellow,
            self.blue,
            self.magenta,
            self.cyan,
            self.orange,
        ]
    }

    pub fn editor_theme(&self) -> edtui::EditorTheme<'_> {
        edtui::EditorTheme::default()
            .block(Block::default().padding(Padding::uniform(1)))
            .base(Style::default().bg(self.bg0).fg(self.fg))
            .cursor_style(Style::default().bg(self.grey2).fg(self.bg_dim))
            .selection_style(Style::default().bg(self.grey1).fg(self.bg_dim))
            .status_line(
                EditorStatusLine::default()
                    .style_text(Style::default().bg(self.cyan).fg(self.bg_dim))
                    .style_line(Style::default().bg(self.bg2).fg(self.fg))
                    .align_left(true),
            )
    }
}
