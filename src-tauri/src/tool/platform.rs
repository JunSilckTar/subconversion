
use std::env;

/// 获取系统平台和体系结构信息
/// 返回值:
/// - `os`: 操作系统信息，如 `windows`, `macos`, `linux` 等
/// - `arch`: 体系结构信息，如 `x86_64`, `arm` 等
pub fn get_platform_with_arch() -> (&'static str, &'static str) {
    // 操作系统：windows, macos, linux
    let os = env::consts::OS;
    // 体系结构：x86_64, arm, etc.
    let arch = env::consts::ARCH;
    (os, arch)
}

