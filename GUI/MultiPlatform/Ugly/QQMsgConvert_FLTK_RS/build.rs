// build
fn main() {
    use std::path::PathBuf;
    use std::env;
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
}