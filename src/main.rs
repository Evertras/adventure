use std::io::{stdout, Write};

use termion::{async_stdin, clear, cursor, input::TermRead, raw::IntoRawMode, style};

mod game;

use game::systems::render::{Color, Renderer};

struct Terminal<'a, W: Write> {
    stdout: &'a mut termion::raw::RawTerminal<W>,
}

impl<'a, W: Write> Renderer for Terminal<'a, W> {
    fn draw_at(&mut self, x: u16, y: u16, fg: &Color, bg: &Color, rune: char) {
        use termion::*;
        write!(
            self.stdout,
            "{}{}{}{}",
            cursor::Goto(x, y),
            color::Fg(color::Rgb(fg.r, fg.g, fg.b)),
            color::Bg(color::Rgb(bg.r, bg.g, bg.b)),
            rune,
        )
        .unwrap();
    }

    fn size(&self) -> (u16, u16) {
        termion::terminal_size().unwrap()
    }

    fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }
}

fn main() {
    let stdout = stdout();
    let stdin = async_stdin();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();

    write!(stdout, "{}{}", clear::All, cursor::Hide,).unwrap();
    stdout.flush().unwrap();

    let terminal = Terminal {
        stdout: &mut stdout,
    };

    let input = game::input::Input::new(stdin.keys());

    game::run(terminal, input);

    write!(
        stdout,
        "{}{}{}{}",
        style::Reset,
        cursor::Show,
        clear::All,
        cursor::Goto(1, 1)
    )
    .unwrap();
    stdout.flush().unwrap();
}
