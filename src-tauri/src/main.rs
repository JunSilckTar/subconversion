// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod build;
mod init;
mod tool;
mod enums;
mod database;

use init::init_app;

fn main() {
    if init_app::init() { println!("init successful!") }
    // build_trigger::build();
}



