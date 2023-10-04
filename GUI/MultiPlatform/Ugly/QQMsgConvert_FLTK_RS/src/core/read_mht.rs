use std::collections::HashMap;

use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

use std::str::FromStr;
// use std::sync::{Arc, Barrier};
use std::thread;
use base64::Engine;
use base64::engine::general_purpose;
use threadpool::ThreadPool;
use crate::program_utils::path;
use walkdir::WalkDir;
use crate::program_utils::path::get_file_name_suffix;

extern crate ini;

// use crate::program_utils::machine_info;

use ini::Ini;

// const OPTIMIZED_NUMBER_OF_CORES: usize = 6;

const STR_DAT_END: &str = "}.dat";
// const LINES_COUNT_PER_FILE: i32 = 30000;

pub fn parse_ori_mht(
    mht_path: PathBuf,
    output_html_path: PathBuf,
    output_tmp_dir: PathBuf,
    lines_count_per_file: usize,
) -> usize {
    //以只读模式打开文件
    let file = File::open(mht_path).unwrap();

    path::remove_if_exist(path::path_buf2str(output_html_path.clone()));

    // 本身output_html_path的后戳已经是html，再加一个htm表示是临时文件！
    let tmp_html_path = format!("{}.htm", output_html_path.to_str().unwrap());
    let mut html_file = File::create(tmp_html_path)
        .expect("create tmp html failed");

    // let img_dictionary: HashMap<String, String> = HashMap::new();

    // while file.

    let lines =
        BufReader::new(file).lines();

    let mut html_code_end = false;
    let mut current_dat_file: File;

    let child_mht_filename_start = "MHT_CHILD";

    let mut child_mht_count = 1;
    let mut line_count = 0;

    let current_dat_path = output_tmp_dir.join(
        format!("{}.{}",
                child_mht_filename_start.clone(),
                child_mht_count.clone()
        )
    );
    let current_dat_path =
        current_dat_path.to_str().unwrap();

    path::remove_if_exist(String::from(current_dat_path.clone()));

    current_dat_file = File::create(current_dat_path.clone())
        .expect("create failed");

    //遍历所有行
    for line in lines {
        if let Ok(data) = line {
            let str_line = String::from(data.trim_end());

            if !html_code_end {
                if str_line.contains("<tr><td><div") {
                    // 检测到消息
                    // 图片文件名替换不在这一步进行了！
                } else if str_line.contains("</html>") {
                    // 检测到 HTML结束标志
                    // html_file.write_all(str_line.as_bytes())
                    //     .expect("writing HTML error!");
                    html_code_end = true;
                }
                // 写出到 HTML
                html_file.write_all(
                    format!("{}\n", str_line)
                        .as_bytes()
                )
                    .expect("writing HTML error!");
            } else {
                // HTML输出结束，该 BASE64 编码的二进制了

                let mut n = "\n";
                if line_count == lines_count_per_file.clone() - 1 {
                    n = "";
                }

                current_dat_file.write_all(
                    format!(
                        "{}{}", str_line, n
                    ).as_bytes())
                    .expect("write child failed");

                line_count += 1;

                if line_count == lines_count_per_file.clone() {
                    // 到指定行数换行
                    line_count = 0;

                    child_mht_count += 1;

                    let current_dat_path = output_tmp_dir.join(
                        format!("{}.{}",
                                child_mht_filename_start.clone(),
                                child_mht_count.clone()
                        )
                    );
                    let current_dat_path =
                        current_dat_path.to_str().unwrap();

                    path::remove_if_exist(String::from(current_dat_path.clone()));

                    current_dat_file = File::create(current_dat_path.clone())
                        .expect("create failed");
                }
            }
        }
    }

    let mut conf = Ini::new();
    conf.with_section(None::<String>)
        .set("encoding", "utf-8");
    conf.with_section(Some("Pretreatment"))
        .set("child_mht_count", child_mht_count.to_string());

    conf.write_to_file(output_tmp_dir.join("info.ini")).unwrap();

    println!("child_mht_count:{}", child_mht_count);

    child_mht_count
}


