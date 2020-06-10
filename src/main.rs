use termion::{color, style, cursor, clear};
use termion::raw::IntoRawMode;
use std::io::{Read, Write, stdout, stdin};

mod game;

fn main() {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let stdin = stdin();
    let stdin = stdin.lock();

    write!(stdout,
           "{}{}",
           clear::All,
           cursor::Hide,
           ).unwrap();
    stdout.flush().unwrap();

    let (width, height) = termion::terminal_size().unwrap();

    let width_scale = 255.0 / (width as f64);
    let height_scale = 255.0 / (height as f64);

    for r in 1..width {
        for g in 1..height {
            let color = color::Rgb((r as f64 * width_scale) as u8, (g as f64 * height_scale) as u8, 255);
            write!(stdout, "{}{}{}ˑ", cursor::Goto(r, g), color::Fg(color::White), color::Bg(color)).unwrap();
        }
    }
    stdout.flush().unwrap();

    let mut bytes = stdin.bytes();
    loop {
        let b = bytes.next().unwrap().unwrap();

        match b {
            b'q' => break,

            b'c' => write!(stdout, "{}", clear::All).unwrap(),

            _ => (),
        };

        stdout.flush().unwrap();
    }
    write!(stdout, "{}{}{}{}", style::Reset, cursor::Show, clear::All, cursor::Goto(1, 1)).unwrap();
    stdout.flush().unwrap();
}

