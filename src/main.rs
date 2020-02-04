extern crate rand;
extern crate termion;

use rand::Rng;
use termion::color;

fn stuff() {
    loop {
        let mut rng = rand::thread_rng();
        let (r, g, b): (u8, u8, u8) = rng.gen();
        let rgb = termion::color::Rgb(r, g, b);
        let (x, y, z): (u8, u8, u8) = rng.gen();
        let xyz = termion::color::Rgb(x, y, z);
        println!(
            "{}{}{}  guacamolification  {}{}{}",
            color::Fg(rgb),
            color::Bg(xyz),
            rng.gen::<i64>(),
            rng.gen::<i64>(),
            color::Fg(color::Reset),
            color::Bg(xyz)
        );
    }
}

fn main() {
    stuff();
}