pub fn start_parse_child(
    output_tmp_dir: PathBuf,
    output_img_dir: PathBuf,
    total_count: usize,
    thread_count: usize,
    output_level: i32,
) {
    // 子文件解析 任务分配

    if thread_count < 1 {
        println!("Thread Count Set Error!");
        return;
    }

    let pool = ThreadPool::new(thread_count);

    let mut interval = total_count / thread_count;
    if interval < 1 {
        interval = 1;
    }

    let mut task_order = Vec::new();

    for i in 0..interval {
        let mut current_task_order = Vec::new();
        let mut j = 0;
        loop {
            let index = i + j + 1;
            task_order.push(index);
            current_task_order.push(index);

            j += interval;
            if i + j >= total_count {
                break;
            }
        }
        if output_level > 1 {
            println!("{:?}", current_task_order);
        }
    }
    if output_level > 0 {
        println!("Order:");
        println!("task_order={:?}", task_order);
        println!("size={}", task_order.len());
    }

    println!("开始分配任务");
    // let barrier = Arc::new(Barrier::new(total_count + 1));
    for i in 0..total_count {
        let index = *task_order.get(i).unwrap();
        if output_level > 1 {
            println!("{}", index);
        }

        let child_path = output_tmp_dir.join(
            format!("MHT_CHILD.{}", index.clone())
        );
        let output_tmp_dir = output_tmp_dir.clone();
        let output_img_dir = output_img_dir.clone();

        // let barrier = barrier.clone();
        pool.execute(move || {
            parse_child(
                index.clone(),
                String::from(child_path.clone().to_str().unwrap()),
                output_tmp_dir,
                output_img_dir,
                0,
            );
            // barrier.wait();
        });
    }
    // barrier.wait();

    while !(pool.active_count() == 0 && pool.queued_count() == 0) {
        let ten_millis = std::time::Duration::from_secs(1);
        thread::sleep(ten_millis);
    }

    println!(
        "active_count:{} queued_count:{}",
        pool.active_count(),
        pool.queued_count()
    )
}

