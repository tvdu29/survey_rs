use std::io::{Stdout, Write, stdin, stdout};
use termion::{color, cursor::{self, DetectCursorPos}, event::Key, input::TermRead, raw::{IntoRawMode, RawTerminal}, terminal_size};

extern crate termion;

fn move_up(stdout: &mut RawTerminal<Stdout>) {
    let cursor = stdout.cursor_pos().unwrap();
    if cursor.1 > 1 {
        write!(stdout, "{}", cursor::Goto(2, cursor.1 - 1)).unwrap();
    };
}

fn move_down(stdout: &mut RawTerminal<Stdout>, data: &Vec<(bool, &str)>) {
    let size = terminal_size().unwrap();
    let cursor = stdout.cursor_pos().unwrap();
    if (cursor.1 as usize) < data.len() && cursor.1 < size.1 {
        write!(stdout, "{}", cursor::Goto(2, cursor.1 + 1)).unwrap();
    };
}

fn check(stdout: &mut RawTerminal<Stdout>, answers: &mut Vec<(bool, &str)>) {
    let cursor = stdout.cursor_pos().unwrap();
    answers[(cursor.1 - 1) as usize].0 = !answers[(cursor.1 - 1) as usize].0;
    write!(stdout, "{}{}[{}] {}{}{}", cursor::Goto(1, cursor.1), if answers[(cursor.1 - 1) as usize].0 {color::LightGreen.fg_str()} else {color::LightBlue.fg_str()}, if answers[(cursor.1 - 1) as usize].0 {'x'} else {' '}, termion::style::Reset, answers[(cursor.1 - 1) as usize].1, cursor::Goto(2, cursor.1)).unwrap();
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let data = vec!["This is line 1", "This is line 2", "This is line 3"];

    let mut answers: Vec<(bool, &str)> = data.clone().into_iter().map(|x| (false, x)).collect();

    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1,1)).unwrap();
    for line in &answers{
        write!(stdout, "{}[{}] {}{}\n\r", if line.0 {color::LightGreen.fg_str()} else {color::LightBlue.fg_str()}, if line.0 {'x'} else {' '}, termion::style::Reset, line.1).unwrap();
    };

    write!(stdout, "{}", termion::cursor::Goto(2,1)).unwrap();
    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Up => move_up(&mut stdout),
            Key::Down => move_down(&mut stdout, &answers),
            Key::Char('\n') => check(&mut stdout, &mut answers),
            _ => continue,
        };
        stdout.flush().unwrap();
    }
    write!(stdout, "{}{}", termion::clear::All, cursor::Goto(1,1)).unwrap();
    stdout.flush().unwrap();

    stdout.suspend_raw_mode().unwrap();

    answers.retain(|&x| x.0);
    answers.into_iter().for_each(|elem| {
        println!("{}", elem.1);
    });
}
