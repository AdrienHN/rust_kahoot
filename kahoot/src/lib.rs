#![recursion_limit = "512"]

mod components;
mod services;

use wasm_bindgen::prelude::*;

use components::chat::Chat;
use components::login::Login;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<Chat>();

    Ok(())
}

#[wasm_bindgen]
pub fn run_index() -> Result<(), JsValue> {
    yew::start_app::<Login>();

    Ok(())
}
