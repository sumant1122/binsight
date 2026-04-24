use crate::binary::BinaryInfo;
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    layout::{Layout, Constraint, Direction},
    style::{Style, Modifier, Color},
    Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;

pub fn run_tui(info: BinaryInfo) -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, info);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

struct App {
    items: Vec<(String, u64)>,
    state: ListState,
    total_size: u64,
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, info: BinaryInfo) -> io::Result<()> {
    let mut group_sizes: std::collections::HashMap<String, u64> = std::collections::HashMap::new();
    for symbol in &info.symbols {
        let name = symbol.module_path.first().cloned().unwrap_or_else(|| "unknown".to_string());
        *group_sizes.entry(name).or_insert(0) += symbol.size;
    }
    let mut items: Vec<_> = group_sizes.into_iter().collect();
    items.sort_by(|a, b| b.1.cmp(&a.1));

    let mut state = ListState::default();
    state.select(Some(0));
    
    let mut app = App { items, state, total_size: info.total_size };

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down => {
                    let i = match app.state.selected() {
                        Some(i) => {
                            if i >= app.items.len() - 1 {
                                0
                            } else {
                                i + 1
                            }
                        }
                        None => 0,
                    };
                    app.state.select(Some(i));
                }
                KeyCode::Up => {
                    let i = match app.state.selected() {
                        Some(i) => {
                            if i == 0 {
                                app.items.len() - 1
                            } else {
                                i - 1
                            }
                        }
                        None => 0,
                    };
                    app.state.select(Some(i));
                }
                _ => {}
            }
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ].as_ref())
        .split(f.size());

    let title = Paragraph::new("binsight Explorer - Press 'q' to quit")
        .block(Block::default().borders(Borders::ALL).title("Info"));
    f.render_widget(title, chunks[0]);

    let items: Vec<ListItem> = app.items.iter().map(|(name, size)| {
        let percentage = (*size as f64 / app.total_size as f64) * 100.0;
        ListItem::new(format!("{:<20} {:>10.2} MB ({:.1}%)", name, *size as f64 / (1024.0 * 1024.0), percentage))
    }).collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Crates"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow))
        .highlight_symbol(">> ");
    f.render_stateful_widget(list, chunks[1], &mut app.state);

    let footer = Paragraph::new("Use ↑/↓ to navigate")
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}
