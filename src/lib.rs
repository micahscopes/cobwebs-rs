#![warn(missing_debug_implemntations, rust_2018_idioms, missing_docs)]

mod geometry;
mod intersections;
mod layout;
mod tabu;
mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
