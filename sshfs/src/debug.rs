use crossterm::event::{self, Event};
use ratatui::Terminal;

pub fn debug_draw(
    draw_func: impl FnOnce(
        Terminal<ratatui::prelude::CrosstermBackend<std::io::Stdout>>,
    ) -> anyhow::Result<()>,
) -> anyhow::Result<()> {
    let terminal: Terminal<ratatui::prelude::CrosstermBackend<std::io::Stdout>> = ratatui::init();

    draw_func(terminal)?;

    loop {
        match event::read()? {
            Event::Key(event) => {
                if event.code == crossterm::event::KeyCode::Char('q') {
                    break;
                }
            }
            _ => {}
        }
    }
    ratatui::restore();
    Ok(())
}
