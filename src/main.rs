use failure::Error;

mod app;
mod event;
mod widgets;

use crate::app::App;

fn main() -> Result<(), Error> {
    let mut app = App::new()?;
    app.run()?;
    Ok(())
}
