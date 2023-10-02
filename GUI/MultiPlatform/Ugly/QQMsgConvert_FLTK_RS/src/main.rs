use std::fmt::Debug;
use std::process::exit;
// src/main.rs
use fltk::{prelude::*, *};

mod ui1;

#[warn(unused_mut)]
fn main() {
    let app = app::App::default();
    let mut ui1 = ui1::UserInterface::make_window();


    ui1.menubar.find_item("文件/打开MHT文件").unwrap().set_callback(move |_| {
        let mut dialog =
            dialog::NativeFileChooser::new(
                dialog::NativeFileChooserType::BrowseFile
            );
        // dialog.set_directory("");
        dialog.set_filter("MHT Files\t*.mht");
        dialog.show();

        println!("选择MHT文件: {}", dialog.filename().to_str().unwrap());
        ui1.input_mht_path.set_value(dialog.filename().to_str().unwrap());
    });

    ui1.menubar.find_item("文件/关闭程序").unwrap().set_callback(move |_| {
        program_end();
    });

    ui1.menubar.find_item("帮助/关于").unwrap().set_callback(move |_| {
        ui1::UserInterface_About::make_window_about().window_about.show();
    });

    ui1.main_window.show();
    app.run().unwrap();
}

fn program_end() {
    exit(0);
}