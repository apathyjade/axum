use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordVerifier, SaltString},
    Argon2,
    PasswordHasher,
};
use std::error::Error;

// --- 2. 密码哈希和验证的核心逻辑 ---
// 为了简化代码，我们可以定义一个配置结构体
// const ITERATIONS: u32 = 2; // 迭代次数，越慢越安全，但会消耗更多 CPU
// const MEMORY_KB: u32 = 19 * 1024; // 内存使用量 (KB)
// const PARALLELISM: u8 = 1; // 并行线程数
// const SALT_LEN: usize = 16; // 盐值长度
// const HASH_LEN: usize = 32; // 哈希输出长度

/// 使用 Argon2 对密码进行哈希处理。
/// 这个函数将返回一个完整的、包含盐值的哈希字符串。
pub fn hash_password<'a>(password: &'a str) -> Result<String, Box<dyn Error>> {
    // 1. 生成安全的随机盐值
    let salt = SaltString::generate(&mut OsRng);
    let bytes_pw = password.as_bytes();
    let argon2 = Argon2::default();
    
    let password_hash = argon2
        .hash_password(bytes_pw, &salt)?
        .to_string(); // 将哈希转换为标准字符串格式
    Ok(password_hash)
}
/// 使用 Argon2 验证密码。
#[allow(dead_code)]
pub fn verify_password(
    password: &[u8],
    password_hash_str: &str, // 从数据库中取出的哈希字符串
) -> Result<(), Box<dyn Error>> {
    let argon2 = Argon2::default();
    // 从字符串解析出 PasswordHash 对象
    let parsed_hash = PasswordHash::new(password_hash_str)
        .map_err(|e| format!("无效的密码哈希格式: {}", e))?;
    // 使用 Argon2 的 verify 方法进行验证
    argon2.verify_password(password, &parsed_hash)?;
    Ok(())
}