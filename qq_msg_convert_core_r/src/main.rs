mod utils;

mod read_mht;
mod path;
mod machine_info;

use std::cmp::{min};
use std::env;
use std::path::{Path, PathBuf};
use time::*;
use std::time::{SystemTime};

extern crate ini;

use ini::Ini;

// const MAX_CORES: usize = 12;
const CONFIG_PATH: &str = "config.ini";
// static mut MAX_CORES: usize = 12;

fn print_line(text: &str, length: usize) {
    for _ in 0..length {
        print!("{}", text);
    }
    println!();
}

fn main() {
    print_line("=", 10);
    println!("QQ聊天记录MHT文件解码工具");
    println!("作者: 孔昊旻");
    print_line("=", 10);

    let mut thread_num = 6;
    let mut lines_count_per_file = 30000;

    if Path::new(CONFIG_PATH).exists() {
        println!("配置文件存在，读取配置文件...");
        let conf = Ini::load_from_file(CONFIG_PATH).unwrap();

        let section_cpu = conf.section(Some("CPU")).unwrap();
        let str_max_core_number = section_cpu.get("MAX_CORE_NUMBER").unwrap();
        let max_cores = str_max_core_number.parse::<usize>().unwrap();
        println!("最大线程数:{}", max_cores);

        // 获取推荐的线程数
        let suggest_thread_num = machine_info::get_thread_nums();

        if max_cores <= 0 {
            println!("最大线程数不能小于等于0，将使用推荐配置");
            thread_num = suggest_thread_num;
        } else {
            let use_recommend =
                section_cpu.get("USE_RECOMMEND_CORE_NUM").unwrap().trim() == "1";

            if use_recommend {
                println!("程序推荐的线程数:{}", suggest_thread_num);
                thread_num = min(suggest_thread_num, max_cores);
            }
        }


        let section_output = conf.section(Some("Output")).unwrap();
        let str_lines_count_per_file = section_output.get("LINES_COUNT_PER_FILE").unwrap();
        let conf_lines_count_per_file = str_lines_count_per_file.parse::<usize>().unwrap();
        lines_count_per_file = conf_lines_count_per_file;
    } else {
        println!("配置文件不存在，使用默认配置。")
    }

    println!("当前系统CPU核心数:{}", machine_info::get_cpu_core_nums());
    println!("最终使用的线程数:{}", thread_num);

    print_line("-", 10);

    let start =
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();

    // let current_path =
    //     env::current_dir().unwrap();

    let mut mht_list: Vec<PathBuf> = Vec::new();

    // 读取环境变量
    let args: Vec<String> = env::args().collect();
    for i in 1..args.len() {
        let arg = (args[i]).trim();

        if arg.to_lowercase().ends_with(".mht") {
            let mht_file_path = PathBuf::from(arg);
            if mht_file_path.is_file() {
                mht_list.push(mht_file_path);
                println!("检测到mht文件 {}", arg);
            } else {
                println!("{:?} 不是一个有效的文件。", arg);
            }
        } else {
            println!("{:?} 不是一个有效的mht文件。", arg);
        }
    }

    println!("有效mht文件数量:{}", mht_list.len());

    // 对每个 mht 文件执行操作
    for mht_file_path in mht_list {
        handle_mht_file(
            mht_file_path,
            thread_num,
            lines_count_per_file.clone(),
        );
    }

    let end =
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();

    let duration = Duration::milliseconds((end - start) as i64);
    println!("\nProgram done!\nstart:\t{:?},\nend:\t{:?},\nduration:{:?}", start, end, duration);
}

fn handle_mht_file(
    mht_file_path: PathBuf,
    thread_num: usize,
    lines_count_per_file: usize,
) {
    print_line("=", 10);

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

    let work_dir = parent.join(file_name_1);

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

    println!("max work thread:{}", thread_num);

    print_line("-", 10);

    println!("开始预处理...");
    // 预处理
    let mut total_count: usize = 0;
    total_count = read_mht::parse_ori_mht(
        mht_file_path,
        html_file_path.clone(),
        tmp_dir_path.clone(),
        lines_count_per_file.clone(),
    );

    // let conf_path = tmp_dir_path.clone().join("info.ini");
    // if conf_path.clone().exists() {
    //     let conf =
    //         Ini::load_from_file(
    //             conf_path.clone()
    //         ).unwrap();
    //     let section = conf.section(Some("Pretreatment")).unwrap();
    //     let str_child_mht_count = section.get("child_mht_count").unwrap();
    //     let child_mht_count = str_child_mht_count.parse::<usize>().unwrap();
    //     total_count = child_mht_count;
    // }
    //
    // read_mht::remove_tmp_files(
    //     tmp_dir_path.clone(),
    //     vec![
    //         ".ok",
    //         ".working"
    //     ]
    // );

    if total_count.clone() == 0 {
        println!("预处理失败，程序退出。");
        return;
    }

    print_line("-", 10);

    println!("开始解码图片...");
    // 解码图片
    read_mht::start_parse_child(
        tmp_dir_path.clone(),
        img_dir_path.clone(),
        total_count,
        thread_num.clone(),
        0,
    );

    print_line("-", 10);

    println!("开始替换HTML的图片路径...");
    read_mht::repair_html(
        html_file_path.clone(),
        img_dir_path,
        img_relative_path.clone(),
        1000,
    );

    print_line("-", 10);

    // std::fs::remove_dir_all(tmp_dir_path).expect("tmp directory remove failed!");
    read_mht::remove_tmp_files(tmp_dir_path, vec![]);
}