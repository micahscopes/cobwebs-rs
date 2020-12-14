mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize)]
pub struct NodePositionUpdate {
    pub id: u32,
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
pub fn position_update(t: f64) -> JsValue {
    let example = NodePositionUpdate {
        id: 1,
        x: 100.*t.sin(),
        y: 200.,
    };

    return JsValue::from_serde(&example).unwrap()
}
