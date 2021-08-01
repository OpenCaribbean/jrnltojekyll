use docx_rs::*;

pub fn main() -> Result<(), DocxError> {
    let path = std::path::Path::new("./output/page_margin.docx");
    let file = std::fs::File::create(&path).unwrap();
    Docx::new()
        .add_paragr