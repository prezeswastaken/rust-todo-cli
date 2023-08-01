use todo_cli::app::{App, AppResult};
use todo_cli::event::{Event, EventHandler};
use todo_cli::handler::handle_key_events;
use todo_cli::{models, database_fetcher, schema};
use todo_cli::tui::Tui;
use std::io;
use tui::backend::CrosstermBackend;
use tui::Terminal;



fn main() -> AppResult<()> {


    // Create an application.
    let mut app = App::new();
    app.fetch_data();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
