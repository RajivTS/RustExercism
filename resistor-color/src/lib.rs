use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;
use std::string::ToString;
use strum_macros::Display;

#[repr(usize)]
#[derive(Debug, IntoEnumIterator, PartialEq, IntEnum, Clone, Copy, Display)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    return _color.int_value();
}

pub fn value_to_color_string(value: usize) -> String {
    match ResistorColor::from_int(value) {
        Err(_) => String::from("value out of range"),
        Ok(val) => val.to_string(),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let mut color_vec: Vec<ResistorColor> = ResistorColor::into_enum_iter().collect();
    color_vec.sort_by(|a, b| color_to_value(*a).cmp(&color_to_value(*b)));
    color_vec
}
