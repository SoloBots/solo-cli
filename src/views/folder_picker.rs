use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub struct FolderPicker {
    current_dir: PathBuf,
    items: Vec<PathBuf>,
    state: ListState,
}

impl FolderPicker {
    pub fn new() -> Self {
        let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let mut picker = Self {
            current_dir,
            items: Vec::new(),
            state: ListState::default(),
        };
        picker.refresh_items();
        picker
    }

    /// Reads directories under the current target path
    fn refresh_items(&mut self) {
        let mut new_items = Vec::new();

        // Always add a helper shortcut to go up a directory level
        if let Some(parent) = self.current_dir.parent() {
            new_items.push(parent.to_path_buf());
        }

        if let Ok(entries) = fs::read_dir(&self.current_dir) {
            for entry in entries.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        new_items.push(entry.path());
                    }
                }
            }
        }
        self.items = new_items;
        if !self.items.is_empty() {
            self.state.select(Some(0));
        } else {
            self.state.select(None);
        }
    }

    pub fn next(&mut self) {
        if let Some(current) = self.state.selected() {
            if !self.items.is_empty() {
                // Simply modulo by the length of the items array
                self.state.select(Some((current + 1) % self.items.len()));
            }
        }
    }

    pub fn previous(&mut self) {
        if let Some(current) = self.state.selected() {
            if !self.items.is_empty() {
                let len = self.items.len();
                // Prevents underflow if wrapping backward
                self.state.select(Some((current + len - 1) % len));
            }
        }
    }

    /// Runs the interactive loop. Returns Option<PathBuf> if a selection was locked in.
    pub fn run(&mut self) -> io::Result<Option<PathBuf>> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let mut selected_path = None;

        loop {
            terminal.draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Min(1),
                        Constraint::Length(3),
                    ])
                    .split(f.area());

                // Top Panel: Instructions
                let header1 =
                    Paragraph::new("Choose Folder in which to clone all the repos for the project")
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .border_type(BorderType::Thick)
                                .border_style(Style::default().fg(Color::Magenta))
                                .title(Line::from(" Instructions ").bold().fg(Color::Cyan)),
                        );
                f.render_widget(header1, chunks[0]);
                // Top Panel: Instructions
                let header2 =
                    Paragraph::new(format!("📂 Current Path: {}", self.current_dir.display()))
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .title(" Target Location "),
                        );
                f.render_widget(header2, chunks[1]);

                // Middle Panel: Folder List
                let list_items: Vec<ListItem> = self
                    .items
                    .iter()
                    .map(|path| {
                        let name = path
                            .file_name()
                            .map(|n| n.to_string_lossy().into_owned())
                            .unwrap_or_else(|| ".. (Go Up)".to_string());

                        // Style parent folder shortcut differently
                        if path == &self.current_dir.parent().unwrap_or_else(|| Path::new("")) {
                            ListItem::new(format!(" ⬆️  .. (Go Up)"))
                        } else {
                            ListItem::new(format!(" 📁 {}", name))
                        }
                    })
                    .collect();

                let list = List::new(list_items)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title(" Subdirectories "),
                    )
                    .highlight_style(
                        Style::default()
                            .bg(Color::Blue)
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    )
                    .highlight_symbol(">> ");

                f.render_stateful_widget(list, chunks[2], &mut self.state);

                // Bottom Panel: Footer Controls
                let footer = Paragraph::new(
                    "▲/▼: Navigate | Enter: Enter Folder | Space: Choose this folder | q: Cancel",
                )
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL));
                f.render_widget(footer, chunks[3]);
            })?;

            if crossterm::event::poll(std::time::Duration::from_millis(16))? {
                if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                    match key.code {
                        crossterm::event::KeyCode::Char('q') => {
                            break;
                        }
                        crossterm::event::KeyCode::Down => self.next(),
                        crossterm::event::KeyCode::Up => self.previous(),
                        crossterm::event::KeyCode::Enter => {
                            if let Some(idx) = self.state.selected() {
                                let target = self.items[idx].clone();
                                self.current_dir = target;
                                self.refresh_items();
                            }
                        }
                        crossterm::event::KeyCode::Char(' ') => {
                            // Selected! Lock in this path.
                            selected_path = Some(self.current_dir.clone());
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;
        Ok(selected_path)
    }
}
