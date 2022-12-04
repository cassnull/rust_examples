#[derive(Debug, PartialEq, Eq)]
pub enum ResistorColor {
    Black,
    Blue,
    Brown,
    Green,
    Grey,
    Orange,
    Red,
    Violet,
    White,
    Yellow,
}

/// Convert a color into a numerical representation.
pub fn color_to_value(color: ResistorColor) -> u32 {
    match color {
        ResistorColor::Black => 0,
        ResistorColor::Blue => 6,
        ResistorColor::Brown => 1,
        ResistorColor::Green => 5,
        ResistorColor::Grey => 8,
        ResistorColor::Orange => 3,
        ResistorColor::Red => 2,
        ResistorColor::Violet => 7,
        ResistorColor::White => 9,
        ResistorColor::Yellow => 4,
    }
}

/// Convert the value into a string representation of color.
pub fn value_to_color_string(value: u32) -> String {
    match value {
        0 => format!("{:?}", ResistorColor::Black),
        1 => format!("{:?}", ResistorColor::Brown),
        2 => format!("{:?}", ResistorColor::Red),
        3 => format!("{:?}", ResistorColor::Orange),
        4 => format!("{:?}", ResistorColor::Yellow),
        5 => format!("{:?}", ResistorColor::Green),
        6 => format!("{:?}", ResistorColor::Blue),
        7 => format!("{:?}", ResistorColor::Violet),
        8 => format!("{:?}", ResistorColor::Grey),
        9 => format!("{:?}", ResistorColor::White),
        _ => String::from("value out of range"),
    }
}

/// Return a list of all the colors ordered by resistance.
pub fn colors() -> Vec<ResistorColor> {
    vec![
        ResistorColor::Black,
        ResistorColor::Brown,
        ResistorColor::Red,
        ResistorColor::Orange,
        ResistorColor::Yellow,
        ResistorColor::Green,
        ResistorColor::Blue,
        ResistorColor::Violet,
        ResistorColor::Grey,
        ResistorColor::White,
    ]
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_black() {
        assert_eq!(color_to_value(ResistorColor::Black), 0);
    }

    #[test]
    fn test_orange() {
        assert_eq!(color_to_value(ResistorColor::Orange), 3);
    }

    #[test]
    fn test_white() {
        assert_eq!(color_to_value(ResistorColor::White), 9);
    }

    #[test]
    fn test_2() {
        assert_eq!(value_to_color_string(2), String::from("Red"));
    }

    #[test]
    fn test_6() {
        assert_eq!(value_to_color_string(6), String::from("Blue"));
    }

    #[test]
    fn test_8() {
        assert_eq!(value_to_color_string(8), String::from("Grey"));
    }

    #[test]
    fn test_11_out_of_range() {
        assert_eq!(
            value_to_color_string(11),
            String::from("value out of range")
        );
    }

    #[test]
    fn test_all_colors() {
        use ResistorColor::*;
        assert_eq!(
            colors(),
            vec![Black, Brown, Red, Orange, Yellow, Green, Blue, Violet, Grey, White]
        );
    }
}
