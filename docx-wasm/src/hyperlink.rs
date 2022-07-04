use wasm_bindgen::prelude::*;

use super::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Hyperlink(docx_rs::Hyperlink);

#[wasm_bindgen(js_name = createHyperlink)]
pub fn create_hyperlink(v: &str, t: docx_rs::HyperlinkType) -> Hyperlink {
    Hyperlink(docx_rs::Hyperlink::new(v, t))
}

#[wasm_bindgen]
impl Hyperlink {
    pub fn add_run(mut self, run: Run) -> Self {
        self.0 = self.0.add_run(run.take());
        self
    }

    pub fn add_insert(mut self, i: Insert) -> Self {
        self.0
            