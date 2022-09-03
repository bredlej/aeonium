use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, sync::{Arc, Mutex}};
use std::sync::mpsc::{Receiver, TryRecvError};
use std::time::Duration;
use cpal::Stream;
use cpal::traits::StreamTrait;
use tui::{backend::{Backend, CrosstermBackend}, layout::{Constraint, Direction, Layout}, widgets::{Block, Borders}, Frame, Terminal, symbols};
use tui::layout::{Alignment};
use tui::style::{Color, Modifier, Style};
use tui::text::Span;
use tui::widgets::{Axis, BorderType, Chart, Dataset};
use crate::{App};
use crate::common::{BeatEvent};
use crate::ui::SampleWidget;
use crate::ui::widgets::BpmWidget;

pub fn run(stream: Stream, app: &mut Arc<Mutex<App>>, beat_receiver: &Receiver<BeatEvent>, sample_receiver: &Receiver<Vec<f32>>) -> anyhow::Result<()> {
    stream.play()?;
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_ui(&mut terminal, app, beat_receiver, sample_receiver);

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

fn run_ui<B: Backend>(terminal: &mut Terminal<B>, app: &mut Arc<Mutex<App>>, beat_receiver: &Receiver<BeatEvent>, sample_receiver: &Receiver<Vec<f32>>) -> io::Result<()> {
    loop {

        let beat_event = beat_receiver.try_recv();
        let received = match beat_event {
            Ok(_) => { true }
            Err(_) => { false }
        };

        let sample_packet = sample_receiver.try_recv();
        let samples = match sample_packet {
            Ok(samples) => {samples}
            Err(_) => {vec![]}
        };

        terminal.draw(|f| ui(f, app, received, &samples))?;

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
                        if app.bpm > 1 {
                            app.bpm -= 1;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut Arc<Mutex<App>>, received_beat: bool, samples: &Vec<f32>) {
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
    let mut data: Vec<(f64, f64)> = vec![];
    for (i, x) in samples.iter().enumerate() {
            data.push((i as f64, *x as f64));
    }
    let datasets = vec![
        Dataset::default()
            .name(format!("{}" , data.len()))
            .marker(symbols::Marker::Braille)
            .style(Style::default().fg(Color::Cyan))
            .data(data.as_slice()),
    ];
    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .title(Span::styled(
                    "Chart 1",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ))
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .title("X Axis")
                .style(Style::default().fg(Color::Gray))
                .bounds([0., data.len() as f64]),
        )
        .y_axis(
            Axis::default()
                .title("Y Axis")
                .style(Style::default().fg(Color::Gray))
                .labels(vec![
                    Span::styled("-2", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw("0"),
                    Span::styled("2", Style::default().add_modifier(Modifier::BOLD)),
                ])
                .bounds([-2.0, 2.0]),
        );

    //f.render_widget(top_bar, top_layout[0]);
    f.render_widget(chart, main_layout[1]);
    f.render_widget(bpm_widget, bpm_area);
}