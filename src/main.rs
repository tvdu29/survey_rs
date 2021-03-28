use std::{convert::TryInto, io::{Stdout, Write, stdin, stdout}};
use termion::{clear, color::{self, Color}, cursor::{self, DetectCursorPos}, event::Key, input::TermRead, raw::{IntoRawMode, RawTerminal}, terminal_size};

extern crate termion;

fn move_up(stdout: &mut RawTerminal<Stdout>, data: &Vec<&str>) {
    let cursor = stdout.cursor_pos().unwrap();
    if cursor.1 > 1 {
        write!(stdout, "{}{}{}", termion::style::Reset, data[(cursor.1 - 1) as usize], cursor::Goto(1, cursor.1 - 1)).unwrap();
        write!(stdout, "{}{}{}{}", color::Green.fg_str(), data[(cursor.1 - 2) as usize], termion::style::Reset, cursor::Goto(1, cursor.1 - 1)).unwrap();
    };
}

fn move_down(stdout: &mut RawTerminal<Stdout>, data: &Vec<&str>) {
    let size = terminal_size().unwrap();
    let cursor = stdout.cursor_pos().unwrap();
    if (cursor.1 as usize) < data.len() && cursor.1 < size.1 {
        write!(stdout, "{}{}{}", termion::style::Reset, data[(cursor.1 - 1)as usize], cursor::Goto(1, cursor.1 + 1)).unwrap();
        write!(stdout, "{}{}{}{}", color::Green.fg_str(), data[cursor.1 as usize], termion::style::Reset, cursor::Goto(1, cursor.1 + 1)).unwrap();
    };
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let data = vec!["This is line 1", "This is line 2", "This is line 3"];

    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1,1)).unwrap();
    for line in (&data).iter().enumerate(){
        write!(stdout, "{}{}{}\n\r", if line.0 == 0 {color::Green.fg_str()} else {color::White.fg_str()}, line.1, termion::style::Reset).unwrap();
    };

    write!(stdout, "{}", termion::cursor::Goto(1,1)).unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Up => move_up(&mut stdout, &data),
            Key::Down => move_down(&mut stdout, &data),
            _ => continue,
        };
        stdout.flush().unwrap();
    }
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
