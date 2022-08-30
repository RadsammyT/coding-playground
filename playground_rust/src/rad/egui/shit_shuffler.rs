
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::thread;

use eframe::egui;
use egui::Vec2;

use super::super::shit_shuffler::Returned; // :)

pub fn init() {
    let options = eframe::NativeOptions {
        always_on_top: false,
        maximized: false,
        decorated: true,
        drag_and_drop_support: true,
        icon_data: None,
        initial_window_pos: None,
         initial_window_size: Option::from(Vec2::new(210 as f32, 100 as f32)),
        min_window_size: None,
        max_window_size: None,
        resizable: false,
        transparent: true,
        vsync: true,
        multisampling: 0,
        depth_buffer: 0,
        stencil_buffer: 0,
        fullscreen: false,
        hardware_acceleration: eframe::HardwareAcceleration::Preferred,
        renderer: eframe::Renderer::Glow,
        follow_system_theme: false,
        default_theme: eframe::Theme::Dark,
        run_and_return: true,

    };
    eframe::run_native(
        "shitshuffler",
        options,
        Box::new(|_cc| Box::new(ShitShuffler::default())),
    );
}

struct ShitShuffler {
    vector: Returned,
    length: u32,
}

impl Default for ShitShuffler {
    fn default() -> Self {
        Self {
            vector: Returned {ret_1: [0].to_vec(), ret_2: 0,},
            length: 12,
        }
    }
}

impl eframe::App for ShitShuffler {
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
            ui.heading("shitshuffler");
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
                    
                    self.vector = super::super::shit_shuffler::run_singular(self.length.try_into().unwrap());
                }
            });
            // ui.label(format!("{:?}, {}", self.vector.ret_1, self.vector.ret_2));
            ui.small(format!("{:?}, {}", self.vector.ret_1, self.vector.ret_2));
        });
    }
}