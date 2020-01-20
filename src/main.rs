use failure::Error;

mod app;
mod widgets;
mod event;

use crate::app::App;

fn main() -> Result<(), Error> {
    let mut app = App::new()?;
    app.run()?;
    Ok(())
}
