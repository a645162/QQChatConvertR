// src/main.rs
// crate名字我就想这么起！我不想蛇形命名法！
#![allow(non_snake_case)]
// #![allow(unused_mut)]

use std::path::Path;
use std::process::exit;
use fltk::{prelude::*, *};
use fltk::enums::Event;
use ini::Ini;

mod program_utils;
mod core;

use program_utils::machine_info;
use core::read_mht;
use program_utils::program_output;

mod ui1;

const CONFIG_PATH: &str = "config/config.ini";

fn main() {
    print_author_text();

    let app = app::App::default();
    let mut ui1 = ui1::UserInterface::make_window();

    // 设置组件响应事件
    set_menu_click_event(ui1.clone());
    set_file_drop(ui1.clone());

    // 设置默认值/读取配置
    set_config(ui1.clone());

    ui1.btn_start.set_callback(move |_| {});

    ui1.main_window.show();
    app.run().unwrap();
}

fn set_config(ui1: ui1::UserInterface) {
    let cpu_core_num = machine_info::get_cpu_core_nums();
    let recommend_thread_num = machine_info::get_thread_nums();

    println!("CPU core number {}", cpu_core_num.clone());
    println!("Recommend thread number {}", recommend_thread_num.clone());

    ui1.box_cpu_core_num.clone()
        .set_label(&format!("当前CPU核心数:{}", cpu_core_num.clone()));

    ui1.box_recommend_thread_num.clone()
        .set_label(&format!("推荐线程数:{}", recommend_thread_num.clone()));

    // if Path::new(CONFIG_PATH).exists() {
    //     println!("配置文件存在，读取配置文件...");
    //     let conf = Ini::load_from_file(CONFIG_PATH).unwrap();
    //
    //     let section_cpu = conf.section(Some("CPU")).unwrap();
    //     let str_max_core_number = section_cpu.get("MAX_CORE_NUMBER").unwrap();
    //     let max_cores = str_max_core_number.parse::<usize>().unwrap();
    //     println!("最大线程数:{}", max_cores);
    //
    //     // 获取推荐的线程数
    //     let suggest_thread_num = machine_info::get_thread_nums();
    //
    //     if max_cores <= 0 {
    //         println!("最大线程数不能小于等于0，将使用推荐配置");
    //         thread_num = suggest_thread_num;
    //     } else {
    //         let use_recommend =
    //             section_cpu.get("USE_RECOMMEND_CORE_NUM").unwrap().trim() == "1";
    //
    //         if use_recommend {
    //             println!("程序推荐的线程数:{}", suggest_thread_num);
    //             thread_num = min(suggest_thread_num, max_cores);
    //         }
    //     }
    //
    //
    //     let section_output = conf.section(Some("Output")).unwrap();
    //     let str_lines_count_per_file = section_output.get("LINES_COUNT_PER_FILE").unwrap();
    //     let conf_lines_count_per_file = str_lines_count_per_file.parse::<usize>().unwrap();
    //     lines_count_per_file = conf_lines_count_per_file;
    // } else {
    //     println!("配置文件不存在，使用默认配置。")
    // }
    //
    // println!("当前系统CPU核心数:{}", machine_info::get_cpu_core_nums());
    // println!("最终使用的线程数:{}", thread_num);
}

fn set_file_drop(ui1: ui1::UserInterface) {
    ui1.main_window.clone().handle({
        let mut dnd = false;
        let mut released = false;

        move |_, ev| match ev {
            Event::DndEnter => {
                dnd = true;
                true
            }
            Event::DndDrag => true,
            Event::DndRelease => {
                released = true;
                true
            }
            Event::Paste => {
                if dnd && released {
                    let path = app::event_text();
                    // ui1.input_mht_path.clone()
                    //     .set_value(&path.clone());
                    // buf.append(&path);
                    println!("拖入文件: {}", path.clone());
                    dnd = false;
                    released = false;
                    true
                } else {
                    false
                }
            }
            Event::DndLeave => {
                dnd = false;
                released = false;
                true
            }
            _ => false,
        }
    });
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

fn print_author_text(){
    program_output::print_line("=", 50);
    println!("QQ聊天记录MHT文件解码工具");
    println!("作者: 孔昊旻");
    program_output::print_line("=", 50);
    println!("注意：本程序为开源的免费软件，仅供学习交流使用，不得用于商业用途！");
    println!("Github地址:");
    println!("https://github.com/a645162/QQChatConvertR");
    program_output::print_line("=", 50);
}

fn program_end() {
    exit(0);
}