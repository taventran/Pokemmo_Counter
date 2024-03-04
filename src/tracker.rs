use crate::calibrate::get_press;
use crate::pokemon_struct::Pokemon;
use crate::save_data::save_data;
use fltk::{button::Button, prelude::*, text};
use image_compare::{rgb_hybrid_compare, CompareError, Similarity};
use screenshots::Screen;
use std::sync::atomic::{AtomicBool, Ordering};
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
pub fn tracker(mut p_mon: Pokemon, text: text::TextDisplay, mut btn: Button) {
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
        // Sleep half a second to get next mouse click
        let two_secs = std::time::Duration::from_millis(2000);
        std::thread::sleep(two_secs);

        // Capture second image to compare
        let mut image = screens[0]
            .capture_area(coords[0].0, coords[0].1, unsigned_w_h.0, unsigned_w_h.1)
            .unwrap();
        image.save(format!("target/check.png")).unwrap();

        let mut image_two = image::open("target/check.png").expect("Not same").to_rgb8();

        // Compare the two images to see if it's the same
        let mut result = rgb_hybrid_compare(&image_one, &image_two);
        let mut score = process_result(result);
        println!("result: {:?}", score);
        // If the same stay here and add to encounters
        if score >= Some(0.95) {
            p_mon.encounters += 5;
            println!(
                "Name of pokemon: {}, Number of Encounters: {}",
                p_mon.name, p_mon.encounters
            );
            // Update label
            let mut update_label = text::TextBuffer::default();
            update_label.set_text(format!("Encounters: {}", p_mon.encounters).as_str());
            text.clone().set_buffer(update_label);
            let _ = save_data(&p_mon);

            // Don't update score while image is the same
            while score >= Some(0.95) {
                let two_secs = std::time::Duration::from_millis(2000);
                std::thread::sleep(two_secs);
                image = screens[0]
                    .capture_area(coords[0].0, coords[0].1, unsigned_w_h.0, unsigned_w_h.1)
                    .unwrap();
                image.save(format!("target/check.png")).unwrap();
                image_two = image::open("target/check.png")
                    .expect("Not same dimensions")
                    .to_rgb8();

                result = rgb_hybrid_compare(&image_one, &image_two);
                score = process_result(result);
                println!("score: {:?}", score);
            }
        }
    }
    // If they exited out of the loop they clicked recalibrate
    tracker(p_mon, text, btn);
}
