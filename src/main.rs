use rand::Rng;
use crossterm::{
    style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand,
};
use std::io::{stdout, Write};

fn print_colored_text() {
    let mut rng = rand::rng();
    let mut stdout = stdout();
    loop {
        let foreground = Color::Rgb {
            r: rng.random(),
            g: rng.random(),
            b: rng.random(),
        };
        let background = Color::Rgb {
            r: rng.random(),
            g: rng.random(),
            b: rng.random(),
        };
        stdout.execute(crossterm::terminal::Clear(crossterm::terminal::ClearType::CurrentLine))
              .expect("Failed to clear line");
        stdout.execute(SetForegroundColor(foreground))
              .expect("Failed to set foreground color");
        stdout.execute(SetBackgroundColor(background))
              .expect("Failed to set background color");
        for _ in 0..11 {
            print!("{} ", rng.random::<i64>());
        }
        print!("guacamolification ");
        stdout.execute(ResetColor)
              .expect("Failed to reset colors");
        stdout.flush()
              .expect("Failed to flush stdout");
    }
}

fn main() {
    print_colored_text();
}
