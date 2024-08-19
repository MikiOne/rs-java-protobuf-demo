use std::env;
use std::path::PathBuf;

fn main() {
    // 获取当前工作目录
    let current_dir = env::current_dir().expect("Failed to get current directory");
    println!("Current Directory: {:?}", current_dir);

    // 获取上一级目录
    let parent_dir = current_dir.parent().expect("Failed to get parent directory");
    println!("Parent Directory: {:?}", parent_dir);

    let binding = parent_dir
        .join("../protos/student.proto");

    let protos = binding
        .as_path()
        .to_str()
        .expect("Failed to find protos directory");
    // 打印当前目录和上一级目录
    println!("protos Directory: {:?}", protos);
}