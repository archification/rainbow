extern crate rand;
extern crate termion;

use std::io::{
    stdout,
    Write
};
use rand::Rng;
use termion::{
    clear,
    color,
    cursor::Goto
};

fn clear() {
    println!(
        "{}{}",
        clear::All,
        Goto(1, 0)
    );
    let _ = stdout().flush();
}

fn stuff() {
    loop {
        let mut rng = rand::thread_rng();
        let (r, g, b): (u8, u8, u8) = rng.gen();
        let rgb = termion::color::Rgb(r, g, b);
        println!(
            "{}{}  guacamolification  {}{}",
            color::Fg(rgb),
            rng.gen::<i64>(),
            rng.gen::<i64>(),
            color::Fg(color::Reset)
        );
        println!("{}",
            color::Bg(rgb)
        );
    }
}

fn main() {
    clear();
    stuff();
}
