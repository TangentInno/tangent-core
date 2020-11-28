/*
*
*	Author: Austin Mullins
*	Copyright: Tangent
*
*/

use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn print(print_color: Color, msg: &str) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(print_color))).unwrap();
    writeln!(&mut stdout, "{}", msg);
}

fn formatter(part: &str, msg: &str) -> String {
    ["[", part, "]", " -- ", msg].join("")
}

pub fn print_normal(part: &str, msg: &str) {
    print(Color::Green, &formatter(part, msg))
}

pub fn print_warning(part: &str, msg: &str) {
    print(Color::Yellow, &formatter(part, msg))
}

pub fn print_error(part: &str, msg: &str) {
    print(Color::Red, &formatter(part, msg))
}