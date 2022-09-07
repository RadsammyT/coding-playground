#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

struct Test {  
    ui_state: i16, // i COULD use an enum for this but NAAAAAH
    ui_list: [i16; 3],
    state_0_text: String,
}

pub fn init() {
    let options = eframe::NativeOptions::default();
    eframe::run_native("test", options, Box::new(|_cc| Box::new(Test::default())));
}

impl Default for Test {
    fn default() -> Self {
        Self {
            ui_state: 0,
            ui_list: [0, 1, 2],
            state_0_text: "".to_string(),
        }
    }
}


impl eframe::App for Test {
    
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.horizontal(|ui| { // much better
                if ui.button("prev").clicked() {
                    if !(self.ui_state == 0) {
                        // println!("prev");
                        self.ui_state -= 1;
                    }
                }

                if ui.button("next").clicked() {
                    if !(self.ui_state == (self.ui_list.len() - 1).try_into().unwrap()) {
                        // println!("next");
                        self.ui_state += 1;
                    }
                }

                ui.label(format!("{}/{}", self.ui_state + 1, self.ui_list.len()));
            });

            match self.ui_state {
                0 => {
                    
                    let mut _test = ui.code_editor(&mut self.state_0_text);
                }

                1 => {
                    ui.label("state 1");
                }

                2 => {
                    ui.label("state 2");
                }
                _ => {
                    ui.label(format!("uh oh, state is {} when the following states are {:?}", self.ui_state, self.ui_list));
                }
            }
        });
    }
}
