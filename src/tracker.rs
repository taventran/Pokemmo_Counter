use crate::calibrate::get_press;
use crate::pokemon_struct::Pokemon;
use crate::save_data::save_data;
use fltk::{button::Button, prelude::*, text};
use image_compare::{rgb_hybrid_compare, CompareError, Similarity};
use screenshots::Screen;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;

// Check result of comparison
fn process_result(result: Result<Similarity, CompareError>) -> Option<f64> {
    match result {
        Ok(similarity) => {
            let s = similarity.score;
            Some(s)
        }
        Err(error) => {
            println!("Something went wrong with the comparison: {}", error);
            None
        }
    }
}

// Using 2 mouse position coordinates find if image is still the same
// And update encounter and gui accordingly
pub fn tracker(
    mut p_mon: Pokemon,
    text: text::TextDisplay,
    mut btn: Button,
    mut add_btns: Vec<Button>,
    mut add_by_text: text::TextDisplay,
    add_by: u32,
) {
    // Should find a more efficient way to change add by
    let add_by = Arc::new(AtomicU32::new(add_by));
    let add_by_clone1 = Arc::clone(&add_by);
    let add_by_clone2 = Arc::clone(&add_by);
    let add_by_clone3 = Arc::clone(&add_by);
    add_btns[0].set_callback(move |_| {
        add_by_clone1.store(1, Ordering::Relaxed);
    });
    add_btns[1].set_callback(move |_| {
        add_by_clone2.store(3, Ordering::Relaxed);
    });
    add_btns[2].set_callback(move |_| {
        add_by_clone3.store(5, Ordering::Relaxed);
    });

    // Get screens
    let screens = Screen::all().unwrap();
    // let mut proceed: bool = false;
    let coords = get_press();

    btn.set_label("recalibrate");

    let width_height = (coords[1].0 - coords[0].0, coords[1].1 - coords[0].1);
    let unsigned_w_h = (width_height.0 as u32, width_height.1 as u32);
    println!(
        "Right clicked at: {:?}, Width and Height: {:?}",
        coords, unsigned_w_h
    );

    // Capture initial image
    let image = screens[0]
        .capture_area(coords[0].0, coords[0].1, unsigned_w_h.0, unsigned_w_h.1)
        .unwrap();
    image.save(format!("target/hp.png")).unwrap();
    let image_one = image::open("target/hp.png").expect("Not same").to_rgb8();

    // Needed to use arc and atomic bool to pass in closure
    let run = Arc::new(AtomicBool::new(true));
    let run_clone = Arc::clone(&run);

    // If they click recalibrate stop while loop
    btn.set_callback(move |_b| {
        run_clone.store(false, Ordering::Relaxed);
    });

    while run.load(Ordering::Relaxed) {
        // Update label
        let mut update_label = text::TextBuffer::default();
        update_label
            .set_text(format!("Increasing By: {}", add_by.load(Ordering::Relaxed)).as_str());
        add_by_text.set_buffer(update_label);

        // Capture second image to compare
        let mut image = screens[0]
            .capture_area(coords[0].0, coords[0].1, unsigned_w_h.0, unsigned_w_h.1)
            .unwrap();
        image.save(format!("target/check.png")).unwrap();

        let mut image_two = image::open("target/check.png").expect("Not same").to_rgb8();

        // Compare the two images to see if it's the same
        let mut result = rgb_hybrid_compare(&image_one, &image_two);
        let mut score = process_result(result);
        // If the same stay here and add to encounters
        if score >= Some(0.95) {
            p_mon.encounters += add_by.load(Ordering::Relaxed);
            println!(
                "Name of pokemon: {}, Number of Encounters: {}",
                p_mon.name, p_mon.encounters
            );
            // Update label
            update_label = text::TextBuffer::default();
            update_label.set_text(format!("Encounters: {}", p_mon.encounters).as_str());
            text.clone().set_buffer(update_label);
            let _ = save_data(&p_mon);

            // Don't update score while image is the same
            while score >= Some(0.95) {
                image = screens[0]
                    .capture_area(coords[0].0, coords[0].1, unsigned_w_h.0, unsigned_w_h.1)
                    .unwrap();
                image.save(format!("target/check.png")).unwrap();
                image_two = image::open("target/check.png")
                    .expect("Not same dimensions")
                    .to_rgb8();

                result = rgb_hybrid_compare(&image_one, &image_two);
                score = process_result(result);
            }
        }
    }
    // If they exited out of the loop they clicked recalibrate
    tracker(
        p_mon,
        text,
        btn,
        add_btns,
        add_by_text,
        add_by.load(Ordering::Relaxed),
    );
}
