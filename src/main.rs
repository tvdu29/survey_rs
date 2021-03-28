use std::io::{stdin, stdout, Write};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

extern crate termion;

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, r#"{}{}Size{:?} q to exit, Type stuff, use alt, and so on...{}"#,
        termion::clear::All,
        termion::cursor::Goto(1,1),
        termion::terminal_size().unwrap(),
        termion::cursor::Hide).unwrap();
    stdout.flush().unwrap();

    for c in stdin.keys() {
        write!(stdout, "{}{}", termion::cursor::Goto(1,1), termion::clear::CurrentLine).unwrap();
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char(c) => println!("{}", c),
            Key::Alt(c) => println!("^{}", c),
            Key::Ctrl(c) => println!("*{}", c),
            Key::Esc => println!("ESC"),
            Key::Left => println!("←"),
            Key::Right => println!("→"),
            Key::Up => println!("↑"),
            Key::Down => println!("↓"),
            Key::Backspace => println!("×"),
            _ => println!("other"),
        };
        stdout.flush().unwrap();
    }
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