pub fn parse_child(
    thread_id: usize,
    child_path: String,
    output_tmp_dir: PathBuf,
    output_img_dir: PathBuf,
    output_level: i32,
) {
    // // 判断文件是否正在处理
    // let opened_path = format!("{}.opened", child_path).as_str().to_string();
    // while Path::new(opened_path.clone().as_str()).exists() {
    //     if output_level > 0 {
    //         println!(
    //             "{} 已经打开，正在等待",
    //             child_path);
    //     }
    // }

    if Path::new(
        format!("{}.ok", child_path).as_str()
    ).exists() {
        if output_level > 0 {
            println!(
                "Thread_id{},{} 不存在任何数据的开头，无需继续！",
                thread_id, child_path);
        }
        return;
    }

    // 创建文件，防止重复打开！
    // File::create(opened_path.clone().as_str()).expect("Create tag failed!");

    // 子文件解析
    if output_level > 0 {
        println!("正在解析文件 {}", child_path.clone());
    }

    let file = File::open(child_path.clone()).unwrap();

    let lines =
        BufReader::new(file).lines();

    let img_path_start = output_img_dir.clone().to_str().unwrap().to_owned() + "/";

    let mut bl_begin = false;
    let mut bl_end = false;
    let mut found_text = false;

    let mut str_img_filename = String::from(
        output_tmp_dir.join(
            format!("thread_{}.working", thread_id)
        )
            .to_str().unwrap()
    );
    path::remove_if_exist(str_img_filename.clone());

    let mut str_suffix = String::from("");
    let mut current_file: File = File::create(str_img_filename.clone())
        .expect("create failed");

    for line in lines {
        if let Ok(data) = line {
            let str_line = String::from(data.trim_end());

            if str_line.len() == 0 {
                if bl_begin && found_text {
                    bl_begin = false;
                    bl_end = true;
                    found_text = false;
                }
            } else if str_line.contains("Content-Location:") {
                bl_begin = true;
                bl_end = false;
                str_img_filename = format!(
                    "{}{}.{}",
                    img_path_start,
                    &str_line[18..18 + 36],
                    str_suffix);

                path::remove_if_exist(str_img_filename.clone());
                current_file = File::create(str_img_filename.clone())
                    .expect("create failed");
                if output_level > 0 {
                    println!(
                        "Thread_id{},开始输出文件 {}",
                        thread_id, str_img_filename.clone());
                }
            } else if str_line.contains("Content-Type:image/") {
                // 获取文件类型(扩展名)
                str_suffix = str_line
                    .replace("Content-Type:image/", "")
                    .replace("jpeg", "jpg");
            } else if bl_begin {
                found_text = true;
                if !base64_decode(
                    &mut current_file,
                    str_line.clone(),
                ) {
                    println!(
                        "Thread_id{},文件 {} 中存在错误的 base64 编码！",
                        thread_id, child_path.clone());
                    println!(
                        "Thread_id{},图片文件 {} 还没有解码输出完整！",
                        thread_id, str_img_filename.clone());
                    println!(
                        "Thread_id{},当前行内容为: {}",
                        thread_id, str_line.clone());
                }
            }
        }
    }

    if !bl_begin {
        if output_level > 0 {
            println!(
                "Thread_id{},文件 {} 中没有任何 dat 的起始标识。",
                thread_id, child_path.clone());
        }
        // 这不是一个起始文件，做一个标记
        File::create(
            format!("{}.ok", child_path.clone())
        ).expect("Create tag failed!");
        return;
    }

    if !bl_end {
        // 本文件结束，但是还没有遇到空行，即还没有结束
        if output_level > 0 {
            println!(
                "Thread_id{},分段文件 {} 解析完毕，但是还有内容需要到下个文件继续寻找",
                thread_id, child_path.clone());
            println!(
                "Thread_id{},图片文件 {} 还没有解码输出完整！",
                thread_id, str_img_filename.clone());
        }
    }

    let mut next_path = child_path.clone();

    loop {
        // 继续解析
        next_path = get_next_file_path(next_path);

        // println!("在 {} 中继续！", next_path.clone());

        let next_file = File::open(next_path.clone()).unwrap();

        let this_lines =
            BufReader::new(next_file).lines();

        for line in this_lines {
            if let Ok(data) = line {
                let str_line = String::from(data.trim_end());

                if str_line.len() == 0 ||
                    str_line.contains("------=_")
                {
                    bl_end = true;
                    break;
                } else {
                    if !base64_decode(
                        &mut current_file,
                        str_line.clone(),
                    ) {
                        println!(
                            "Thread_id{},文件 {} 中存在错误的 base64 编码！",
                            thread_id, next_path.clone());
                        println!(
                            "Thread_id{},图片文件 {} 还没有解码输出完整！",
                            thread_id, str_img_filename.clone());
                        println!(
                            "Thread_id{},起始文件为 {} ！",
                            thread_id, child_path.clone());
                        println!(
                            "Thread_id{},当前行内容为: {}",
                            thread_id, str_line.clone());
                    }
                }
            }
        }


        if bl_end {
            if output_level > 0 {
                println!(
                    "Thread_id{},{}文件输出结束，最后解析的文件为{}",
                    thread_id.clone(), str_img_filename.clone(), next_path.clone());
            }
            break;
        } else {
            // 这不是一个起始文件，做一个标记
            File::create(
                format!("{}.ok", next_path.clone())
            ).expect("Create tag failed!");
        }
    }

    // 程序结束，删除标识
    // std::fs::remove_file(opened_path.clone().as_str()).expect("could not remove file");
}

fn get_next_file_path(path: String) -> String {
    let index = path.rfind(".").unwrap();

    let path1 = String::from(&path.clone()[0..index]);
    let path2 = String::from(&path.clone()[index + 1..path.clone().len()]);

    let current_num: i32 = FromStr::from_str(path2.as_str()).unwrap();

    format!("{}.{}", path1, current_num + 1)
}

fn base64_decode(file: &mut File, base64_str: String) -> bool {
    let mut buffer = Vec::<u8>::new();

    match general_purpose::STANDARD
        .decode_vec(base64_str.clone(), &mut buffer) {
        Ok(_) => {}
        Err(e) => {
            println!("base64 error:{}", e);
            return false;
        }
    }

    // general_purpose::STANDARD
    //     .decode_vec(base64_str, &mut buffer).expect("base64 error");
    // println!("{:?}", buffer);

    file.write_all(&*buffer).expect("write failed");

    buffer.clear();

    return true;
}

