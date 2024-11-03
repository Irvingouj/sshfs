use std::{io, sync::Arc};

use crossterm::event::{self, Event, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use tracing::{debug, info};

use crate::state::State;

#[derive(Debug)]
pub struct App {
    state: Arc<State>,
}

impl App
{
    pub async fn run(state: Arc<State>) -> io::Result<()> {
        let mut terminal = ratatui::init();
        let _ = tokio::task::spawn_blocking(move || return App { state }.run_inner(&mut terminal))
            .await??;
        Ok(())
    }

    /// runs the application's main loop until the user quits
    fn run_inner(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let state_clone = self.state.clone();
        tokio::task::spawn_blocking(move || App::handle_events(&state_clone));
        info!("Running app");
        while self.should_draw() {
            terminal.draw(|frame| self.draw(frame))?;
            debug!("Frame drawn");
        }

        ratatui::restore();
        info!("Terminal restored");
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
        debug!("Widget rendered");
    }

    fn should_draw(&self) -> bool {
        if self.state.is_exiting() {
            return false;
        }

        self.state.clone().notified();
        return true;
    }
}

impl App {
    fn handle_events(state: &Arc<State>) -> io::Result<()> {
        loop {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    match key_event.code {
                        event::KeyCode::Char('q') => {
                            state.exit();
                            break;
                        }
                        _ => {}
                    }
                }
                _ => {}
            };
        }
        Ok(())
    }
}
