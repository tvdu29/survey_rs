extern crate termion;

mod multiselect;
mod select;

use multiselect::ask_multiselect;
use select::ask_select;

fn main() {
    let mut data = vec!["This is line 1", "This is line 2", "This is line 3"];

    println!("{}{}{}", termion::clear::All, termion::cursor::Goto(1,1), ask_select("Which line ?", &mut data).unwrap());
    
}
