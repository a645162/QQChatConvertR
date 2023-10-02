// src/main.rs
use fltk::{prelude::*, *};

mod ui1;

#[warn(unused_mut)]
fn main() {
    let app = app::App::default();
    let mut ui1 = ui1::UserInterface::make_window();
    //
    //
    ui1.main_window.show();
    app.run().unwrap();
}
