// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod build;
mod tool;
use build::build_trigger;
use tool::platform;

fn main() {
    build_trigger::build();
    let (os, arch) = platform::get_platform_with_arch();
    println!("Operating System: {}, Architecture: {}", os, arch);
}



