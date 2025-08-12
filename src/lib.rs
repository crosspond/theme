mod everforest;

use ratatui::style::Color;

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
}
