use rust_embed::RustEmbed;
mod pokemon_struct; // Use module code
mod save_data;
use crate::pokemon_struct::Pokemon;
use crate::save_data::save_file; // Get the struct I want
mod calibrate;
mod read_data;
mod tracker;
use crate::read_data::{create_vec, read_from_file};
use crate::tracker::tracker;
use fltk::{
    app, button::Button, enums::*, frame::Frame, image::PngImage, prelude::*, text, window::Window,
};
use fltk_grid::Grid;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;

#[derive(RustEmbed)]
#[folder = "images/"]
struct Asset;

fn main() {
    // Initialize a pokemon
    let name = "magikarp".to_string();
    let (name, encounters) = read_from_file("save.csv", name).unwrap();
    let cur_poke = Pokemon {
        name: Box::leak(name.to_owned().into_boxed_str()),
        encounters,
    };
    let cur_poke_clone1 = cur_poke.clone();

    let all_pokemon = create_vec("save.csv");
    // let saved_pokemon;
    match &all_pokemon {
        Ok(all_pokemon) => {
            all_pokemon.iter().for_each(|pokemon| {
                println!("Name: {}, Encounters: {}", pokemon.name, pokemon.encounters);
            });
        }
        Err(err) => {
            println!("Error: {} Couldn't make vec", err);
        }
    }
    let all_pokemon = all_pokemon.unwrap();

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
    label1.set_text(format!("Hunting: {}", { cur_poke.name }).as_str());
    let mut name = text::TextDisplay::new(1, 1, 1, 1, "");
    name.set_buffer(label1);
    name.set_color(Color::rgb_color(213, 49, 65));
    name.set_text_color(Color::rgb_color(244, 244, 244));

    let mut label2 = text::TextBuffer::default();
    label2.set_text(format!("{}", cur_poke.encounters).as_str());
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
    grid.set_widget(&mut num_encounters, 1, 0..2);
    grid.set_widget(&mut btn_increase, 1, 2);
    grid.set_widget(&mut btn_decrease, 1, 3);
    grid.set_widget(&mut add_by_text, 2, 0..4);
    grid.set_widget(&mut calibrate_btn, 3, 0..4);
    grid.set_widget(&mut btn1, 4, 0);
    grid.set_widget(&mut btn2, 4, 1);
    grid.set_widget(&mut btn3, 4, 2);
    grid.set_widget(&mut img_frame, 4, 3);

    let mut add_btns: Vec<Button> = vec![btn1, btn2, btn3];

    let (sender_increase, receiver_increase) = channel();
    let sender1 = sender_increase.clone();
    let sender2 = sender_increase.clone();

    // Should find a more efficient way to do this
    let mut add_by_text_clone1 = add_by_text.clone();
    let mut add_by_text_clone2 = add_by_text.clone();
    let mut add_by_text_clone3 = add_by_text.clone();

    let add_by = Arc::new(AtomicI32::new(1));
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

    btn_increase.set_callback(move |_| {
        sender1.send(1).unwrap();
    });

    btn_decrease.set_callback(move |_| {
        sender2.send(-1).unwrap();
    });

    calibrate_btn.set_callback(move |b| {
        // Cloning objects to avoid data races
        let b = b.clone();
        let add_btns = add_btns.clone();
        let add_by_text = add_by_text.clone();
        let add_by = add_by.load(Ordering::Relaxed).clone();
        let sender = sender_increase.clone();

        thread::spawn(move || {
            tracker(b, add_btns, add_by_text, add_by, sender);
        });
    });

    let mut num_encounters = num_encounters.clone();

    thread::spawn(move || loop {
        let mut info = read_from_file("save.csv", cur_poke_clone1.name.to_string());
        let (name, mut encounters) = info.unwrap();
        let num = receiver_increase.recv().unwrap();
        encounters += num;
        let temp_poke = Pokemon {
            name: Box::leak(name.to_owned().into_boxed_str()),
            encounters,
        };
        let mut new_label = text::TextBuffer::default();
        new_label.set_text(format!("{}", encounters).as_str());
        num_encounters.set_buffer(new_label);
        // save_data(&temp_poke).err();
        save_file("save.csv", all_pokemon.clone(), temp_poke);
        println!("Received: {}", num);
    });

    wind.end();
    wind.show();
    app.run().unwrap();
}
