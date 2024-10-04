#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Hide console window on Windows in release

use crate::app::MyApp;

mod app;
mod web;
mod file_reader;

#[cfg(target_arch = "wasm32")]
use eframe::web_sys;
// #[cfg(target_arch = "wasm32")]
// use wasm_bindgen::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Run natively
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My Egui Web App",
        native_options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    ).unwrap();
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();
}