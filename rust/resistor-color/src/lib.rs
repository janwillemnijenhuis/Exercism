use int_enum::IntEnum;
use enum_iterator::IntoEnumIterator;

#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum, IntoEnumIterator)]
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

fn main() {
    let s: String = value_to_color_string(5);
    println!("{}",s);
    let v: Vec<ResistorColor> = colors();
    println!("{:?}", v);
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    _color.int_value()
}

pub fn value_to_color_string(value: usize) -> String {
    if value < 10 {
        return format!("{:?}", ResistorColor::from_int(value).unwrap());
    }
    return String::from("value out of range");
}

pub fn colors() -> Vec<ResistorColor> {
    let mut vec_resistors: Vec<ResistorColor> = vec![ResistorColor::Black;10];
    let mut iter_object = ResistorColor::into_enum_iter();
    loop {
        let mut t = iter_object.last().unwrap();
        let mut s = iter_object.next().unwrap();
        vec_resistors[color_to_value(s)] = s;
        if s == t {
            break;
        }
    }
    vec_resistors
}
