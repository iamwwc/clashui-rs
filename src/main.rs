use std::{io, thread, time::Duration};

use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode}, execute, event::{EnableMouseCapture, DisableMouseCapture}};
use tui::{backend::CrosstermBackend, Terminal, widgets::{Block, Borders}};
mod ui;
fn main() -> Result<(), io::Error>{
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen,EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|f| {
        let size = f.size();
        ui::ui(f);
        let block = Block::default()
        .title("Block")
        .borders(Borders::ALL);
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
