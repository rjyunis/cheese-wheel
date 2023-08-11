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
use mouce::common::MouseEvent;

// Interact with windows
use winapi::um::winuser::{GetCursorPos, WindowFromPoint, GetWindowTextW, GetWindowTextLengthW, SetLayeredWindowAttributes, LWA_ALPHA, LWA_COLORKEY, SetWindowLongPtrW, GWL_EXSTYLE, WS_EX_LAYERED, GetWindowLongPtrW, WS_EX_TRANSPARENT};
use winapi::um::winnt::WCHAR;
use winapi::shared::windef::{POINT, COLORREF};
// use winapi::shared::windef::HWND;
use winapi::shared::ntdef::LONG;
use winapi::shared::minwindef::DWORD;

fn main() -> Result<(), Box<dyn std::error::Error>> {

  let mut mouse_manager = Mouse::new();

  println!("hello, world");

  // listen(mouse_manager);

  mouse_manager.hook(Box::new(|event| {

    if let MouseEvent::Press(mouce::common::MouseButton::Left) = event {

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

      // Set WS_EX_LAYERED extended style
      let current_style = unsafe { GetWindowLongPtrW(window_handle, GWL_EXSTYLE) as DWORD };


      // Check if both WS_EX_LAYERED and WS_EX_TRANSPARENT are set
      // They should always be in sync. If they're not, err on caution and go to low-energy state (normal)
      let is_layered_transparent = (current_style & (WS_EX_LAYERED | WS_EX_TRANSPARENT)) == (WS_EX_LAYERED | WS_EX_TRANSPARENT);
      let new_style: u32;
      if is_layered_transparent {
        new_style = current_style & !(WS_EX_LAYERED | WS_EX_TRANSPARENT);
      }
      else {
        new_style = current_style & !(WS_EX_LAYERED | WS_EX_TRANSPARENT);
      }

      unsafe { SetWindowLongPtrW(window_handle, GWL_EXSTYLE, (new_style as LONG).try_into().unwrap()) };

      // Transparency parameters
      let transparency_value = 80; // Adjust this value as needed (0 - fully transparent, 255 - fully opaque)
      let key_color = COLORREF::default(); // Specify a key color if needed

      // Set window attributes for transparency
      unsafe {
        SetLayeredWindowAttributes(window_handle, key_color, transparency_value, LWA_ALPHA | LWA_COLORKEY);
      }
    }
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
