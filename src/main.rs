extern crate rand;
extern crate crossterm;

use rand::Rng;
use crossterm::{
    style::{
        Color,
        ResetColor,
        SetBackgroundColor,
        SetForegroundColor
    },
};

fn stuff() {
    loop {
        let mut rng = rand::thread_rng();
        let (r, g, b): (u8, u8, u8) = rng.gen();
        let rgb = Color::Rgb { r:r, g:g, b:b };
        let (x, y, z): (u8, u8, u8) = rng.gen();
        let xyz = Color::Rgb { r:x, g:y, b:z };
        println!(
            "{}{}{}{}{}{}{} guacamolification {}{}{}{}{}{}",
            SetForegroundColor(rgb),
            SetBackgroundColor(xyz),
            rng.gen::<i64>(),
            rng.gen::<i64>(),
            rng.gen::<i64>(),
            rng.gen::<i64>(),
            rng.gen::<i64>(),
            rng.gen::<i64>(),
            rng.gen::<i64>(),
            rng.gen::<i64>(),
            rng.gen::<i64>(),
            rng.gen::<i64>(),
            ResetColor
        );
    }
}

fn main() {
    stuff();
}
