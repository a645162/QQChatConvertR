// build

extern crate winres;

use std::path::PathBuf;
use std::env;

fn main() {
    println!("开始执行 build.rs");
    let file_name = "ui1";
    println!("cargo:rerun-if-changed=src/{}.fl", file_name.clone());
    let generator = fl2rust::Generator::default();
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    generator.in_out(
        format!("ui/{}.fl", file_name.clone()).as_str(),
        out_path.join(
            format!("{}.rs", file_name.clone()).as_str()
        ).to_str().unwrap(),
    )
        .expect("Failed to generate rust from fl file!");

    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("res/logo.ico")
            .set("QQ消息记录MHT转换工具(FLTK-RS)", "QQMsgConvert_FLTK_RS.exe")
            // manually set version 1.0.0.0
            .set_version_info(winres::VersionInfo::PRODUCTVERSION, 0x0001000000000000);
        res.compile()
            .expect("RES编译失败！");
    }
    println!("结束执行 build.rs");
}