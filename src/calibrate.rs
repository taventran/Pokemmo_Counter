use device_query::{DeviceQuery, DeviceState, MouseState};

// Getting mouse press to calibrate where to look for name
pub fn get_press() -> Vec<(i32, i32)> {
    // Need to get box for HP
    let mut mouse_pos_vec = Vec::new();
    let device_state = DeviceState::new();

    let mut mouse_1: MouseState = device_state.get_mouse();
    while !mouse_1.button_pressed[1] {
        mouse_1 = device_state.get_mouse();
    }

    // Sleep half a second to get next mouse click
    let half_sec = std::time::Duration::from_millis(500);
    std::thread::sleep(half_sec);

    let mut mouse_2: MouseState = device_state.get_mouse();
    while !mouse_2.button_pressed[1] {
        mouse_2 = device_state.get_mouse();
    }

    mouse_pos_vec.push(mouse_1.coords);
    mouse_pos_vec.push(mouse_2.coords);
    return mouse_pos_vec;
}
