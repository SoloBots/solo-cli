use std::io;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use crossterm::{execute, terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};

pub fn run() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title(" Solo Interactive Dashboard ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan));
            
            let text = Paragraph::new("Welcome to your Ratatui View!\n\nPress 'q' to return to the solo prompt.")
                .alignment(Alignment::Center)
                .block(block);
                
            f.render_widget(text, size);
        })?;

        if crossterm::event::poll(std::time::Duration::from_millis(16))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if key.code == crossterm::event::KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}