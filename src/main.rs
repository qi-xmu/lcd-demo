use std::process::ExitCode;

use slint_backend::KeyPress;

mod display;
mod slint_backend;

slint::include_modules!();
fn main() -> ExitCode {
    let keys = vec![
        KeyPress::new(6, slint::platform::Key::UpArrow.into()),
        KeyPress::new(19, slint::platform::Key::DownArrow.into()),
        KeyPress::new(5, slint::platform::Key::LeftArrow.into()),
        KeyPress::new(26, slint::platform::Key::RightArrow.into()),
        KeyPress::new(13, slint::platform::Key::Return.into()),
        KeyPress::new(21, slint::platform::Key::F1.into()),
        KeyPress::new(20, slint::platform::Key::F2.into()),
        KeyPress::new(16, slint::platform::Key::F3.into()),
    ];
    let win_size = slint::PhysicalSize::new(128, 128);
    let display =
        display::get_st7735s_display((win_size.width as u16, win_size.height as u16), 27, 24, 25);
    slint_backend::init(display, win_size, Some(keys));
    let ui = AppWindow::new().unwrap();

    ui.run().unwrap();

    ExitCode::SUCCESS
}
