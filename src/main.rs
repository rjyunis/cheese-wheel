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
use crate::mouce::Mouse;

fn main() {

  let mouse_manager = Mouse::new();

  println!("hello, world");

  listen(mouse_manager);

}

fn listen(mouse_manager) {
  mouse_manager.hook(Box::new(|event| {
    println!("you did{:?}", event);
  }))?;
  loop {
    // Call sleep to avoid heavy cpu load
    sleep(Duration::from_secs(u64::max_value()));
  }

}
