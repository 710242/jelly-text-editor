use crate::*;

#[derive(Debug)]
pub struct App {
    pub es: EventStream,
    pub mode: TypeMode,
    pub stdout: std::io::Stdout,
    pub title: Title,
}

impl App {
    pub fn new() -> Self {
        App {
            es: EventStream::new(),
            mode: TypeMode::Normal,
            stdout: stdout(),
            title: Title::new(),
        }
    }
    pub fn match_event(&mut self, event: Event) -> Option<bool> {
        match self.mode {
            TypeMode::Normal => match event {
                Event::Key(event) => match event {
                    KeyEvent {
                        code: KeyCode::Char('q'),
                        ..
                    }
                    | KeyEvent {
                        code: KeyCode::Char('c'),
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    } => Some(true),
                    KeyEvent {
                        code: KeyCode::Right,
                        ..
                    } => {
                        self.title.set_align(Align::Right);
                        None
                    }
                    KeyEvent {
                        code: KeyCode::Left,
                        ..
                    } => {
                        self.title.set_align(Align::Left);
                        None
                    }
                    KeyEvent {
                        code: KeyCode::Up, ..
                    } => {
                        self.title.set_align(Align::Middle);
                        None
                    }
                    _ => None,
                },
                _ => None,
            },
            TypeMode::Insert => None,
            TypeMode::Select => None,
            TypeMode::Command => None,
        }
    }
    pub async fn next_event(&mut self) -> Result<Event, std::io::Error> {
        self.es.next().await.unwrap()
    }
}
