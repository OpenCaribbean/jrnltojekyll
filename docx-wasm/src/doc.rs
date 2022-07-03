use super::*;
use wasm_bindgen::prelude::*;

extern crate console_error_panic_hook;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Docx(docx_rs::Docx);

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn createDocx() -> Docx {
    use std::panic;
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    Docx(docx_rs::Docx::new())
}

#[wasm_bindgen]
impl Docx {
    pub fn add_paragraph(mut self, p: Paragraph) -> Self {
        self.0 = self.0.add_paragraph(p.take());
        self
    }

    pub fn add_table_of_contents(mut self, t: TableOfContents) -> Self {
        self.0 = self.0.add_table_of_contents(t.take());
        self
    }

    pub fn add_bookmark_start(mut self, id: usize, name: &str) -> Self {
        self.0 = self.0.add_bookmark_start(id, name);
        self
    }

    pub fn add_bookmark_end(mut self, id: usize) -> Docx {
        self.0 = self.0.add_bookmark_end(id);
        self
    }

    pub fn add_table(mut self, t: Table) -> Docx {
        self.0.document = self.0.document.add_table(t.take());
        self
    }

    pub fn add_abstract_numbering(mut self, num: AbstractNumbering) -> Docx {
        self.0.numberings = self.0.numberings.add_abstract_numbering(num.take());
        self
    }

    pub fn add_numbering(mut self, num: Numbering) -> Docx {
        self.0.numberings = self.0.numberings.add_numbering(num.take());
        self
    }

    pub fn created_at(mut self, date: &str) -> Self {
        self.0.doc_props = self.0.doc_props.created_at(date);
        self
    }

    pub fn updated_at(mut self, date: &str) -> Self {
        self.0.doc_props = self.0.doc_props.updated_at(date);
        self
    }

    pub fn custom_property(mut self, name: &str, item: &str) -> Self {
        self.0.doc_props = self.0.doc_props.custom_property(name, item);
        self
    }

    pub fn doc_id(mut self, id: &str) -> Docx {
        self.0 = self.0.doc_id(id);
        self
    }

    pub fn add_doc_var(mut self, name: &str, val: &str) -> Docx {
        self.0 = self.0