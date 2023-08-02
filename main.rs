use eframe::egui;   

pub mod model;
pub mod gui;
pub mod app;

use app::App;


fn main () -> Result<(), eframe::Error> {

    std::fs::read("/usr/bin/zsh").expect("zsh can be opened");
    println!("zsh accessible");

    dbg!(std::process::Command::new("/usr/bin/zsh")
        .arg("-c")
        .arg("echo hello")
        .output()
        .expect("failed to execute process"));

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Counter",
        options,
        Box::new(App::setup),
    )
}

