use wasm_bindgen::prelude::*;

#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn readDocx(buf: &[u8]) -> Result<String, JsValue> {
    let mut d = docx_rs::read_docx(buf);
    match d {
        Ok(re