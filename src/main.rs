fn init_window() -> pancurses::Window {
    let window = pancurses::initscr();

    window.nodelay(true);
    window.refresh();
    window.keypad(true);

    pancurses::noecho();
    pancurses::nl();

    pancurses::start_color();

    for i in 0..pancurses::COLORS() {
        pancurses::init_pair(i as i16, i as i16, pancurses::COLOR_BLACK);
    }

    window
}

fn main() {
    let window = init_window();

    window.attrset(pancurses::COLOR_PAIR(1));

    let mut height = window.get_max_y();
    let mut width = window.get_max_x();

    loop {
        match window.getch() {
            Some(pancurses::Input::Character(c)) => { window.addch(c); },
            Some(pancurses::Input::KeyDC) => break,
            Some(pancurses::Input::KeyResize) => {
                height = window.get_max_y();
                width = window.get_max_x();
            },
            Some(input) => { window.addstr(&format!("{:?}", input)); },
            None => (),
        }

        window.erase();

        for x in 0..width {
            for y in 0..height {
                let index = (y * width) + x;
                let index = index % pancurses::COLORS();
                let attr = pancurses::COLOR_PAIR(index as u32);

                window.attron(attr);
                window.mvaddch(y, x, 'X');
                window.attroff(attr);
            }
        }

        window.mvprintw(height-1, 0, &format!("{}x{} - {}", width, height, pancurses::COLORS()));
    }
    pancurses::endwin();
}

