// src/main.rs
// crate名字我就想这么起！我不想蛇形命名法！
#![allow(non_snake_case)]
// #![allow(unused_mut)]

use std::cmp::{max, min};
use std::collections::HashMap;
use std::path::Path;
use std::process::exit;
use fltk::{prelude::*, *};
use fltk::enums::{Event, Font};
use fltk::image::{Pixmap, RgbImage};
use fltk_theme::{color_themes, ColorTheme, SchemeType, ThemeType, WidgetScheme, WidgetTheme};
use ini::Ini;

mod program_utils;
mod core;

use program_utils::machine_info;
use core::read_mht;
use program_utils::program_output;
use program_utils::config;

mod ui1;

const CONFIG_PATH: &str = "config/config.ini";

const DEFAULT_DIRECTORY: &str = "";

fn main() {
    print_author_text();

    let app: app::App =
        app::App::default()
            .with_scheme(app::Scheme::Gtk);

    let mut ui1 = ui1::UserInterface::make_window();

    // 设置主题
    set_theme();
    set_widget_theme(ui1.clone());


    // 设置组件响应事件
    set_menu_click_event(app.clone(), ui1.clone());
    set_file_drop(ui1.clone());
    set_widget_event(app.clone(), ui1.clone());

    // 设置默认值/读取配置
    set_config(ui1.clone());

    ui1.btn_start.set_callback(move |_| {
        dialog::message_default("执行完毕！");
    });

    ui1.main_window.show();


    app.run().unwrap();
}

fn set_theme() {
    // 设置字体
    let font = Font::load_font("fonts/msyh.ttc").unwrap();
    Font::set_font(Font::Helvetica, &font);

    // 设置主题
    // let color_theme =
    //     ColorTheme::new(color_themes::BLACK_THEME);
    // color_theme.apply();
    let widget_theme =
        WidgetTheme::new(ThemeType::Metro);
    widget_theme.apply();
    // let widget_scheme=
    //     WidgetScheme::new(SchemeType::Fluent);
    // widget_scheme.apply();
}

fn set_widget_theme(ui1: ui1::UserInterface) {
    ui1.box_recommend_thread_num.clone()
        .set_align(enums::Align::Left | enums::Align::Inside);

    // // 设置图标
    // // let icon_image = RgbImage::from("icon.png").unwrap();
    // let mut pxm = Pixmap::new(&"res/logo.png").unwrap();
    // // 设置窗口图标
    // ui1.main_window.clone().set_icon(Some(icon_image));
}

