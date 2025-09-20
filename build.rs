fn main() {
    // let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    // 添加重新运行条件，当UI文件或构建脚本改变时重新运行
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=build.rs");
    eprintln!("Starting Slint compilation...");
    eprintln!("Compiling ui/main.slint...");

}
