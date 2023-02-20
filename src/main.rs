pub mod app;

use app::App;
use async_std::prelude::*;
use async_std::task;
use crossterm::execute;
use crossterm::{cursor::*, event::*, style::*, terminal::*, Command, QueueableCommand};
use std::io::{stdout, Write};
use std::time::Duration;

#[derive(Debug)]
pub enum TypeMode {
    Normal,
    Insert,
    Select,
    Command,
}

#[derive(Debug)]
pub enum Align {
    Left,
    Middle,
    Right,
}

#[derive(Debug)]
pub struct Title {
    text: String,
    align: Align,
}

impl Title {
    fn new() -> Self {
        Title {
            text: "".to_string(),
            align: Align::Left,
        }
    }
    fn set_title(&mut self, title: String) {
        self.text = title;
    }
    fn set_align(&mut self, align: Align) {
        self.align = align;
    }
}

// if we want both functions from event read() and poll() works with ? unwrap
// we need to return std::io::Error, but by the offical rust programming book(https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#the--operator-can-be-used-in-functions-that-return-result)
// it is recommand to use Box<dyn Error> which Error is trait(or we say, dynamic object) and io::Error is not
// so this place we use smart pointer with dynamic object impl trait std::error::Error
#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();
    enable_raw_mode()?;
    // disable_raw_mode()?;
    loop {
        if poll(Duration::from_millis(200))? {
            clear_screen(&mut app).await;
            // let event = app.es.next().await.unwrap()?;
            let event = app.next_event().await.unwrap();
            match event {
                Event::Key(event) => match event {
                    KeyEvent {
                        code: KeyCode::Char('q'),
                        ..
                    }
                    | KeyEvent {
                        code: KeyCode::Char('c'),
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    } => {
                        break;
                    }

                    _ => {}
                },
                _ => {}
            }
            render_square(&mut app, (20, 5)).await;
            show_curr_event(&mut app, event).await;
        } else {
        }
    }
    disable_raw_mode()?;
    Ok(())
}

async fn clear_screen(app: &mut App) -> Result<(), std::io::Error> {
    execute!(app.stdout, Clear(ClearType::All))
}

async fn render_square(app: &mut App, size: (usize, usize)) -> Result<(), std::io::Error> {
    // size.0 = width , size.1 = height
    if size.0 < 3 || size.1 < 3 || app.title.text.len() > size.1 {
        panic!("input block size too small (at least 3)")
    }

    let title_text = app.title.text.clone();

    // head line
    let top = match &app.title.align {
        Align::Left => format!(
            "{1}{0:─<width$}{2}",
            title_text,
            "┌",
            "┐",
            width = size.0 - 3
        ),
        Align::Middle => format!(
            "{1}{0:─^width$}{2}",
            title_text,
            "┌",
            "┐",
            width = size.0 - 3
        ),
        Align::Right => format!(
            "{1}{0:─>width$}{2}",
            title_text,
            "┌",
            "┐",
            width = size.0 - 3
        ),
    };
    execute!(app.stdout, MoveTo(0, 0), Print(top));

    // border lines
    for i in 1..size.1 - 1 {
        execute!(
            app.stdout,
            MoveTo(0, i as u16),
            Print("│"),
            MoveTo((size.0 - 2) as u16, i as u16),
            Print("│")
        );
    }

    // foot line
    execute!(
        app.stdout,
        MoveTo(0, (size.1 - 1) as u16),
        Print(format!(
            "{1}{0:─^width$}{2}",
            "",
            "└",
            "┘",
            width = size.0 - 3
        ))
    );
    Ok(())
}

async fn show_curr_event(app: &mut App, event: Event) -> Result<(), std::io::Error> {
    execute!(
        app.stdout,
        MoveTo(0, size()?.1 - 5),
        Print(format!("{:?}", event)),
        SetCursorStyle::BlinkingBlock
    )
}
