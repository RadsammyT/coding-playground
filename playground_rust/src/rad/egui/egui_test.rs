#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{thread::{JoinHandle, self}, convert::TryInto};
use super::super::timer;
use eframe::egui;
use egui::{FontDefinitions, FontFamily, ColorImage, Context};
use image;


struct Files {
    state_2_image: String,
}

impl Files {
    fn default() -> Self {
        Self {
            state_2_image: "E:/CODING WORKSPACE/coding-playground/playground_rust/src/rad/egui/assets/t6LxQ0dfin.jpg".to_string(),
        }
    }
}

struct Main {  
    ui_state: i32, // enums are useless since we cant track which page is in what order, also adding a new page might not be easy to implement with enums
    ui_list: Vec<String>,
    menu_bar: MenuBar, // added separate stucts for the pages cuz its more clean
    state_0: State0,
    state_1: State1,
    state_2: State2,
    files: Files
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
    timer: timer::Timer,
    finish: bool,
}

impl Default for State1 {
    fn default() -> Self {
        Self {
            length: 0,
            output: "".to_string(),
            thread: None,
            timer: timer::Timer::default(),
            finish: false,
        }
    }
}

struct State2 {
    handle: Option<egui::TextureHandle>,
    texture: Option<egui::ColorImage>,
}

impl Default for State2 {
    fn default() -> Self {
        Self {
            handle: None,
            texture: None,
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
    eframe::run_native("Main", options, Box::new(|_cc| Box::new(Main::default(_cc))));
}

impl Main {
    fn default(cc: &eframe::CreationContext<'_>) -> Self {
        setup(&cc.egui_ctx);
        let mut ret = Self {
            ui_state: 0,
            ui_list: ["code editor".to_string(), 
                        "shit shitshuffler".to_string(),
                        "something".to_string()].to_vec(),
            state_0: State0::default(),
            state_1: State1::default(),
            state_2: State2::default(),
            menu_bar: MenuBar::default(),
            files: Files::default(),
            // state_0: State0 { 
                // text: "".to_string()
            // }
        };

        ret.state_2.texture = Some(image_load(std::path::Path::new(&ret.files.state_2_image), 3).unwrap_or({
            ColorImage::default()
        }));
        
        return ret;
    }
}

fn setup(c: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert("font".to_owned(), 
    egui::FontData::from_static(include_bytes!("../assets/PZIM3X5.TTF")),    
    );

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "font".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("font".to_owned());


    c.set_fonts(fonts);
}

fn image_load(p: &std::path::Path, scale: i32) -> Result<egui::ColorImage, image::ImageError> {
    if scale < 1 {
        panic!("Scale is {} but must NOT be less than 1!", scale);
    }
    let image = image::io::Reader::open(p)?.decode()?;
    let new = image.resize_to_fill(image.width()/scale as u32, image.height()/scale as u32, image::imageops::FilterType::Nearest);
    let size = [new.width() as _, new.height() as _];
    let new_buffer = new.to_rgba8();
    let pixels = new_buffer.as_flat_samples();
    return Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ));

}

impl eframe::App for Main {

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
                    ui.menu_button("Main", |ui| {
                        ui.add(egui::widgets::DragValue::new(&mut self.menu_bar.number));
                        ui.checkbox(&mut self.menu_bar.boolean, "Main");
                        if ui.button("CLOSE").clicked() {
                            ui.close_menu();
                        }
                    });
                }); // end of top bar
                
            ui.separator();
            match self.ui_state {
                0 => {
                    
                    let mut _main = ui.code_editor(&mut self.state_0.text)
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
                                if !(self.state_1.finish) {
                                    self.state_1.timer.end_timer();
                                    self.state_1.finish = true;
                                }

                                let test = self.state_1.thread.as_ref().unwrap(); //<-- cant join the thread without it erroring here
                                self.state_1.output = format!("{:?} in {}", test, self.state_1.timer.get_elapsed().unwrap()); 

                                // dont plan on fixing this because it will cause me a headache and a half
                            }
                            if ui.button("Retry").clicked() {
                                self.state_1.thread = None;
                                self.state_1.output = String::new();
                                self.state_1.finish = false;
                            }
                        },
                        None => {
                            if ui.button("submit").clicked() {
                                let len = self.state_1.length.to_owned();
                                self.state_1.timer.start_timer();
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

                        let text = self.state_2.texture.to_owned().unwrap();
                        let handle: &egui::TextureHandle = self.state_2.handle.get_or_insert_with(|| {
                            ui.ctx().load_texture("test", text, egui::TextureFilter::Linear)
                        });
                        ui.image(handle, handle.size_vec2());
                        ui.label("If image is the entire color palette then something has gone wrong.");
                }
                _ => {
                    ui.label(format!("uh oh, state is {} when the following states are {:?}", self.ui_state, self.ui_list));
                }
            }
        });
    }
}
