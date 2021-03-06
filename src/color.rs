
// color enum, represent stones
#[derive(Debug, PartialEq, Clone, Copy, PartialOrd)]
pub enum Color {
    Invis,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple
}

impl Color {
    pub fn from_str(col: &str) -> Color {
        match col {
            "red" => Color::Red,
            "orange" => Color::Orange,
            "yellow" => Color::Yellow,
            "green" => Color::Green,
            "blue" => Color::Blue,
            "purple" => Color::Purple,
            _ => Color::Invis
        }
    }
}

