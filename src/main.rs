#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::io::{Read, Write};
use std::net::TcpStream;

use eframe::{run_native, App};
use eframe::NativeOptions;
use eframe::egui::*;
struct Ludo {
    stream: TcpStream,
}

impl Ludo {
    pub fn new(stream: TcpStream) -> Ludo { 
        Ludo { stream }
    }
}

impl App for Ludo {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx,  |ui| {
            ui.heading("Ludo");
            if ui.button("test").clicked() {
                self.stream.write(b"test").unwrap();
            }
        });
    }
}

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    
    let mut order;

    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();


    //connect to the server
    let data = b"connect";
    let data_str = String::from_utf8_lossy(data);
    stream.write(data).unwrap();
    println!("Client sent {} bytes: {}", data.len(), data_str);
    let response = read_i32(&mut stream);
    println!("Client recieved {}", response);
    order = &response;

    let app = Ludo::new(stream);
    let win_options = NativeOptions::default();
    run_native("Ludo",win_options,Box::new(|_cc| Box::new(app)));
}

fn read_i32(stream: &mut impl Read) -> i32 {
    let mut bytes = [0; 4];  // create a buffer for the bytes
    stream.read(&mut bytes).unwrap();  // read 4 bytes from the stream
    i32::from_be_bytes(bytes) // convert the bytes to an i32 value
}

