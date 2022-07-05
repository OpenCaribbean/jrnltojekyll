
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct TableCellBorder(docx_rs::TableCellBorder);

#[wasm_bindgen(js_name = createTableCellBorder)]
pub fn create_table_cell_border(position: docx_rs::TableCellBorderPosition) -> TableCellBorder {
    TableCellBorder(docx_rs::TableCellBorder::new(position))
}

impl TableCellBorder {
    pub fn take(self) -> docx_rs::TableCellBorder {
        self.0
    }
}

#[wasm_bindgen]
impl TableCellBorder {
    pub fn size(mut self, size: usize) -> TableCellBorder {
        self.0.size = size;
        self
    }

    pub fn color(mut self, color: String) -> TableCellBorder {
        self.0.color = color;
        self
    }

    pub fn border_type(mut self, border_type: docx_rs::BorderType) -> TableCellBorder {
        self.0.border_type = border_type;
        self
    }

    pub fn get_size(&self) -> usize {
        self.0.get_size()
    }

    pub fn get_color(&self) -> String {
        self.0.get_color()
    }

    pub fn get_border_type(&self) -> docx_rs::BorderType {
        self.0.get_border_type()
    }
}