use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct LineSpacing(docx_rs::LineSpacing);

#[wasm_bindgen(js_name = createLineSpacing)]
pub fn create_line_spacing() -> LineSpacing {
    Line