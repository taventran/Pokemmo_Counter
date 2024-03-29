use crate::calibrate::get_press;
use crate::read_data::get_exe_directory;
use fltk::{button::Button, prelude::*, text};
use image_compare::{rgb_hybrid_compare, CompareError, Similarity};
use screenshots::Screen;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::mpsc::Sender;
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
    mut btn: Button,
    mut add_btns: Vec<Button>,
    mut add_by_text: text::TextDisplay,
    add_by: i32,
    sender: Sender<i32>,
) {
    // Should find a more efficient way to change add by
    let add_by = Arc::new(AtomicI32::new(add_by));
    let add_by_clone1 = Arc::clone(&add_by);
    let add_by_clone2 = Arc::clone(&add_by);
    let add_by_clone3 = Arc::clone(&add_by);
    let add_by_text1 = add_by_text.clone();
    let add_by_text2 = add_by_text.clone();
    let add_by_text3 = add_by_text.clone();

    add_btns[0].set_callback(move |_| {
        add_by_clone1.store(1, Ordering::Relaxed);
        let mut update_label = text::TextBuffer::default();
        update_label.set_text(format!("Auto Increasing By: {}", 1).as_str());
        add_by_text1.clone().set_buffer(update_label);
    });
    add_btns[1].set_callback(move |_| {
        add_by_clone2.store(3, Ordering::Relaxed);
        let mut update_label = text::TextBuffer::default();
        update_label.set_text(format!("Auto Increasing By: {}", 3).as_str());
        add_by_text2.clone().set_buffer(update_label);
    });
    add_btns[2].set_callback(move |_| {
        add_by_clone3.store(5, Ordering::Relaxed);
        let mut update_label = text::TextBuffer::default();
        update_label.set_text(format!("Auto Increasing By: {}", 5).as_str());
        add_by_text3.clone().set_buffer(update_label);
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
    image
        .save(get_exe_directory().unwrap().join("hp.png"))
        .unwrap();
    let image_one = image::open(get_exe_directory().unwrap().join("hp.png"))
        .expect("Not same")
        .to_rgb8();

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
            .set_text(format!("Auto Increasing By: {}", add_by.load(Ordering::Relaxed)).as_str());
        add_by_text.set_buffer(update_label);

        // Capture second image to compare
        let mut image = screens[0]
            .capture_area(coords[0].0, coords[0].1, unsigned_w_h.0, unsigned_w_h.1)
            .unwrap();
        image
            .save(get_exe_directory().unwrap().join("check.png"))
            .unwrap();

        let mut image_two = image::open(get_exe_directory().unwrap().join("check.png"))
            .expect("Not same")
            .to_rgb8();

        // Compare the two images to see if it's the same
        let mut result = rgb_hybrid_compare(&image_one, &image_two);
        let mut score = process_result(result);

        // Set sleep so it doesn't hog too much CPU
        let sec = std::time::Duration::from_millis(1000);
        std::thread::sleep(sec);

        // If the same stay here and add to encounters
        if score >= Some(0.95) {
            sender.send(add_by.load(Ordering::Relaxed)).unwrap();
            while score >= Some(0.95) {
                image = screens[0]
                    .capture_area(coords[0].0, coords[0].1, unsigned_w_h.0, unsigned_w_h.1)
                    .unwrap();
                image
                    .save(get_exe_directory().unwrap().join("check.png"))
                    .unwrap();
                image_two = image::open(get_exe_directory().unwrap().join("check.png"))
                    .expect("Not same dimensions")
                    .to_rgb8();
                // 5 seconds to wait for attack animations
                std::thread::sleep(std::time::Duration::from_millis(5000));

                result = rgb_hybrid_compare(&image_one, &image_two);
                score = process_result(result);
                if !run.load(Ordering::Relaxed) {
                    break;
                }
            }
        }
    }
    println!("Exited while loop and restarted tracker");
    // If they exited out of the loop they clicked recalibrate
    tracker(
        btn,
        add_btns,
        add_by_text,
        add_by.load(Ordering::Relaxed),
        sender,
    );
}
