use crate::calibrate::get_press;
use crate::pokemon_struct::Pokemon;
use crate::save_data::save_data;
use fltk::{button::Button, prelude::*, text::TextDisplay};
use image_compare::{rgb_hybrid_compare, CompareError, Similarity};
use screenshots::Screen;
use std::sync::mpsc::{self, TryRecvError};

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
pub fn tracker(mut p_mon: Pokemon, text: TextDisplay, mut btn: Button) {
    // Get screens
    let (tx, rx) = mpsc::channel();
    let screens = Screen::all().unwrap();
    // let mut proceed: bool = false;
    let coords = get_press();
    btn.set_label("recalibrate");
    let _ = tx.send(());

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
    // In loop keep checking if hp image is the same
    while true {
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
            text.clone()
                .with_label(&(format!("Encounters: {}", p_mon.encounters)));
            let _ = save_data(&p_mon);
            // break if not the same
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
        match rx.try_recv() {
            Ok(_) | Err(TryRecvError::Disconnected) => {
                println!("Terminating.");
                break;
            }
            Err(TryRecvError::Empty) => {}
        }
    }
}