fn set_config(ui1: ui1::UserInterface) {
    let mut cpu_core_num = machine_info::get_cpu_core_nums();
    let recommend_thread_num = machine_info::get_thread_nums();

    println!("CPU truly core number {}", cpu_core_num.clone());
    println!("Recommend thread number {}", recommend_thread_num.clone());


    if Path::new(CONFIG_PATH).exists() {
        println!("配置文件存在，读取配置文件...");
        program_output::print_line("-", 50);
        let ini_hashmap: HashMap<String, HashMap<String, String>> =
            config::read_ini_hashmap(CONFIG_PATH);

        // [CPU]
        let str_max_core_number =
            config::get_value(
                ini_hashmap.clone(),
                "CPU",
                "MAX_CORE_NUMBER",
                "0",
            );
        let max_cores = str_max_core_number.parse::<usize>().unwrap();
        if max_cores > 0 {
            println!("配置文件定义的最大线程数:{}", max_cores);
            cpu_core_num = min(max_cores, cpu_core_num.clone());
            println!("最终最大的线程数:{}", cpu_core_num.clone());
        }

        let str_worker_number =
            config::get_value(
                ini_hashmap.clone(),
                "CPU",
                "WORKER_NUMBER",
                "0",
            );
        let worker_number = str_worker_number.parse::<usize>().unwrap();
        if worker_number > 0 {
            if worker_number < cpu_core_num {
                println!("配置文件定义的工作线程数:{}", worker_number);
                ui1.spinner_work_thread.clone()
                    .set_value(worker_number as f64);
            } else {
                println!("超过最大线程数，使用最大线程数！{}", cpu_core_num.clone());
                ui1.spinner_work_thread.clone()
                    .set_value(cpu_core_num as f64);
            }
        } else {
            let final_number = min(recommend_thread_num, cpu_core_num);
            println!(
                "配置文件未规定工作线程数，使用(推荐值与最大值)的最小值{}",
                final_number.clone()
            );
            ui1.spinner_work_thread.clone()
                .set_value(final_number as f64);
        }

        // [Pretreatment]
        let str_tmp_lines_number =
            config::get_value(
                ini_hashmap.clone(),
                "Pretreatment",
                "TMP_LINES_COUNT_PER_FILE",
                "0",
            );
        let tmp_lines_number = str_tmp_lines_number.parse::<usize>().unwrap();
        if tmp_lines_number > 5000 {
            println!("配置文件定义的临时文件行数:{}", tmp_lines_number);
            ui1.input_tmp_lines_num.clone()
                .set_value(tmp_lines_number as f64);
        }

        // [Output]
        let str_lines_count_per_file =
            config::get_value(
                ini_hashmap.clone(),
                "Output",
                "LINES_COUNT_PER_FILE",
                "30000",
            );
        let conf_lines_count_per_file =
            str_lines_count_per_file.parse::<usize>().unwrap();
        ui1.input_output_html_lines_num.clone()
            .set_value(conf_lines_count_per_file as f64);

        ui1.checkbox_output_ori_path.clone()
            .set_checked(
                config::get_value(
                    ini_hashmap.clone(),
                    "Output",
                    "OUTPUT_TO_ORIGIN_DIRECTORY",
                    "1",
                )
                    == "1"
            );

        ui1.checkbox_output_sub_dir.clone()
            .set_checked(
                config::get_value(
                    ini_hashmap.clone(),
                    "Output",
                    "AUTO_CREATE_CHILD_DIRECTORY",
                    "1",
                )
                    == "1"
            );

        // [AfterOutput]
        ui1.checkbox_output_clean_tmp.clone()
            .set_checked(
                config::get_value(
                    ini_hashmap.clone(),
                    "AfterOutput",
                    "CLEAN_ALL_TMP_FILES",
                    "1",
                )
                    == "1"
            );

        println!("配置文件读取完毕！");
        program_output::print_line("-", 50);
    } else {
        println!("配置文件不存在，使用默认配置。")
    }

    ui1.box_cpu_core_num.clone()
        .set_label(&format!("当前CPU核心数:{}", cpu_core_num.clone()));

    ui1.box_recommend_thread_num.clone()
        .set_label(&format!("推荐线程数:{}", recommend_thread_num.clone()));

    ui1.spinner_work_thread.clone()
        .set_maximum(cpu_core_num.clone() as f64);
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

fn set_widget_event(app: app::App, ui1: ui1::UserInterface) {
    // 防止按 Esc键 时关闭窗口
    ui1.main_window.clone()
        .set_callback(move |_| {
            if app::event() == enums::Event::Close {
                dialog::message_default("试图关闭程序！");
                // app.quit();
            }
        });

    ui1.checkbox_output_ori_path.clone()
        .set_callback(move |_| {
            if ui1.checkbox_output_ori_path.clone().is_checked() {
                ui1.input_output_dir_path.clone().deactivate();
            } else {
                ui1.input_output_dir_path.clone().activate();
            }
        });

    ui1.spinner_work_thread.clone()
        .set_callback(move |_| {
            if ui1.spinner_work_thread.clone().value()
                >
                ui1.spinner_work_thread.clone().maximum()
            {
                ui1.spinner_work_thread.clone().set_value(
                    ui1.spinner_work_thread.clone().maximum()
                );
            }
        });

    // 保存配置按钮
    ui1.btn_save_cfg.clone()
        .set_callback(move |_| {});
}

fn save_cfg() {}

fn set_menu_click_event(app: app::App, ui1: ui1::UserInterface) {
    ui1.menubar.find_item("文件/打开MHT文件").unwrap().set_callback(move |_| {
        let mut dialog =
            dialog::NativeFileChooser::new(
                dialog::NativeFileChooserType::BrowseFile
            );
        let current_dir =
            program_utils::path::get_current_dir();
        let current_dir_str = current_dir.to_str().unwrap();
        dialog.set_directory(
            &current_dir_str
        ).expect("设置默认路径失败！");

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
        program_end(app.clone());
    });

    ui1.menubar.find_item("帮助/关于").unwrap().set_callback(move |_| {
        ui1::UserInterfaceAbout::make_window_about().window_about.show();
    });
}

fn print_author_text() {
    program_output::print_line("=", 50);
    println!("QQ聊天记录MHT文件解码工具");
    println!("作者: 孔昊旻");
    program_output::print_line("=", 50);
    println!("注意：本程序为开源的免费软件，仅供学习交流使用，不得用于商业用途！");
    println!("Github地址:");
    println!("https://github.com/a645162/QQChatConvertR");
    program_output::print_line("=", 50);
}

fn program_end(app: app::App) {
    app.quit();
    // exit(0);
}