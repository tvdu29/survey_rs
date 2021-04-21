use std::{
    convert::TryInto,
    error::Error,
    io::{stdin, stdout, Stdout, Write},
};
use termion::{
    color,
    cursor::{self, DetectCursorPos},
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
    terminal_size,
};

pub fn ask_select<'a>(
    message: &'a str,
    options: &'a mut Vec<&str>,
) -> Result<&'a str, Box<dyn Error>> {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode()?;

    write!(
        stdout,
        "{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide
    )?;
    write!(
        stdout,
        "{}? {}{}\n\r",
        color::LightYellow.fg_str(),
        termion::style::Reset,
        message
    )?;
    let (_, pad) = stdout.cursor_pos()?;
    for line in options.iter().enumerate() {
        write!(
            stdout,
            "{}{} {}{}\n\r",
            color::LightBlue.fg_str(),
            if line.0 == 0 { '>' } else { ' ' },
            termion::style::Reset,
            line.1
        )?;
    }

    write!(stdout, "{}", termion::cursor::Goto(2, (pad).try_into()?))?;
    stdout.flush()?;
    let mut ret = "";

    for c in stdin.keys() {
        match c? {
            Key::Char('q') => break,
            Key::Up => move_up(&mut stdout, options, &pad),
            Key::Down => move_down(&mut stdout, options, &pad),
            Key::Char('\n') => {
                let cursor = stdout.cursor_pos()?;
                ret = options[(cursor.1 - pad) as usize];
                break;
            }
            _ => continue,
        };
        stdout.flush()?;
    }
    write!(stdout, "{}", termion::cursor::Show)?;
    stdout.flush()?;
    stdout.suspend_raw_mode()?;

    Ok(ret)
}

fn move_up(stdout: &mut RawTerminal<Stdout>, data: &Vec<&str>, pad: &u16) {
    let cursor = stdout.cursor_pos().unwrap();
    if cursor.1 > *pad {
        write!(
            stdout,
            "{}  {}",
            cursor::Goto(1, cursor.1),
            data[(cursor.1 - pad) as usize]
        )
        .unwrap();
        write!(
            stdout,
            "{}{}> {}{}",
            cursor::Goto(1, cursor.1 - 1),
            color::LightBlue.fg_str(),
            termion::style::Reset,
            data[(cursor.1 - pad) as usize - 1]
        )
        .unwrap();
        stdout.flush().unwrap();
    };
}

fn move_down(stdout: &mut RawTerminal<Stdout>, data: &Vec<&str>, pad: &u16) {
    let size = terminal_size().unwrap();
    let cursor = stdout.cursor_pos().unwrap();
    if cursor.1 - pad + 1 < data.len() as u16 && cursor.1 < size.1 {
        write!(
            stdout,
            "{}  {}",
            cursor::Goto(1, cursor.1),
            data[(cursor.1 - pad) as usize]
        )
        .unwrap();
        write!(
            stdout,
            "{}{}> {}{}",
            cursor::Goto(1, cursor.1 + 1),
            color::LightBlue.fg_str(),
            termion::style::Reset,
            data[(cursor.1 - pad) as usize + 1]
        )
        .unwrap();
        stdout.flush().unwrap();
    };
}
