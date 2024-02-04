use frontend::{app::App, tui::Tui};

pub(crate) mod backend;
pub(crate) mod frontend;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();

    let mut tui = Tui::new()?;
    tui.enter()?;

    for _ in 0..1 {
        tui.draw(&mut app)?;
    }

    // tui.exit()?;
    Ok(())
}
