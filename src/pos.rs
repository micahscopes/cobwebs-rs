use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct NodePositionUpdate {
    pub id: i32,
    pub x: f32,
    pub y: f32,
}

use serde::{Serialize,Deserialize};
#[no_mangle]
pub fn position_update(time: f32) -> JsValue {
    let example = NodePositionUpdate {
        id: 1,
        x: 100.,
        y: 100.,
    };

    JsValue::from_serde(&example).unwrap()
}