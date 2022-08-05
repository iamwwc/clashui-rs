use std::{io, thread, time::Duration, fmt::Error};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, EventStream, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, cursor::position,
};
use futures::{select, FutureExt, StreamExt};
use futures_timer::Delay;
use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders},
    Terminal,
};
mod ui;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    
    print_event().await?;
    // term()?;
    Ok(())
}

fn term() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|f| {
        let size = f.size();
        ui::ui(f);
        let block = Block::default().title("Block").borders(Borders::ALL);
        f.render_widget(block, size);
    })?;
    thread::sleep(Duration::from_millis(5000));
    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

async fn print_event() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let mut reader = EventStream::new();

    loop {
        let mut event = reader.next().fuse();
        select! {
            maybe_event = event => {
                match maybe_event {
                    Some(Ok(event)) => {
                        println!("Event::{:?}\r", event);

                        if event == Event::Key(KeyCode::Char('c').into()) {
                            println!("Cursor position: {:?}\r", position());
                        }

                        if event == Event::Key(KeyCode::Esc.into()) {
                            break;
                        }
                    }
                    Some(Err(err)) => println!("Error: {:?}\r", err),
                    None => break,
                }
            }
        };
    }
    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}
