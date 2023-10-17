// src/main.rs
// crate名字我就想这么起！我不想蛇形命名法！
#![allow(non_snake_case)]
// #![allow(unused_mut)]

use std::cmp::{max, min};
use std::collections::HashMap;
use std::process::exit;
use fltk::{prelude::*, *};
use fltk::enums::{Event, Font};
use fltk::image::{Pixmap, RgbImage};
use fltk_theme::{color_themes, ColorTheme, SchemeType, ThemeType, WidgetScheme, WidgetTheme};
use ini::Ini;

use std::env;
use std::path::{Path, PathBuf};
use time::*;
use std::time::{SystemTime};

mod program_utils;
mod core;

use QQMsgConvert_FLTK_RS::core::read_mht;
use QQMsgConvert_FLTK_RS::program_utils::machine_info;
use QQMsgConvert_FLTK_RS::program_utils::program_output;
use QQMsgConvert_FLTK_RS::program_utils::config;
use QQMsgConvert_FLTK_RS::program_utils::path;

mod ui1;

const CONFIG_PATH: &str = "config/config.ini";

const DEFAULT_DIRECTORY: &str = "";

fn main() {
    print_author_text();

    // 读取运行参数
    let mut file_path = "";
    let args: Vec<String> = env::args().collect();
    for i in 1..args.len() {
        let arg = (args[i]).trim();

        if arg.to_lowercase().ends_with(".mht") {
            let mht_file_path = PathBuf::from(arg);
            if mht_file_path.is_file() {
                file_path = arg;
                println!("检测到mht文件 {}", arg);
                break;
            }
        }
    }

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

    // 从运行参数导入mht文件的路径
    if file_path.len() > 0 {
        ui1.input_mht_path.clone()
            .set_value(file_path);
    }

    // 设置默认值/读取配置
    set_config(ui1.clone());

    ui1.btn_start.set_callback(move |_| {
        dialog::message_default("执行完毕！");
    });

    let ui_clone_save = ui1.clone();
    // 保存配置按钮
    ui_clone_save.btn_save_cfg.clone()
        .set_callback(move |_| {
            save_cfg(ui_clone_save.clone());
        });


    ui1.main_window.show();


    app.run().unwrap();
}

// 设置主题
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

// 设置相关配置
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
                "20000",
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

// 注册文件拖拽监听
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

// 设置控件事件
fn set_widget_event(app: app::App, ui1: ui1::UserInterface) {
    let save_ui = ui1.clone();
    // 保存配置按钮
    save_ui.btn_save_cfg.clone()
        .set_callback(move |_| {
            save_cfg(save_ui.clone());
        });

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
}

// 保存配置
fn save_cfg(ui1: ui1::UserInterface) {
    let mut conf = Ini::load_from_file(CONFIG_PATH).unwrap();

    conf.with_section(None::<String>)
        .set("encoding", "utf-8");

    // [Program]
    conf.with_section(Some("Program"))
        .set(
            "AUTO_SAVE_CONFIG",
            program_output::bool_to_binary_str(
                ui1.checkbox_auto_save_cfg.clone().is_checked()
            ),
        );

    // [Output]
    conf.with_section(Some("Output"))
        .set(
            "OUTPUT_TO_ORIGIN_DIRECTORY",
            program_output::bool_to_binary_str(
                ui1.checkbox_output_ori_path.clone().is_checked()
            ),
        )
        .set(
            "AUTO_CREATE_CHILD_DIRECTORY",
            program_output::bool_to_binary_str(
                ui1.checkbox_output_sub_dir.clone().is_checked()
            ),
        );

    // [AfterOutput]
    conf.with_section(Some("AfterOutput"))
        .set(
            "CLEAN_ALL_TMP_FILES",
            program_output::bool_to_binary_str(
                ui1.checkbox_output_clean_tmp.clone().is_checked()
            ),
        );


    conf.write_to_file(CONFIG_PATH).unwrap();
}

// 设置菜单项事件
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


        let current_dir =
            program_utils::path::get_current_dir();
        let mut current_dir_str = String::from(current_dir.to_str().unwrap());
        if ui1.input_output_dir_path.clone().value().len() != 0 {
            current_dir_str = ui1.input_output_dir_path.clone().value();
        }

        dialog.set_directory(
            &current_dir_str.as_str()
        ).expect("设置默认路径失败！");

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

// 输出作者信息
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

