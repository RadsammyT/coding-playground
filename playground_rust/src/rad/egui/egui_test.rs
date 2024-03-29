#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{thread::{JoinHandle, self}, convert::TryInto, vec};
use crate::rad::{timer::Timer, shit_shuffler};

use super::super::timer;
use eframe::egui;
use egui::{ Color32,plot::BarChart, Stroke, Pos2};
use egui_extras::RetainedImage;

const CALIBRATION_LENGTH: i32 = 12;
const CALIBRATION_AVERAGE_LEN: i32 = 10;
const USE_PIXELZIM_FONT: bool = false;
// struct Files {
//     state_2_image: Option<RetainedImage>,
//     state_0_image: Option<RetainedImage>,
// }

// impl Files {
//     fn default() -> Self {
//         Self {
//             state_2_image: Some(RetainedImage::from_image_bytes("two", include_bytes!("../../../assets/t6LxQ0dfin.jpg")).unwrap()),
//             state_0_image: Some(RetainedImage::from_image_bytes("two", include_bytes!("../../../assets/jig.jpg")).unwrap()),
//         }
//     }
// }

struct Main {  
    ui_state: i32, // enums are useless since we cant track which page is in what order, also adding a new page might not be easy to implement with enums
    ui_list: Vec<String>,
    menu_bar: MenuBar, // added separate stucts for the pages cuz its more clean
    state_0: State0,
    state_1: State1,
    state_2: State2,
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
    chart: Vec<egui::plot::Bar>,
    image: Option<RetainedImage>,
}

impl Default for State0 {
    fn default() -> Self {
        use egui::plot::Bar as bar;
        
        let newchart = vec![bar::new(0.0, 10.10), bar::new(1.0, 4.3), bar::new(2.0, 3.4)];
        Self {
            text: "0 = \n1 = \n2 = ".to_string(),
            chart: newchart,

            image: Some(RetainedImage::from_image_bytes("two", include_bytes!("../../../assets/jig.jpg")).unwrap()),
        }
    }
}
struct State1 { // shit shuffler
    thread: Option<JoinHandle<String>>,
    length: i32,
    output: String,
    timer: timer::Timer,
    finish: bool,
    fail_calib: f64, 
    /*
    figured that since i cant access the fails I could probably approximate the fails
    based on a single threaded shitshuffler, which means that id have to make the main program thread
    hang on shitshuffler instead of doing it in a separate thread 
    */
}

impl Default for State1 {
    fn default() -> Self {
        Self {
            length: 0,
            output: "".to_string(),
            thread: None,
            timer: timer::Timer::default(),
            finish: false,
            fail_calib: 0.0,
        }
    }
}

struct State2 {
    image: Option<RetainedImage>,
}

