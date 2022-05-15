use config::AppConfig;
use service::AppService;
use std::io::{stdin, stdout};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::Terminal;
use ui::AppUI;

mod config;
mod service;
mod ui;

static APP_NAME: &str = "news-api-view";

enum AppEvent {
    Tick,
    Quit,
}

fn main() {
    let cfg = AppConfig::load();

    let (tx, rx) = mpsc::channel::<AppEvent>();
    let tx_clone = tx.clone();
    thread::spawn(move || {
        for key in stdin().keys().flatten() {
            if key == Key::Esc {
                tx_clone.send(AppEvent::Quit).unwrap()
            }
        }
    });
    thread::spawn(move || loop {
        tx.send(AppEvent::Tick).unwrap();
        thread::sleep(Duration::from_secs(cfg.refresh_secs));
    });

    let stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let service = AppService::new(cfg.url, Duration::from_secs(cfg.interval_secs));
    let mut ui = AppUI::new(service);

    loop {
        terminal.draw(|f| ui.draw(f)).unwrap();

        match rx.recv().unwrap() {
            AppEvent::Tick => {}
            AppEvent::Quit => {
                return;
            }
        }
    }
}
