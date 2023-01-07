#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, run_native};

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    
}