pub fn remove_tmp_files(tmp_dir: PathBuf, keywords: Vec<&str>) {
    let dir_path = tmp_dir.to_str().unwrap();

    for entry in WalkDir::new(
        dir_path).into_iter().filter_map(|e| e.ok()
    ) {
        let path = entry.path().display().to_string();

        let mut found = false || keywords.clone().len() == 0;
        for k in keywords.clone() {
            if path.contains(k) {
                found = true;
                break;
            }
        }

        if !found {
            continue;
        }

        let path_buf = PathBuf::from(path.clone());

        if Path::new(path.clone().as_str()).exists() && path_buf.is_file() {
            std::fs::remove_file(path.clone()).expect("could not remove file");
        }
    }
}

pub fn repair_html(
    output_html_path: PathBuf,
    output_img_dir: PathBuf,
    img_relative_path: String,
    html_max_item_size: i32,
) {
    let img_dir_path = output_img_dir.to_str().unwrap();

    let mut img_path_list: HashMap<String, String> = HashMap::new();
    println!("开始遍历目录 {}", img_dir_path);
    for entry in WalkDir::new(
        img_dir_path).into_iter().filter_map(|e| e.ok()
    ) {
        let path = entry.path().display().to_string();
        // println!("{}", path);
        let path_buf = PathBuf::from(path);

        if !path_buf.is_file() {
            continue;
        }

        let file_name = String::from(
            path_buf.file_name().unwrap().to_str().unwrap()
        );

        // if file_name.to_lowercase().ends_with(".png") {
        //     // PNG 就不用替换了.
        //     continue;
        // }

        let file_name_suffix = get_file_name_suffix(file_name);

        if file_name_suffix.1.len() > 0 {
            // println!("* {} = {}", file_name_suffix.0, file_name_suffix.1);
            img_path_list.insert(file_name_suffix.0, file_name_suffix.1);
        }
    }
    println!("已经完成遍历！找到{}个文件。", img_path_list.len());

    // let guid_list = img_path_list.keys();

    let tmp_html_path = format!("{}.htm", output_html_path.to_str().unwrap());

    // 模板相关
    let file = File::open("template.html").unwrap();
    let template_lines =
        BufReader::new(file).lines();

    let mut html_start_text: String = String::new();
    let mut html_end_text: String = String::new();

    let mut found_insert_point = false;
    for line in template_lines {
        if let Ok(data) = line {
            if !found_insert_point {
                // 找到关键词后跳过关键词
                let keyword = "{add_text}";
                if data.contains(keyword) {
                    found_insert_point = true;
                    let index = data.find(keyword).unwrap();
                    html_start_text = format!(
                        "{}\n{}", html_start_text,
                        String::from(&data[0..index])
                    );
                    html_end_text = format!(
                        "{}\n{}", html_end_text,
                        String::from(&data[index + keyword.len()..data.len()])
                    );
                } else {
                    html_start_text = format!("{}\n{}", html_start_text, data);
                }
            } else {
                html_end_text = format!("{}\n{}", html_end_text, data);
            }
        }
    }

    // 清除多余的空格
    html_start_text = html_start_text.trim_start().to_string();
    html_end_text = format!("{}\n", html_end_text.trim_end());

    let mut html_start = false;
    let mut body_start = false;
    let mut count: u32 = 0;
    let mut html_file_num: u32 = 1;

    // 打开 tmp 文件
    let file = File::open(tmp_html_path).unwrap();
    let lines =
        BufReader::new(file).lines();

    // 初始 HTML 路径
    let mut current_html_path = output_html_path.clone();

    // 创建html文件
    path::remove_if_exist(String::from(current_html_path.clone().to_str().unwrap()));
    let mut html_file = File::create(current_html_path.clone())
        .expect("create final html failed");

    html_file.write_all(html_start_text.as_bytes()).expect("Write html failed!");

    let mut not_found_files_count = 0;

    //遍历tmp所有行
    for line in lines {
        if let Ok(data) = line {
            let mut str_line = String::from(data.trim());

            // 直接跳过空行
            if str_line.len() == 0 {
                continue;
            }

            if !html_start && str_line.contains("<html ") {
                html_start = true;
            }

            if html_start {

                // 图片路径替换
                if str_line.contains(STR_DAT_END) {
                    // 检测到dat数据(图片)
                    let index = str_line.find("{").unwrap();
                    let str_img_id = &str_line[index + 1..index + 1 + 36];
                    // println!("{}\n{}", str_img_id, str_line);

                    // 替换dat文件扩展名为真实扩展名
                    if img_path_list.contains_key(str_img_id) {
                        str_line =
                            str_line.replace(
                                STR_DAT_END,
                                &*format!(
                                    ".{}",
                                    img_path_list.get(&String::from(str_img_id))
                                        .unwrap()
                                ),
                            );
                        // 替换 src 开头
                        str_line = str_line.replace(
                            "src=\"{",
                            format!("src=\"{}/", img_relative_path).as_str(),
                        );
                    } else {
                        not_found_files_count += 1;
                        // println!("{}找不到", str_img_id);
                        // 默认认为是 png 得了！
                        // str_line = str_line
                        //     .replace(STR_DAT_END, ".png");
                    }
                }


                if str_line.contains("<table") {
                    // 找到 table 起始标识，输出前面的内容

                    let new_line = String::from(
                        &str_line[
                            str_line.find("<table").unwrap() + 6
                                ..
                                str_line.len()
                            ]
                    ).trim().to_string();

                    if new_line.len() > 0 {
                        html_file.write_all(
                            format!("{}\n", new_line).as_bytes()
                        ).expect("write failed!");
                    }

                    body_start = true;
                } else if str_line.contains("</table>") {
                    // 找到 table 结束标识，输出前面的内容
                    let new_line = String::from(
                        &str_line[
                            0
                                ..
                                str_line.find("</table>").unwrap()
                            ]
                    ).trim().to_string();

                    if new_line.len() > 0 {
                        html_file.write_all(
                            format!("\t\t{}\n", new_line).as_bytes()
                        ).expect("write failed!");
                    }

                    break;
                } else {
                    // 正常写出中间行
                    if body_start {
                        html_file.write_all(
                            format!("\t\t{}\n", str_line).as_bytes()
                        ).expect("write failed!");
                    }
                }
            }

            if html_max_item_size >= 0 && str_line.contains("<tr><td") {
                // 有效消息计数
                count += 1;
            }

            if html_max_item_size >= 0 && count >= html_max_item_size as u32 {
                // 计数器归零
                count = 0;
                // 到指定数量，开始分文件！

                // 写出上一个 HTML 结尾
                html_file.write_all(html_end_text.as_bytes())
                    .expect("Write html failed!");
                // TODO:分下一个文件

                // 获取新的文件的文件名
                html_file_num += 1;
                let html_file_name =
                    current_html_path.clone().file_name().unwrap().to_str().unwrap().to_string();
                let index = html_file_name.rfind(".").unwrap();
                let mut new_name = String::from(&html_file_name[0..index]);
                if html_file_num > 2 {
                    let i = new_name.rfind(".").unwrap();
                    new_name = format!(
                        "{}.{}.html",
                        &new_name[0..i],
                        html_file_num
                    ).to_string();
                } else {
                    new_name = format!("{}.2.html", new_name)
                }

                current_html_path =
                    current_html_path.clone().parent().unwrap().to_path_buf()
                        .join(new_name);

                // 创建新的HTML文件
                path::remove_if_exist(String::from(
                    current_html_path.clone().to_str().unwrap()
                ));
                html_file = File::create(current_html_path.clone())
                    .expect("create final html failed");

                // 写出新文件的开头
                html_file.write_all(html_start_text.as_bytes())
                    .expect("Write html failed!");
            }
        }
    }

    html_file.write_all(html_end_text.as_bytes()).expect("Write html failed!");

    println!("总共有{}个资源丢失！", not_found_files_count);
}