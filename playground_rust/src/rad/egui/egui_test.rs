#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{thread::{JoinHandle, self}, convert::TryInto};

use eframe::egui;
use egui::widgets;



struct Test {  
    ui_state: i32, // enums are useless since we cant track which page is which, also adding a new page might not be easy to implement with enums
    ui_list: Vec<String>,
    state_0: State0,
    state_1: State1
}



struct State0 {
    text: String,
}

impl Default for State0 {
    fn default() -> Self {
        Self {
            text: "".to_string(),
        }
    }
}
struct State1 {
    thread: Option<JoinHandle<String>>,
    length: i32,
    output: String,
}

impl Default for State1 {
    fn default() -> Self {
        Self {
            length: 0,
            output: "".to_string(),
            thread: None
        }
    }
}


pub fn init() {
    let options = eframe::NativeOptions::default();
    eframe::run_native("test", options, Box::new(|_cc| Box::new(Test::default())));
}

impl Default for Test {
    fn default() -> Self {
        Self {
            ui_state: 0,
            ui_list: ["code editor".to_string(), 
                        "shit shitshuffler".to_string(),
                        "empty".to_string()].to_vec(),
            state_0: State0::default(),
            state_1: State1::default(),
            // state_0: State0 { 
                // text: "".to_string()
            // }
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

                ui.label(
                    format!(
                        "{}/{}: {}",
                        self.ui_state + 1,
                        self.ui_list.len(),
                        self.ui_list.get(self.ui_state as usize).unwrap()
                    )
                        /*
                            note to future self:
                                {
                                    self.ui_list.get(self.ui_state.into()) 
                                    OR
                                    Into::<usize>::into(self.ui_state)
                                }
                            is apparently inferior to "as" when it comes to number types:
                                {
                                    self.ui_state as usize
                                }
                        */ 

                );
            });
            
            ui.separator();
            match self.ui_state {
                0 => {
                    
                    let mut _test = ui.code_editor(&mut self.state_0.text);
                    ui.label(format!("{} characters", self.state_0.text.len()));
                }

                1 => {
                    ui.heading("ShitShuffler Multithreading (shit doesnt work)");
                    
                    ui.add_space(1.5);
                    ui.horizontal(|ui| {
                        ui.label("length: ");
                        ui.add(egui::Slider::new(&mut self.state_1.length, 1..=20));
                    });

                    match &mut self.state_1.thread {
                        Some(_) => {
                            if self.state_1.thread.as_ref().unwrap().is_finished() {
                                self.state_1.output = format!("{:?}", self.state_1.thread.as_ref().unwrap());
                            }
                        },
                        None => {
                            if ui.button("submit").clicked() {
                                let len = self.state_1.length.to_owned();
                                self.state_1.thread = Some({
                                    thread::spawn(move || {
                                        super::super::shit_shuffler::run_singular_string(len)
                                    })
                                });
                            }
                        },
                    }

                    ui.label(format!("{}", self.state_1.output));
                    
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
