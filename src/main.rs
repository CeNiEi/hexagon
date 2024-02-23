use frontend::{app::App, tui::Tui};

pub(crate) mod backend;
pub(crate) mod frontend;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();

    let mut tui = Tui::new()?;
    tui.enter()?;

    while !app.terminate {
        tui.draw(&mut app)?;
        app.handle_events()?;
    }

    Ok(())
}
