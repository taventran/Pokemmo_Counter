use rust_embed::RustEmbed;
mod pokemon_struct; // Use module code
mod save_data;
use crate::pokemon_struct::Pokemon; // Get the struct I want
mod calibrate;
mod read_data;
mod tracker;
use crate::read_data::read_data;
use crate::tracker::tracker;
use fltk::{
    app, button::Button, enums::*, frame::Frame, image::PngImage, prelude::*, text, window::Window,
};
use fltk_grid::Grid;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::thread;

#[derive(RustEmbed)]
#[folder = "images/"]
struct Asset;

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
    grid.set_layout(5, 4);
    // Adding image
    // let img = Asset::get("target/gengar.png").unwrap();

    let mut img = PngImage::load("images/pokeball.png").unwrap();
    img.scale(70, 55, true, true);
    let mut img_frame = Frame::default();
    img_frame.set_image(Some(img));

    // Creating buttons
    let mut calibrate_btn = Button::new(1, 1, 1, 1, "Calibrate Position");
    calibrate_btn.set_color(Color::rgb_color(131, 16, 16));
    calibrate_btn.set_label_color(Color::rgb_color(244, 244, 244));

    let mut btn1 = Button::new(0, 0, 0, 0, "+1");
    let mut btn2 = Button::new(0, 0, 0, 0, "+3");
    let mut btn3 = Button::new(0, 0, 0, 0, "+5");

    // Adding colors
    btn1.set_color(Color::rgb_color(131, 16, 16));
    btn1.set_label_color(Color::rgb_color(244, 244, 244));
    btn2.set_color(Color::rgb_color(131, 16, 16));
    btn2.set_label_color(Color::rgb_color(244, 244, 244));
    btn3.set_color(Color::rgb_color(131, 16, 16));
    btn3.set_label_color(Color::rgb_color(244, 244, 244));

    let mut btn_increase = Button::new(0, 0, 0, 0, "⬆️");
    let mut btn_decrease = Button::new(0, 0, 0, 0, "⬇️");
    // Creating Labels and styling them
    let mut label1 = text::TextBuffer::default();
    label1.set_text(format!("Hunting: {}", { magikarp.name }).as_str());
    let mut name = text::TextDisplay::new(1, 1, 1, 1, "");
    name.set_buffer(label1);
    name.set_color(Color::rgb_color(213, 49, 65));
    name.set_text_color(Color::rgb_color(244, 244, 244));

    let mut label2 = text::TextBuffer::default();
    label2.set_text(format!("{}", magikarp.encounters).as_str());
    let mut num_encounters = text::TextDisplay::new(1, 1, 1, 1, "");
    num_encounters.set_buffer(label2);
    num_encounters.set_color(Color::rgb_color(213, 49, 65));
    num_encounters.set_text_color(Color::rgb_color(244, 244, 244));

    let mut label3 = text::TextBuffer::default();
    label3.set_text("Increasing By: 1");
    let mut add_by_text = text::TextDisplay::new(1, 1, 1, 1, "");
    add_by_text.set_buffer(label3);
    add_by_text.set_color(Color::rgb_color(213, 49, 65));
    add_by_text.set_text_color(Color::rgb_color(244, 244, 244));

    grid.set_widget(&mut name, 0, 0..4);
    grid.set_widget(&mut num_encounters, 1, 0..4);
    grid.set_widget(&mut btn_increase, 1, 2);
    grid.set_widget(&mut btn_decrease, 1, 3);
    grid.set_widget(&mut add_by_text, 2, 0..4);
    grid.set_widget(&mut calibrate_btn, 3, 0..4);
    grid.set_widget(&mut btn1, 4, 0);
    grid.set_widget(&mut btn2, 4, 1);
    grid.set_widget(&mut btn3, 4, 2);
    grid.set_widget(&mut img_frame, 4, 3);

    let mut add_btns: Vec<Button> = vec![btn1, btn2, btn3];

    // Should find a more efficient way to do this
    let mut add_by_text_clone1 = add_by_text.clone();
    let mut add_by_text_clone2 = add_by_text.clone();
    let mut add_by_text_clone3 = add_by_text.clone();

    let add_by = Arc::new(AtomicU32::new(1));
    let add_by_clone1 = Arc::clone(&add_by);
    let add_by_clone2 = Arc::clone(&add_by);
    let add_by_clone3 = Arc::clone(&add_by);

    add_btns[0].set_callback(move |_| {
        add_by_clone1.store(1, Ordering::Relaxed);
        let mut update_label = text::TextBuffer::default();
        update_label
            .set_text(format!("Increasing By: {}", add_by_clone1.load(Ordering::Relaxed)).as_str());
        add_by_text_clone1.set_buffer(update_label);
    });
    add_btns[1].set_callback(move |_| {
        add_by_clone2.store(3, Ordering::Relaxed);
        let mut update_label = text::TextBuffer::default();
        update_label
            .set_text(format!("Increasing By: {}", add_by_clone2.load(Ordering::Relaxed)).as_str());
        add_by_text_clone2.set_buffer(update_label);
    });
    add_btns[2].set_callback(move |_| {
        add_by_clone3.store(5, Ordering::Relaxed);
        let mut update_label = text::TextBuffer::default();
        update_label
            .set_text(format!("Increasing By: {}", add_by_clone3.load(Ordering::Relaxed)).as_str());
        add_by_text_clone3.set_buffer(update_label);
    });

    btn_increase.hide();
    btn_decrease.hide();
    // TODO: Add a manual way to increase and decrease count

    // Multithreading to allow for tracker to occur while app runs
    calibrate_btn.set_callback(move |b| {
        // Cloning objects to avoid data races
        let magikarp = magikarp.clone();
        let num_encounters = num_encounters.clone();
        let b = b.clone();
        let add_btns = add_btns.clone();
        let add_by_text = add_by_text.clone();
        let add_by = add_by.load(Ordering::Relaxed).clone();
        // Update pokemon clone with most recent data
        thread::spawn(move || {
            tracker(magikarp, num_encounters, b, add_btns, add_by_text, add_by);
        });
    });

    wind.end();
    wind.show();
    app.run().unwrap();
}
