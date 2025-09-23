use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // 如果是 Windows 平台，则处理 libpq.dll
    if cfg!(target_os = "windows") {
        let target_dir = env::var("OUT_DIR").unwrap();
        let target_dir = Path::new(&target_dir).parent().unwrap().parent().unwrap().parent().unwrap();
        
        // 获取 libpq.dll 路径
        if let Ok(pq_lib_dir) = env::var("PQ_LIB_DIR") {
            let lib_path = Path::new(&pq_lib_dir).join("libpq.dll");
            if lib_path.exists() {
                let debug_target = target_dir.join("debug").join("libpq.dll");
                let release_target = target_dir.join("release").join("libpq.dll");
                
                // 复制到 debug 和 release 目录
                if let Some(parent) = debug_target.parent() {
                    if !parent.exists() {
                        let _ = fs::create_dir_all(parent);
                    }
                }
                let _ = fs::copy(&lib_path, &debug_target);
                
                if let Some(parent) = release_target.parent() {
                    if !parent.exists() {
                        let _ = fs::create_dir_all(parent);
                    }
                }
                let _ = fs::copy(&lib_path, &release_target);
            }
        }
    }
    
    // 通知 Cargo 当环境变量变化时重新运行此脚本
    println!("cargo:rerun-if-env-changed=PQ_LIB_DIR");
}