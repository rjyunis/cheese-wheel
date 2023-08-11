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


fn main() -> Result<(), Box<dyn std::error::Error>> {

  let mut mouse_manager = Mouse::new();

  println!("hello, world");

  // listen(mouse_manager);

  mouse_manager.hook(Box::new(|event| {
    println!("you did{:?}", event);
  }))?;
  loop {
    // Call sleep to avoid heavy cpu load
    sleep(Duration::from_secs(u64::max_value()));
  }

}

/*
 * TODO ask tim
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
