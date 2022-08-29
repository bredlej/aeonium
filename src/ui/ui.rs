use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, sync::{Arc, Mutex}};
use std::sync::mpsc::{Receiver};
use std::time::Duration;
use cpal::Stream;
use cpal::traits::StreamTrait;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Frame, Terminal,
};
use tui::layout::{Alignment};
use tui::widgets::{BorderType};
use crate::{App};
use crate::common::{BeatEvent};
use crate::ui::widgets::BpmWidget;

pub fn run(stream: Stream, app: &mut Arc<Mutex<App>>, beat_receiver: &Receiver<BeatEvent>) -> anyhow::Result<()> {
    stream.play()?;
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, app, beat_receiver);

    // restore terminal
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut Arc<Mutex<App>>, beat_receiver: &Receiver<BeatEvent>) -> io::Result<()> {
    loop {


        let beat_event = beat_receiver.try_recv();
        let received = match beat_event {
            Ok(_) => { true }
            Err(_) => { false }
        };

        terminal.draw(|f| ui(f, app, received))?;

        if event::poll(Duration::from_millis(100)).unwrap() {
            if let Event::Key(key) = event::read()? {
                let mut app = app.lock().unwrap();

                match key.code {
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    KeyCode::Char('+') => {
                        app.bpm += 1;
                    }
                    KeyCode::Char('-') => {
                        app.bpm -= 1;
                    }
                    _ => {}
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut Arc<Mutex<App>>, received_beat: bool) {
    let size = f.size();

    let block = Block::default()
        .title("Aeonium v0.0.1")
        .borders(Borders::ALL)
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);

    f.render_widget(block, size);

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(50), Constraint::Percentage(40)].as_ref())
        .split(f.size());

    let top_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 3)])
        .split(main_layout[0]);

    let top_bar = Block::default()
        .title("Settings")
        .borders(Borders::ALL)
        .border_type(BorderType::Double);
    let bpm_area = top_bar.inner(top_layout[0]);

    let bpm_widget = BpmWidget { bpm: &app.lock().unwrap().bpm, has_beat: received_beat };

    f.render_widget(top_bar, top_layout[0]);
    f.render_widget(bpm_widget, bpm_area);
}