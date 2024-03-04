mod pokemon_struct; // Use module code
mod save_data;
use crate::pokemon_struct::Pokemon; // Get the struct I want
mod calibrate;
mod read_data;
mod tracker;
use crate::read_data::read_data;
use crate::tracker::tracker;
use fltk::{app, button::Button, enums::Align, enums::*, prelude::*, text, window::Window};
use fltk_grid::Grid;
use std::thread;

fn main() {
    // Initialize a pokemon
    let starting_num = read_data();
    let name: &str = "Magikarp";
    let encounters: u32 = starting_num;
    let magikarp = Pokemon { name, encounters };

    // Declare app size
    let app = app::App::default();
    app::background(81, 70, 78);
    app::set_font(Font::Courier);
    app::set_font_size(20);
    let mut wind = Window::default()
        .with_size(300, 300)
        .center_screen()
        .with_label("Counter");
    // Initializng Grid layout
    let mut grid = Grid::default_fill();
    grid.set_align(Align::Center);
    // grid.show_grid(true);
    grid.set_layout(5, 3);

    // Creating buttons
    let mut calibrate_btn = Button::new(1, 1, 1, 1, "Calibrate Position");
    calibrate_btn.set_color(Color::XtermBgYellow);

    let mut btn1 = Button::new(0, 0, 0, 0, "+1");
    let mut btn2 = Button::new(0, 0, 0, 0, "+3");
    let mut btn3 = Button::new(0, 0, 0, 0, "+5");

    let mut label1 = text::TextBuffer::default();
    label1.set_text(format!("Hunting: {}", { magikarp.name }).as_str());
    let mut name = text::TextDisplay::new(1, 1, 1, 1, "");
    name.set_buffer(label1);
    name.set_color(Color::rgb_color(60, 90, 166));
    name.set_text_color(Color::rgb_color(199, 160, 8));
    name.set_text_size(25);

    let mut label2 = text::TextBuffer::default();
    label2.set_text(format!("Encounters: {}", magikarp.encounters).as_str());
    let mut num_encounters = text::TextDisplay::new(1, 1, 1, 1, "");
    num_encounters.set_text_size(50);
    num_encounters.set_buffer(label2);
    num_encounters.set_color(Color::rgb_color(60, 90, 166));
    num_encounters.set_text_color(Color::rgb_color(199, 160, 8));
    num_encounters.set_text_size(25);

    let mut label3 = text::TextBuffer::default();
    label3.set_text("Increasing Encounters By: 1");
    let mut add_by_text = text::TextDisplay::new(1, 1, 1, 1, "");
    add_by_text.set_text_size(50);
    add_by_text.set_buffer(label3);
    add_by_text.set_color(Color::rgb_color(60, 90, 166));
    add_by_text.set_text_color(Color::rgb_color(199, 160, 8));
    add_by_text.set_text_size(25);

    grid.set_widget(&mut name, 0, 0..3);
    grid.set_widget(&mut num_encounters, 1, 0..3);
    grid.set_widget(&mut calibrate_btn, 2, 0..3);
    grid.set_widget(&mut btn1, 3, 0);
    grid.set_widget(&mut btn2, 3, 1);
    grid.set_widget(&mut btn3, 3, 2);
    grid.set_widget(&mut add_by_text, 4, 0..3);

    wind.end();
    wind.show();

    let add_btns: Vec<Button> = vec![btn1, btn2, btn3];
    // Multithreading to allow for tracker to occur while app runs
    calibrate_btn.set_callback(move |b| {
        // Cloning objects to avoid data races
        let mut magikarp = magikarp.clone();
        let num_encounters = num_encounters.clone();
        let b = b.clone();
        let add_btns = add_btns.clone();
        let add_by_text = add_by_text.clone();
        // Update pokemon clone with most recent data
        thread::spawn(move || {
            tracker(magikarp, num_encounters, b, add_btns, add_by_text);
        });
    });

    app.run().unwrap();
}
