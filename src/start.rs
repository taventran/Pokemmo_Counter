use fltk::{
    app, button::Button, enums::*, frame::Frame, image::PngImage, input, prelude::*, text,
    window::Window,
};
use fltk_grid::Grid;

pub fn start_menu() -> String {
    let app = app::App::default();
    let mut get_name: String;
    // Declare app size
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
    grid.set_layout(3, 1);
    let mut input_text = input::Input::new(100, 100, 200, 30, None);
    let mut button = Button::new(100, 150, 200, 30, "Submit");

    grid.set_widget(&mut input_text, 0, 0);
    grid.set_widget(&mut button, 1, 0);

    button.set_callback(move |_| {
        let name = input_text.value();
        println!("Input: {}", name);
    });

    wind.show();

    //     app.run().unwrap();
    //     get_name
    return "Nothing".to_string();
}
