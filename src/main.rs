#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
use app::*;

use std::io::{Read, Write};
use std::net::TcpStream;

use eframe::{run_native, App};
use eframe::egui::*;


// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    
    let order;

    let mut stream = TcpStream::connect("188.149.142.50:6666").unwrap();


    //connect to the server
    let data = b"connect";
    let data_str = String::from_utf8_lossy(data);
    stream.write(data).unwrap();
    println!("Client sent {} bytes: {}", data.len(), data_str);
    let response = read_i32(&mut stream);
    println!("Client order {}", response);
    order = response;
    let response = read_i32(&mut stream);
    println!("Client recieved current turn {}", response);
    let current_turn = response;

    let app = Ludo::new(stream, order, current_turn);
    let options = eframe::NativeOptions {
        //icon_data: Some(load_icon("assets/favicon.png")),
        ..Default::default()
    };
    run_native("Ludo",options,Box::new(|_cc| Box::new(app)));
}

fn read_i32(stream: &mut impl Read) -> i32 {
    let mut bytes = [0; 4];  // create a buffer for the bytes
    stream.read(&mut bytes).unwrap();  // read 4 bytes from the stream
    i32::from_be_bytes(bytes) // convert the bytes to an i32 value
}

fn load_icon(path: &str) -> eframe::IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    eframe::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}