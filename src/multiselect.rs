use std::{convert::TryInto, error::Error, io::{stdin, stdout, Stdout, Write}};
use termion::{
    color,
    cursor::{self, DetectCursorPos},
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
    terminal_size,
};

struct Multiselect<'a> {
    prompt: &'a str,
    options: Vec<&'a str>,
}

pub fn ask_multiselect<'a>(message: &'a str, options: &'a mut Vec<&str>) -> Result<&'a Vec<&'a str>, Box<dyn Error>> {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode()?;

    let mut answers: Vec<(bool, &str)> = options
        .clone()
        .into_iter()
        .map(|x| (false, x))
        .collect();

    write!(
        stdout,
        "{}? {}{}\n\r",
        color::LightYellow.fg_str(),
        termion::style::Reset,
        message
    )?;
    let pad = {
        let cursor = stdout.cursor_pos()?;
        let size = terminal_size()?;
        match (cursor.1 as usize) < (size.1 as usize + 1) - &answers.len() {
            true => cursor.1 as usize,
            false => (size.1 as usize + 1) - &answers.len(),
        }
    };

    for line in &answers {
        write!(
            stdout,
            "{}[{}] {}{}{}",
            if line.0 {
                color::LightGreen.fg_str()
            } else {
                color::LightBlue.fg_str()
            },
            if line.0 { 'x' } else { ' ' },
            termion::style::Reset,
            line.1,
            if answers.iter().position(|x| x == line).unwrap() < answers.len() - 1 {"\n\r"} else {""},
        )?;
    }

    write!(stdout, "{}", termion::cursor::Goto(2, pad.try_into().unwrap()))?;
    stdout.flush()?;

    for c in stdin.keys() {
        match c? {
            Key::Char('q') => break,
            Key::Up => move_up(&mut stdout, &pad.try_into().unwrap()),
            Key::Down => move_down(&mut stdout, &answers, &pad.try_into().unwrap()),
            Key::Char('\n') => check(&mut stdout, &mut answers, &pad.try_into().unwrap()),
            _ => continue,
        };
        stdout.flush()?;
    }

    stdout.suspend_raw_mode()?;

    options.retain(|&x| answers.contains(&(true, x)));
    Ok(options)
}

fn move_up(stdout: &mut RawTerminal<Stdout>, pad: &u16) {
    let cursor = stdout.cursor_pos().unwrap();
    if cursor.1 > *pad {
        write!(stdout, "{}", cursor::Goto(2, cursor.1 - 1)).unwrap();
    };
}

fn move_down(stdout: &mut RawTerminal<Stdout>, data: &Vec<(bool, &str)>, pad: &u16) {
    let size = terminal_size().unwrap();
    let cursor = stdout.cursor_pos().unwrap();
    if cursor.1 - pad + 1 < data.len() as u16 && cursor.1 < size.1 {
        write!(stdout, "{}", cursor::Goto(2, cursor.1 + 1)).unwrap();
    };
}

fn check(stdout: &mut RawTerminal<Stdout>, answers: &mut Vec<(bool, &str)>, pad: &u16) {
    let cursor = stdout.cursor_pos().unwrap();
    answers[(cursor.1 - pad) as usize].0 = !answers[(cursor.1 - pad) as usize].0;
    write!(
        stdout,
        "{}{}[{}] {}{}{}",
        cursor::Goto(1, cursor.1),
        if answers[(cursor.1 - pad) as usize].0 {
            color::LightGreen.fg_str()
        } else {
            color::LightBlue.fg_str()
        },
        if answers[(cursor.1 - pad) as usize].0 {
            'x'
        } else {
            ' '
        },
        termion::style::Reset,
        answers[(cursor.1 - pad) as usize].1,
        cursor::Goto(2, cursor.1)
    )
    .unwrap();
}