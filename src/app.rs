use crossterm::event::{self, Event, KeyCode};
use std::{
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::Backend,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState, BorderType},
    Frame, Terminal,
};

use crate::program::{Program, Status};

struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
}

/// This struct holds the current state of the app. In particular, it has the `items` field which is a wrapper
/// around `ListState`. Keeping track of the items state let us render the associated widget with its state
/// and have access to features such as natural scrolling.
///
/// Check the event handling at the bottom to see how to change the state on incoming events.
/// Check the drawing logic for items on how to specify the highlighting style for selected items.
pub struct App {
    items: StatefulList<Program>
}

impl App {
    pub fn new(programs: Vec<Program>) -> App {
        App {
            items: StatefulList::with_items(programs)
        }
    }
}

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
    is_submenu: bool
) -> io::Result<()> {
    let last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                let index = app.items.state.selected();
                let selected: Option<Program> = {
                    if index.is_some() {
                        Some(app.items.items[index.unwrap()].clone())
                    } else {
                        None
                    }
                };

                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Left | KeyCode::Char('h') => {
                        if is_submenu {
                            return Ok(());
                        }
                    },
                    KeyCode::Down | KeyCode::Char('j') => app.items.next(),
                    KeyCode::Up | KeyCode::Char('k') => app.items.previous(),
                    KeyCode::Right | KeyCode::Char('l') => {
                        // render new app with the selected items' dependencies like a submenu
                        if selected.is_some() {
                            let deps = selected.unwrap().dependencies.clone();
                            let sub_app = App::new(deps.programs);
                            run_app(terminal, sub_app, tick_rate, true);
                        }
                    },
                    KeyCode::Enter | KeyCode::Char('i') => {
                        if selected.is_some() {
                            selected.unwrap().install();
                        }
                    },
                    KeyCode::Char('c') => {
                        if selected.is_some() {
                            selected.unwrap().configure();
                        }
                    },
                    _ => {}
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // Iterate through all elements in the `items` app and append some info to it.
    let items: Vec<ListItem> = app
        .items
        .items
        .iter()
        .map(|i| {
            let mut lines = vec![Spans::from(i.name.clone())];
            
            // get the status text
            let status = {
                if i.status == Status::Missing {
                    "⮽ Missing"
                } else {
                    "🗹 Installed"
                }
            };
            // append it to the item
            lines.push(Spans::from(
                Span::styled(status, Style::default().add_modifier(Modifier::ITALIC))
            ));
            
            // optionally append a hint for subitems to it
            if i.has_dependencies() {
                lines.push(Spans::from(
                    Span::styled(">>>", Style::default().add_modifier(Modifier::ITALIC))
                ));
            }

            // based on status set othe colors
            match i.status {
                Status::Missing => ListItem::new(lines).style(Style::default().fg(Color::White).bg(Color::Red)),
                Status::Installed => ListItem::new(lines).style(Style::default().fg(Color::White).bg(Color::Green)),
            }
            
        })
        .collect();

    // Create a List from all list items and highlight the currently selected one
    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("List"))
        .highlight_style(
            Style::default()
                .bg(Color::Gray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("-> ");

    // take the terminal size as window size
    let rect = f.size();

    // We can now render the item list
    f.render_stateful_widget(items, rect, &mut app.items.state);
}
