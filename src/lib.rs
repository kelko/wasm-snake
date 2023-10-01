mod model;
mod snake;
mod world;
mod reward;
mod display;

use wasm_bindgen::prelude::*;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub use world::World;
pub use display::Display;