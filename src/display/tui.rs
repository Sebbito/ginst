//! # App
//!
//! Crate containing the apps state and code for rendering it in the terminal

use crossterm::event::{self, Event, KeyCode};
use std::{
    io,
    time::{Duration, Instant}, error::Error};
use tui::{
    backend::Backend,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame, Terminal, layout::{Direction, Constraint, Layout},
};

use crate::types::Programable;

#[derive(Clone)]
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

    // fn unselect(&mut self) {
    //     self.state.select(None);
    // }
}

/// This struct holds the current state of the app. In particular, it has the `items` field which is a wrapper
/// around `ListState`. Keeping track of the items state let us render the associated widget with its state
/// and have access to features such as natural scrolling.
///
/// Check the event handling at the bottom to see how to change the state on incoming events.
/// Check the drawing logic for items on how to specify the highlighting style for selected items.
use crossterm::{
event::{DisableMouseCapture, EnableMouseCapture},
execute,
terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::backend::CrosstermBackend;
use crate::types::Runnable;

#[derive(Clone)]
pub struct App<T: Programable> {
    items: StatefulList<T>,
}

impl<T: Programable> App<T> {
    pub fn new(items: Vec<T>) -> App<T> {
        App {
            items: StatefulList::with_items(items)
        }
    }
}

impl<T: Programable> Runnable for App<T> {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        // setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // create app and run it
        let tick_rate = Duration::from_millis(250);
        let res = run_app(&mut terminal, self.clone(), tick_rate, false);

            // restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        if let Err(err) = res {
            println!("{:?}", err);
        }
        Ok(())

    }
}

pub fn run_app<B: Backend, T: Programable>(
    terminal: &mut Terminal<B>,
    mut app: App<T>,
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
                let selected: Option<T> = {
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
                            let dependencies = selected.unwrap().get_sublist();
                            if dependencies.len() != 0 {
                                let sub_app = App::new(dependencies);
                                run_app(terminal, sub_app.clone(), tick_rate, true)?;
                            }
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

fn generate_key_overview_box() -> Block<'static> {
    Block::default()
        .title(Span::styled(
            "q = quit, i/<Enter> = install, c = configure, arrow keys/h,j,k,l = move",
            Style::default()
                .fg(Color::White)
                .bg(Color::Red)
                .add_modifier(Modifier::BOLD),
        ))
        .title_alignment(tui::layout::Alignment::Center)
}

fn ui<B: Backend, T: Programable + Clone>(f: &mut Frame<B>, app: &mut App<T>) {
    // Iterate through all elements in the `items` app and append some info to it.
    let items: Vec<ListItem> = app
        .items
        .items
        .iter()
        .map(|i| {
            let mut lines = vec![Spans::from(i.get_name())];
            
            // get the status text
            let status = match i.is_installed() {
                true => "🗹 Installed".to_owned(),
                false => "⮽ Missing".to_owned(),
            };
            // append it to the item
            lines.push(Spans::from(
                Span::styled(status, Style::default().add_modifier(Modifier::ITALIC))
            ));
            
            // optionally append a hint for subitems to it
            if !i.get_sublist().is_empty() {
                lines.push(Spans::from(
                    Span::styled(">>>", Style::default().add_modifier(Modifier::ITALIC))
                ));
            }

            // based on status set othe colors
            match i.is_installed() {
                true => ListItem::new(lines).style(Style::default().fg(Color::White).bg(Color::Green)),
                false => ListItem::new(lines).style(Style::default().fg(Color::White).bg(Color::Red)),
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
    // let rect = f.size();

    // create two chunks one for the key overview and one for the list
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
        .split(f.size());

    // We can now render the item list
    f.render_stateful_widget(items, chunks[0], &mut app.items.state);
    f.render_widget(generate_key_overview_box(), chunks[1]);
}
