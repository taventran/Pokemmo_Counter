mod pokemon_struct; // Use module code
mod save_data;
use crate::pokemon_struct::Pokemon; // Get the struct I want
mod calibrate;
mod tracker;
use crate::tracker::tracker;
mod read_data;
use crate::read_data::read_data;
use fltk::{app, button, prelude::*, text::TextDisplay, window::Window};
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
    let mut wind = Window::default()
        .with_size(160, 200)
        .center_screen()
        .with_label("Counter");
    // Initializng Grid layout
    let mut grid = Grid::default_fill();
    grid.show_grid(true);
    grid.set_layout(5, 1);

    let mut btn = button::Button::new(0, 1, 0, 1, "Calibrate Position");
    let mut text = TextDisplay::new(
        1,
        1,
        1,
        1,
        (format!("Encounters: {}", magikarp.encounters)).as_str(),
    );
    grid.set_widget(&mut btn, 3, 0);
    grid.set_widget(&mut text, 2, 0);
    wind.end();
    wind.show();

    // Multithreading to allow for tracker to occur while app runs
    btn.set_callback(move |b| {
        // Cloning objects to avoid data races
        let mut magikarp = magikarp.clone();
        let text = text.clone();
        let b = b.clone();
        // Update clone with most recent saved data
        magikarp.encounters = read_data();
        thread::spawn(move || {
            tracker(magikarp, text, b);
        });
    });

    app.run().unwrap();
}
