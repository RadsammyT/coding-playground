#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{thread::{JoinHandle, self}, convert::TryInto};

use eframe::egui;



struct Test {  
    ui_state: i32, // enums are useless since we cant track which page is in what order, also adding a new page might not be easy to implement with enums
    ui_list: Vec<String>,
    menu_bar: MenuBar, // added separate stucts for the pages cuz its more clean
    state_0: State0,
    state_1: State1
}

/*
    in hindsight:
    i could have set up state structs in one single enum like this:
    enum State {
        One {
            text: String,
        }
        ... and so on
    }
    that way I could somewhat put all of them in one enumerator instead of just plain spread out
*/

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

struct MenuBar {
    boolean: bool,
    number: i32,
}

impl Default for MenuBar {
    fn default() -> Self {
        Self {
            boolean: false,
            number: 0,
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
                        "something".to_string()].to_vec(),
            state_0: State0::default(),
            state_1: State1::default(),
            menu_bar: MenuBar::default(),
            // state_0: State0 { 
                // text: "".to_string()
            // }
        }
    }
}


impl eframe::App for Test {
    
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

                egui::menu::bar(ui, |ui| { // top bar
                    

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

                    ui.hyperlink_to("my git", "https://github.com/RadsammyT/coding-playground");
                    ui.menu_button("Test", |ui| {
                        ui.add(egui::widgets::DragValue::new(&mut self.menu_bar.number));
                        ui.checkbox(&mut self.menu_bar.boolean, "Test");
                        if ui.button("CLOSE").clicked() {
                            ui.close_menu();
                        }
                    });
                }); // end of top bar
                
            ui.separator();
            match self.ui_state {
                0 => {
                    
                    let mut _test = ui.code_editor(&mut self.state_0.text)
                        .on_hover_ui_at_pointer(|ui| {
                            ui.heading("spooky");
                        });
                    ui.label(format!("{} characters", self.state_0.text.len()));
                }

                1 => {
                    ui.heading("ShitShuffler Multithreading (shit doesnt work)");
                    ui.add_space(30.0);
                    ui.code("can't seem to get this to work in the some(_) arm in the match statement of the thread, setting the output to the joined thread in the process: \ncannot move out of a shared reference \nmove occurs because value has type `JoinHandle<String>`, which does not implement the `Copy` traitrustcE05 \n\nif you know how to fix this plz do a PR!");
                    ui.add_space(30.0);
                    ui.horizontal(|ui| {
                        ui.label("length: ")
                            .on_hover_text_at_pointer("if you are NOT running this on --release, then set to lower lengths, generally <= 10");
                        ui.add(egui::Slider::new(&mut self.state_1.length, 1..=20))
                            .on_hover_text_at_pointer("if you are NOT running this on --release, then set to lower lengths, generally <= 10");
                            
                    });
                    
                    match &mut self.state_1.thread {
                        Some(_) => {
                            if self.state_1.thread.as_ref().unwrap().is_finished() {
                                self.state_1.output = format!("{:?}", self.state_1.thread.as_ref().unwrap()); //<-- cant join the thread without it erroring here
                                // dont plan on fixing this because it will cause me a headache and a half
                            }
                            if ui.button("Retry").clicked() {
                                self.state_1.thread = None;
                                self.state_1.output = String::new();
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
                }
                _ => {
                    ui.label(format!("uh oh, state is {} when the following states are {:?}", self.ui_state, self.ui_list));
                }
            }
        });
    }
}
