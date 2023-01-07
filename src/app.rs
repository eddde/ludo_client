
use crate::*;


pub struct Ludo {
    stream: TcpStream,
    order: i32,
    turn: i32,
}

impl Ludo {
    pub fn new(stream: TcpStream, order: i32, turn: i32) -> Self { 
        Self { stream, order , turn,}
    }
}

impl App for Ludo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx,  |ui| {
            let text = format!("Player {}", self.order);
            ui.heading(text);
            let turn_text = format!("Current turn: {}", self.turn.to_string());
            ui.label(turn_text);

            if ui.button("Change turn").clicked() {
                if self.order == self.turn {
                    send_command(&mut self.stream, "turn", self.order + 1)
                }
            }
        });
    }
}

fn send_command(stream: &mut TcpStream, command: &str, data: i32) {
    let comm = format!("{}|{}", command, data);
    stream.write(comm.as_bytes()).unwrap();
}

fn request_gamestate(stream: &mut TcpStream){
    send_command(stream, "gamestate", 0);
    //Todo get gamestate and update own state
}