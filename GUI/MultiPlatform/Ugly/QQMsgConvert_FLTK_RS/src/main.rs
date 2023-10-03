// src/main.rs
// crate名字我就想这么起！我不想蛇形命名法！
#![allow(non_snake_case)]

use std::process::exit;
use fltk::{prelude::*, *};

mod program_utils;
mod core;

use program_utils::machine_info;
use core::read_mht;

mod ui1;

#[warn(unused_mut)]
fn main() {
    let app = app::App::default();
    let mut ui1 = ui1::UserInterface::make_window();

    set_menu_click_event(ui1.clone());

    println!("CORE {}", machine_info::get_cpu_core_nums());

    ui1.main_window.show();
    app.run().unwrap();
}

fn set_menu_click_event(ui1: ui1::UserInterface) {
    ui1.menubar.find_item("文件/打开MHT文件").unwrap().set_callback(move |_| {
        let mut dialog =
            dialog::NativeFileChooser::new(
                dialog::NativeFileChooserType::BrowseFile
            );
        // dialog.set_directory("");
        dialog.set_filter("MHT Files\t*.mht");
        dialog.show();
        let file_path = dialog.filename().clone();
        if file_path.clone().exists()
            && file_path.clone().to_str().unwrap().len() != 0
        {
            println!("选择MHT文件: {}", file_path.clone().to_str().unwrap());
            ui1.input_mht_path.clone()
                .set_value(file_path.clone().to_str().unwrap());
        } else {
            println!("未选择MHT文件！");
        }
    });

    ui1.menubar.find_item("输出/选择输出目录").unwrap().set_callback(move |_| {
        let mut dialog =
            dialog::NativeFileChooser::new(
                dialog::NativeFileChooserType::BrowseSaveDir
            );
        // dialog.set_directory("");
        dialog.show();
        let dir_path = dialog.filename().clone();
        println!("{}", dir_path.to_str().unwrap());
        if dir_path.clone().exists()
            && dir_path.clone().to_str().unwrap().len() != 0
        {
            println!("选择保存路径: {}", dir_path.clone().to_str().unwrap());
            ui1.input_output_dir_path.clone()
                .set_value(dir_path.clone().to_str().unwrap());
        } else {
            println!("未选择保存路径！");
        }
    });

    ui1.menubar.find_item("文件/关闭程序").unwrap().set_callback(move |_| {
        program_end();
    });

    ui1.menubar.find_item("帮助/关于").unwrap().set_callback(move |_| {
        ui1::UserInterfaceAbout::make_window_about().window_about.show();
    });
}

fn program_end() {
    exit(0);
}