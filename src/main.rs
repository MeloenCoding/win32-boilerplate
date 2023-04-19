use windows::{Win32::{UI::{WindowsAndMessaging::{CS_OWNDC}}}, s};

use crate::window::Window;
mod window;
mod error;

fn main() {
    // If you want to test the custom error, just uncomment this:
    // error::WindowError::new("Unable to load cursor.", None, loc!());

    // create a new window with name "Test". See window.rs for more info
    let mut window: Window = Window::new(s!("Test"), CS_OWNDC);

    // show the window you've created
    window.show_window(); 

    loop {
        match window.handle_message() {
            Some(_) => {
                // when getMessage = 0 (exit without an error) | -1 (exit with an error)
                break;
            },
            None => {/* do nothing and continue the loop */},
        }
    }

    // print the exit codes
    window.get_exit_codes();
}