impl Default for State2 {
    fn default() -> Self {
        Self {
            image: Some(RetainedImage::from_image_bytes("two", include_bytes!("../../../assets/t6LxQ0dfin.jpg")).unwrap())
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
        
        if USE_PIXELZIM_FONT {
            setup(&cc.egui_ctx);
        }
        let ret = Self {
            ui_state: 0,
            ui_list: ["its jigsaw oh god oh fuck oh shit".to_string(), 
                        "shit shitshuffler".to_string(),
                        "something".to_string()].to_vec(),
            state_0: State0::default(),
            state_1: State1::default(),
            state_2: State2::default(),
            menu_bar: MenuBar::default(),
            // state_0: State0 { 
                // text: "".to_string()
            // }
        };

        


        return ret;
    }
}

fn setup(c: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert("font".to_owned(), 
    egui::FontData::from_static(include_bytes!("../../../assets/PZIM3X5.TTF")),    
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

impl eframe::App for Main {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::TopBottomPanel::top("god_on_top").show(ctx, |ui| {
            ui.horizontal(|ui| { 
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
                    ui.label(format!("0b{:b}", self.menu_bar.number));
                    ui.checkbox(&mut self.menu_bar.boolean, "Main");
                    if ui.button("CLOSE").clicked() {
                        ui.close_menu();
                    }
                });
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.ui_state {
                0 => {
                    use egui::plot::Plot;
                    let mut _main = ui.code_editor(&mut self.state_0.text)
                        .on_hover_ui_at_pointer(|ui| {
                            ui.heading("spooky");
                        });
                    ui.label(format!("{} characters", self.state_0.text.len()));
                    self.state_0.image.as_ref().unwrap().show_scaled(ui, 0.5);
                    ui.painter().circle(Pos2::new(0.0,0.0), 5.0, Color32::RED, Stroke::default());
                    ui.label("Hello. Before you is a chart of p*nis sizes that is measured, in inches, from 1 mother, 1 father, and 1 child. From these measurements, you will determine whose p*nis size belongs to which member of the family. Your answer must be inserted in the textbox at the top of this interface. If you do not answer the question, the gas will be released in 10 minutes.");


                    let plot_test = BarChart::new(self.state_0.chart.to_owned());
                    Plot::new("bartest")
                        .view_aspect(2.0)
                        .allow_zoom(true)
                        .show(ui, |pui| {
                            pui.bar_chart(plot_test)
                        });
                    
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
                                // dont plan on fixing this because it will cause me a headache and a half
                                self.state_1.output = format!("{:?} in {}", test, self.state_1.timer.get_elapsed().unwrap()); 
                                // ui.label(format!("Approximately {} fails",   self.state_1.fail_calib * self.state_1.timer.get_elapsed().unwrap()).as_str());
                                
                                ui.label(format!("{}", self.state_1.output));
                                ui.label(format!("Approximately {} fails", 
                                (self.state_1.fail_calib * self.state_1.timer
                                    .get_elapsed()
                                    .unwrap())
                                    .round() as i32)
                                    .as_str()); 
                                // so i said screw it and instead of doing nothing i decided to APPROXIMATE the fails instead based on a previous shitshuffle
                            }
                            if self.state_1.thread.as_ref().unwrap().is_finished() {
                                if ui.button("Retry").clicked() {
                                    self.state_1.thread = None;
                                    self.state_1.output = String::new();
                                    self.state_1.finish = false;
                                }
                            } else {
                                ui.label("Wait");
                            }
                        },
                        None => {
                            if self.state_1.fail_calib == 0.0 {
                                // ui.label("I will need to calibrate how fast your computer is by running shitshuffler on the same thread, which will hang this GUI. Calibration will take place on an CALIBRATION_LENGTH-sized array which won't take long (relatively speaking on a beefy computer) but I could approximate the fails based on that.");
                                if ui.button("Calibrate").clicked() {
                                    self.state_1.fail_calib = calibrate_fails();
                                }
                            } else {
                                ui.label(format!("Your machine does {} fails per second", self.state_1.fail_calib));
                                ui.add_space(20.0);
                                if ui.button("submit").clicked() {
                                    let len = self.state_1.length.to_owned();
                                    self.state_1.timer.start_timer();
                                    self.state_1.thread = Some({
                                        thread::spawn(move || {
                                            super::super::shit_shuffler::run_singular_string(len)
                                        })
                                    });
                                }
                                if ui.button("Recalibrate").clicked() {
                                    self.state_1.fail_calib = calibrate_fails();
                                }

                            }
                        },
                    }


                }

                2 => {
                    self.state_2.image.as_ref().unwrap().show_scaled(ui, 0.2);
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        egui::Grid::new("grid").striped(true).show(ui, |ui| {
                            ui.label("I'm KILLING you.");
                            ui.label("I'm KILLING YOU.");
                            ui.end_row();

                            ui.label("I Don't care about anything else- its just");
                            ui.label("GET THAT FUCKING GUY RIGHT NOW.");
                            ui.end_row();

                            ui.label("There is no: \"Oh hes running? ill back off!\" ");
                            ui.label("Nope, its just \" STOMP STOMP STOMP STOMP \"");

                            for i in 0..1000 {
                                ui.label(format!("{i}"));
                                ui.end_row();
                            }

                        });
                    });

                }
                _ => {
                    ui.label(format!("uh oh, state is {} when the following states are {:?}", self.ui_state, self.ui_list));
                }
            }
        });
    }
}

fn calibrate_fails() -> f64 {
    let mut fails: Vec<f64> = vec![];
    let mut clock = Timer::default();
    for _ in 0..CALIBRATION_AVERAGE_LEN {
        clock.start_timer();
        let temp = shit_shuffler::run_singular(CALIBRATION_LENGTH);
        clock.stop_timer();
        fails.push(temp.ret_2 as f64 /clock.get_elapsed().unwrap());
    }

    let mut result = 0.0;
    for i in &fails {
        result += *i;
    }
    return result / fails.len() as f64
}
