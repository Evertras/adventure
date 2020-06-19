use termion::event::Key;
use termion::input::Keys;

#[derive(PartialEq, Debug)]
pub enum Action {
    HardExit,

    Up,
    Down,
    Left,
    Right,
}

pub fn to_action(c: Key) -> Option<Action> {
    match c {
        Key::Esc | Key::Ctrl('c') => Some(Action::HardExit),

        Key::Up | Key::Char('k') => Some(Action::Up),
        Key::Down | Key::Char('j') => Some(Action::Down),
        Key::Right | Key::Char('l') => Some(Action::Right),
        Key::Left | Key::Char('h') => Some(Action::Left),
        _ => None,
    }
}

pub trait Buffer {
    fn step(&mut self) -> Vec<Action>;
}

pub struct Input<R> {
    keys: Keys<R>,
}

impl<R> Input<R> {
    pub fn new(keys: Keys<R>) -> Input<R> {
        Input { keys }
    }
}

impl<R: std::io::Read> Buffer for Input<R> {
    fn step(&mut self) -> Vec<Action> {
        let mut result: Vec<Action> = vec![];

        loop {
            match self.keys.next() {
                Some(a) => match to_action(a.unwrap()) {
                    Some(a) => result.push(a),
                    None => (),
                },
                None => break,
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use termion::input::TermRead;

    #[test]
    fn reads_single_key() {
        let keys = b"k".keys();
        let mut input = Input::new(keys);

        let actions = input.step();

        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0], Action::Up);
    }

    #[test]
    fn reads_same_key_multiple_times() {
        let keys = b"kkkk".keys();
        let mut input = Input::new(keys);

        let actions = input.step();

        assert_eq!(actions.len(), 4);

        for action in actions {
            assert_eq!(action, Action::Up);
        }
    }

    #[test]
    fn reads_nothing_with_unused_keys() {
        let keys = b"____".keys();
        let mut input = Input::new(keys);

        let actions = input.step();

        assert_eq!(actions.len(), 0);
    }

    #[test]
    fn reads_key_after_unused_keys() {
        let keys = b"____j".keys();
        let mut input = Input::new(keys);

        let actions = input.step();

        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0], Action::Down);
    }

    #[test]
    fn reads_multiple_different_keys() {
        let keys = b"kj".keys();
        let mut input = Input::new(keys);

        let actions = input.step();

        assert_eq!(actions.len(), 2);
        assert_eq!(actions[0], Action::Up);
        assert_eq!(actions[1], Action::Down);
    }
}
