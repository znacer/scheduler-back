use super::models::Color;

#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Palette {
    Blue,
    Orange,
    Green,
    Red,
    Purple,
    Brown,
    Pink,
    Grey,
    Turquoise,
    Yellow,
}

impl Palette {
    pub fn rgb(&self) -> Color {
        let c = match self {
            Palette::Blue => (31, 119, 180),
            Palette::Orange => (255, 127, 14),
            Palette::Green => (44, 160, 44),
            Palette::Red => (214, 39, 40),
            Palette::Purple => (148, 103, 189),
            Palette::Brown => (140, 86, 75),
            Palette::Pink => (227, 119, 194),
            Palette::Grey => (127, 127, 127),
            Palette::Turquoise => (23, 190, 207),
            Palette::Yellow => (255, 187, 33),
        };
        Color::new(c.0, c.1, c.2)
    }
}
