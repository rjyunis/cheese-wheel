/*
 * Detect mouse clicks
 * in Rust
 *
 * Yunis, Ramsey J.
 * 2023-08-10
 */

// Avoid excessive polling
use std::thread::sleep;
use std::time::Duration;

// Mouse API Library
use mouce::Mouse;
// use mouce::MouseActions;

// Interact with windows
use winapi::um::winuser::{GetCursorPos, WindowFromPoint, GetWindowTextW, GetWindowTextLengthW};
use winapi::um::winnt::WCHAR;
use winapi::shared::windef::POINT;

fn main() -> Result<(), Box<dyn std::error::Error>> {

  let mut mouse_manager = Mouse::new();

  println!("hello, world");

  // listen(mouse_manager);

  mouse_manager.hook(Box::new(|event| {
    println!("you did{:?}", event);

    // Retrieve Mouse Position and Window
    let mut cursor_pos: POINT = POINT { x: 0, y: 0 };
    unsafe { GetCursorPos(&mut cursor_pos) };
    let window_handle = unsafe { WindowFromPoint(cursor_pos) };

    // Retrieve window title
    const BUFFER_SIZE: usize = 256;
    let mut window_title: [WCHAR; BUFFER_SIZE] = [0; BUFFER_SIZE];
    unsafe {
        let title_length = GetWindowTextLengthW(window_handle) + 1;
        GetWindowTextW(window_handle, window_title.as_mut_ptr(), title_length);
    }

    // Convert title to rust string
    let window_title_string = String::from_utf16_lossy(&window_title);
    println!("Current window title: {}", window_title_string);

  }))?;
  loop {
    // Call sleep to avoid heavy cpu load
    sleep(Duration::from_secs(u64::max_value()));
  }

}

/*
 * TODO ask tim about this
fn listen(mouse_manager: Box<dyn MouseActions>) -> Int,  {
  let hook = mouse_manager.hook(Box::new(|event| {
    println!("you did{:?}", event);
  }))?;
  loop {
    // Call sleep to avoid heavy cpu load
    sleep(Duration::from_secs(u64::max_value()));
  }

  return ();
}
*/
