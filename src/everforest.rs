use super::Theme;

use ratatui::style::Color;

impl Theme {
    pub fn everforest() -> Self {
        Self {
            bg_dim: Color::Rgb(35, 47, 53),          // #232f35
            bg0: Color::Rgb(43, 51, 57),             // #2b3339
            bg1: Color::Rgb(50, 58, 64),             // #323a40
            bg2: Color::Rgb(54, 62, 68),             // #363e44
            bg3: Color::Rgb(58, 66, 72),             // #3a4248
            bg4: Color::Rgb(78, 86, 92),             // #4e565c
            bg5: Color::Rgb(84, 92, 98),             // #545c62
            bg_red: Color::Rgb(230, 126, 128),       // #e67e80
            bg_yellow: Color::Rgb(219, 188, 127),    // #dbbc7f
            bg_visual: Color::Rgb(58, 66, 72),       // #3a4248
            bg_green: Color::Rgb(131, 192, 146),     // #83c092
            bg_blue: Color::Rgb(115, 147, 179),      // #7393b3
            fg: Color::Rgb(211, 198, 170),           // #D3C6AA
            red: Color::Rgb(230, 126, 128),          // #e67e80
            orange: Color::Rgb(230, 152, 117),       // #e69875
            yellow: Color::Rgb(219, 188, 127),       // #dbbc7f
            green: Color::Rgb(131, 192, 146),        // #83c092
            cyan: Color::Rgb(149, 209, 201),         // #95d1c9
            blue: Color::Rgb(115, 147, 179),         // #7393b3
            magenta: Color::Rgb(236, 175, 204),      // #ecafcc
            grey0: Color::Rgb(78, 86, 92),           // #4e565c
            grey1: Color::Rgb(84, 92, 98),           // #545c62
            grey2: Color::Rgb(98, 106, 112),         // #626a70
            statusline_1: Color::Rgb(46, 54, 60),    // #2e363c
            statusline_2: Color::Rgb(61, 69, 75),    // #3d454b
            statusline_3: Color::Rgb(131, 192, 146), // #83c092
        }
    }
}
