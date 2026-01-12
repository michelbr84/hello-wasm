use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // Acessa window/document
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("no global `window` exists"))?;
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("should have a document on window"))?;

    // Pega a div #app e coloca o texto
    let app = document
        .get_element_by_id("app")
        .ok_or_else(|| JsValue::from_str("missing #app element"))?;

    app.set_inner_html("Hello World! (Rust + WASM)");

    Ok(())
}