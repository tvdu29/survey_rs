use std::{
    error::Error,
    io::{stdin, stdout, Write},
};
use termion::{cursor::DetectCursorPos, raw::IntoRawMode};

pub fn ask_input(message: &str) -> Result<String, Box<dyn Error>> {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode()?;

    write!(
        stdout,
        "{}? {}{}\n\r",
        termion::color::LightYellow.fg_str(),
        termion::style::Reset,
        message
    )?;
    let cursor = stdout.cursor_pos()?;
    write!(stdout, "{}", termion::cursor::Goto(0, cursor.1))?;

    stdout.suspend_raw_mode()?;

    let mut result = String::new();

    stdin.read_line(&mut result)?;
    result = result.trim().to_string();
    return Ok(result);
}
