
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::thread;

use eframe::egui;

use super::shit_shuffler::Returned;

pub fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    vector: Returned,
    length: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            vector: Returned {ret_1: [0].to_vec(), ret_2: 0,},
            length: 12,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // ui.heading("My egui Application");
            // ui.horizontal(|ui| {
            //     ui.label("Your name: ");
            //     ui.text_edit_singleline(&mut self.name);
            // });
            // ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));

            // if ui.button("Click each year").clicked() {
            //     self.age += 1;
            // }
            // ui.label(format!("Hello '{}', age {}", self.name, self.age));
            ui.heading("shitshuffler?");
            ui.horizontal(|ui| {
                ui.label("length: ");
                ui.add(egui::Slider::new(&mut self.length, 1..=20));
            });
            
            ui.vertical(|ui| {
                if ui.button("Roll").clicked() {
                    // let len = self.length.to_owned(); //WAIT THAT WORKS?!
                    // self.vector = thread::spawn(move || {
                    //     return super::shit_shuffler::run_singular(len.try_into().unwrap());
                    // }).join().unwrap();
                    // it doesnt.. kinda. i wanted this to be the solution so that the app doesnt hang when rolling
                    
                    self.vector = super::shit_shuffler::run_singular(self.length.try_into().unwrap());
                }
            });
            ui.label(format!("{:?}, {}", self.vector.ret_1, self.vector.ret_2));
        });
    }
}