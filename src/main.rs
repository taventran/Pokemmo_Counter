mod pokemon_struct; // Use module code
mod save_data;
use crate::pokemon_struct::Pokemon; // Get the struct I want
mod calibrate;
use crate::calibrate::get_press;
mod tracker;
use crate::tracker::tracker;
mod read_data;
use crate::read_data::read_data;
use fltk::{app, button, enums::Color, frame::Frame, prelude::*, window::Window};
use std::thread;

fn main() {
    // Initialize a pokemon
    let starting_num = read_data();
    let name: &str = "Magikarp";
    let encounters: u32 = starting_num;
    let magikarp = Pokemon { name, encounters };

    // Declare app size
    let app = app::App::default();
    let mut wind = Window::default()
        .with_size(160, 200)
        .center_screen()
        .with_label("Counter");
    let frame = Frame::default()
        .with_size(100, 40)
        .center_of(&wind)
        .with_label(&(format!("Encounters: {}", magikarp.encounters)));
    let mut btn = button::Button::new(160, 220, 80, 40, "Click me!");
    btn.set_label("Calibrate Position");
    // let mut proceed: bool = false;

    // btn.set_callback(move |b| {
    //     b.set_label("Recalibrate");
    //     proceed = true;
    // });

    // while (!proceed) {}

    // Get where mouse was pressed x and y
    let coords: Vec<(i32, i32)> = get_press();

    wind.make_resizable(true);
    wind.set_color(Color::TransparentBg);
    wind.end();
    wind.show();

    // Multithreading to allow for tracker to occur while app runs
    thread::spawn(move || {
        tracker(coords, magikarp, frame);
    });

    app.run().unwrap();
}
