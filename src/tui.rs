use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::{execute, terminal::*};
use ratatui::prelude::*;
use ratatui::{
    symbols::border,
    widgets::{block::*, *},
};
use std::io;
use std::io::{stdout, Stdout};

#[derive(Debug, Default)]
pub struct App {
    device_id: String,
    current_playing: String,
    current_station: String,
    current_playlist: Vec<String>,
    current_song: String,
    station_list: Vec<String>,
    counter: u8,
    exit: bool,
}

impl App {
    fn initialize_station_list(&mut self) {
        self.station_list = vec![
            "Rock".to_string(),
            "Pop".to_string(),
            "MPB".to_string(),
            "Jazz".to_string(),
            "Blues".to_string(),
            "Classical".to_string(),
            "Electronic".to_string(),
            "Funk".to_string(),
            "Samba".to_string(),
            "Reggae".to_string(),
            "Rap".to_string(),
            "Country".to_string(),
            "Metal".to_string(),
            "Sertanejo".to_string(),
            "Forró".to_string(),
            "Gospel".to_string(),
            "Soul".to_string(),
            "Folk".to_string(),
            "Indie".to_string(),
            "Punk".to_string(),
            "R&B".to_string(),
        ];
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut Tui) -> io::Result<()> {
        self.initialize_station_list();
        self.current_station.clone_from(&self.station_list[0]);

        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn play_next_song(&mut self) {
        self.current_playing = "Next Song".to_string();
        println!("Playing next song");
    }

    fn play_prev_song(&mut self) {
        // self.counter -= 1;
        self.current_playing = "Previous Song".to_string();
        println!("Playing previous song");
    }

    fn play_next_station(&mut self) {
        // find the position of the current station in station_list
        let current_station_index = self
            .station_list
            .iter()
            .position(|station| station == &self.current_station)
            .unwrap();

        if current_station_index == self.station_list.len() - 1 {
            self.current_station.clone_from(&self.station_list[0]);
        } else {
            self.current_station
                .clone_from(&self.station_list[current_station_index + 1]);
        }
    }

    fn play_prev_station(&mut self) {
        let current_station_index = self
            .station_list
            .iter()
            .position(|station| station == &self.current_station)
            .unwrap();

        if current_station_index == 0 {
            self.current_station
                .clone_from(&self.station_list[self.station_list.len() - 1]);
        } else {
            self.current_station
                .clone_from(&self.station_list[current_station_index - 1]);
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.play_next_song(),
            KeyCode::Right => self.play_prev_song(),
            KeyCode::Char('n') => self.play_next_station(),
            KeyCode::Char('p') => self.play_prev_station(),
            _ => {}
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Spotify Radio TUI ".bold());
        let instructions = Title::from(Line::from(vec![
            " Prev ".into(),
            "<Left>".blue().bold(),
            " Next ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
            " Play".into(),
            "<Space>".blue().bold(),
            " Next Station ".into(),
            "<N> ".blue().bold(),
            " Previous Station ".into(),
            "<P> ".blue().bold(),
        ]));

        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::THICK);

        let station_text = Text::from(vec![Line::from(vec![
            "Playing: ".into(),
            self.current_station.to_string().yellow(),
            " Station".into(),
        ])]);

        Paragraph::new(station_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

/// A type alias for the terminal type used in this application
pub type Tui = Terminal<CrosstermBackend<Stdout>>;

/// Initialize the terminal
pub fn init() -> io::Result<Tui> {
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

/// Restore the terminal to its original state
pub fn restore() -> io::Result<()> {
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

pub fn init_ui() -> io::Result<()> {
    let mut terminal = init()?;
    let app_result = App::default().run(&mut terminal);
    restore()?;
    app_result
}