// 开始处理过程
fn start_process(ui1: ui1::UserInterface) {
    let start =
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();


    let mht_file_path =
        ui1.input_mht_path.value();
    let mht_file_path_buf =
        PathBuf::from(mht_file_path);

    let output_dir_path =
        ui1.input_output_dir_path.value();
    let output_dir_path_buf =
        PathBuf::from(output_dir_path);

    let thread_num =
        ui1.spinner_work_thread.value() as usize;

    let clean_up = ui1.checkbox_output_clean_tmp.clone().is_checked();

    handle_mht_file(
        mht_file_path_buf,
        output_dir_path_buf,
        thread_num,
        30000,
        clean_up,
    );

    let end =
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();

    let duration = Duration::milliseconds((end - start) as i64);
    println!("\nProgram done!\nstart:\t{:?},\nend:\t{:?},\nduration:{:?}", start, end, duration);
}

/*
    * 处理 mht 文件
    * handle_mht_file
    * mht_file_path: mht 路径
    * output_dir_path: 输出目录路径
    * thread_num: 工作线程数
    * lines_count_per_file: HTML数据行数
    * clean_up: 清理
*/
fn handle_mht_file(
    mht_file_path: PathBuf,
    output_dir_path: PathBuf,
    thread_num: usize,
    lines_count_per_file: usize,
    clean_up: bool,
) {
    program_output::print_line("=", 10);

    let mht_file_path_str = mht_file_path.to_str().unwrap();

    let parent = mht_file_path.parent().unwrap();
    let file_name = String::from(
        mht_file_path.file_name().unwrap().to_str().unwrap()
    );

    if mht_file_path.is_file() && !file_name.to_lowercase().ends_with(".mht") {
        println!("输入文件类型错误: {}", mht_file_path_str);
        return;
    }

    println!("开始处理文件:\n {}", mht_file_path_str);

    let index = file_name.rfind(".").unwrap();
    let file_name_1 = &file_name[0..index];
    // let file_name_2 = &file_name[index + 1..file_name.len()];

    // let work_dir = parent.join(file_name_1);
    // 输出目录就是工作目录
    let work_dir = output_dir_path;

    // println!("{:?}", work_dir);
    println!("输出目录: \n{}", work_dir.to_str().unwrap());

    path::create_if_missing(work_dir.to_str().unwrap())
        .expect("create workdir failed");

    // 输出的路径
    let html_file_path = work_dir.join(format!("{}.html", file_name_1));

    let img_relative_path = String::from("Data/img");

    let img_dir_path = work_dir.join(path::fix_path(img_relative_path.clone()));
    let tmp_dir_path = work_dir.join("tmp");

    path::create_if_missing(img_dir_path.to_str().unwrap())
        .expect("create failed");

    path::create_if_missing(tmp_dir_path.to_str().unwrap())
        .expect("create tmp_dir_path failed");

    println!("Thread pool count:{}", thread_num);

    program_output::print_line("-", 10);

    println!("开始预处理...");
    // 预处理原 mht 文件，中间文件输出至 tmp_dir_path 目录
    let mut total_count: usize;
    total_count = read_mht::parse_ori_mht(
        mht_file_path,
        html_file_path.clone(),
        tmp_dir_path.clone(),
        lines_count_per_file.clone(),
    );

    let conf_path = tmp_dir_path.clone().join("info.ini");
    if conf_path.clone().exists() {
        let conf =
            Ini::load_from_file(
                conf_path.clone()
            ).unwrap();
        let section = conf.section(Some("Pretreatment")).unwrap();
        let str_child_mht_count = section.get("child_mht_count").unwrap();
        let child_mht_count = str_child_mht_count.parse::<usize>().unwrap();
        total_count = child_mht_count;
    }

    read_mht::remove_tmp_files(
        tmp_dir_path.clone(),
        vec![
            ".ok",
            ".working",
        ],
    );

    if total_count.clone() == 0 {
        println!("预处理失败，程序退出。");
        return;
    }

    program_output::print_line("-", 10);

    println!("开始解码图片...");
    // 解码图片
    read_mht::start_parse_child(
        tmp_dir_path.clone(),
        img_dir_path.clone(),
        total_count,
        thread_num.clone(),
        0,
    );

    program_output::print_line("-", 10);

    // HTML图片替换，将解码的真实图片文件路径替换虚假的路径
    println!("开始替换HTML的图片路径...");
    read_mht::repair_html(
        html_file_path.clone(),
        img_dir_path,
        img_relative_path.clone(),
        1000,
    );

    program_output::print_line("-", 10);

    // std::fs::remove_dir_all(tmp_dir_path).expect("tmp directory remove failed!");
    // 移除临时文件
    if clean_up {
        read_mht::remove_tmp_files(tmp_dir_path, vec![]);
    }
}

// 程序退出程序
fn program_end(app: app::App) {
    // let ui1=app.
    // if ui1.checkbox_auto_save_cfg.clone().is_checked(){
    //     save_cfg(ui1);
    // }

    app.quit();
    // exit(0);
}