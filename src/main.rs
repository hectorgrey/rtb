use crossterm::{
	event::{self, KeyCode, KeyEventKind},
	terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
	ExecutableCommand,
};
use ratatui::{
	prelude::{CrosstermBackend, Stylize, Terminal},
	widgets::Paragraph,
};
use std::{
	io::{stdout, Result},
	time::Duration,
};

fn main() -> Result<()> {
	stdout().execute(EnterAlternateScreen)?;
	enable_raw_mode()?;
	let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
	terminal.clear()?;

	loop {
		terminal.draw(|frame| {
			let area = frame.size();
			frame.render_widget(
				Paragraph::new("Hello World!  Press q to quit.")
					.white()
					.on_black()
					.bold(),
				area,
			);
		})?;
		if event::poll(Duration::from_millis(16))? {
			if let event::Event::Key(key) = event::read()? {
				if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
					break;
				}
			}
		}
	}

	stdout().execute(LeaveAlternateScreen)?;
	disable_raw_mode()?;

	Ok(())
}
