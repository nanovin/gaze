// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use screenshot::init_screenshot_worker;
use search::init_search;

mod screenshot;
mod search;

fn main() {
    init_screenshot_worker();
    init_search();
}
