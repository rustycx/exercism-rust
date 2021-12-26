use int_enum::IntEnum;
use enum_iterator::IntoEnumIterator;

#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum, IntoEnumIterator)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

impl From<ResistorColor> for String {
    fn from(color: ResistorColor) -> Self {
        format!("{:?}", color)
    }
}

pub fn color_to_value(color: ResistorColor) -> usize {
    color as usize
}

pub fn value_to_color_string(value: usize) -> String {
    match ResistorColor::from_int(value) {
        Err(_) => String::from("value out of range"),
        Ok(v) => v.into(),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    ResistorColor::into_enum_iter().collect()
}
