use serde::Serialize;

use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

/*
    Please see. L.4.3.2.18 Cell Border Properties

    left – left border
    right – right border
    top – top border
    bottom – bottom border
    insideH – inner horizontal borders
    insideV – inner vertical borders
    tl2br – diagonal border from top left corner to bottom right corner
    tr2bl – diagonal border from top right corner to bottom left corner
*/
#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TableCellBorder {
    pub border_type: BorderType,
    pub size: usize,
    pub color: String,
    position: TableCellBorderPosition,
    space: usize,
}

impl TableCellBorder {
    pub fn new(position: TableCellBorderPosition) -> TableCellBorder {
        TableCellBorder {
            position,
            border_type: BorderType::Single,
            size: 2,
            space: 0,
            color: "000000".to_owned(),
        }
    }

    pub fn color(mut self, color: impl Into<String>) -> TableCellBorder {
        self.color = color.into();
        self
    }

    pub fn size(mut self, size: usize) -> TableCellBorder {
        self.size = size;
        self
    }

    pub fn border_type(mut self, border_type: BorderType) -> TableCellBorder {
        self.border_type = border_type;
        self
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn get_color(&self) -> String {
        self.color.clone()
    }

    pub fn get_border_type(&self) -> BorderType {
        self.border_type
    }
}

impl BuildXML for TableCellBorder {
    fn build(&self) -> Vec<u8> {
        let base = XMLBuilder::new();
        let base = match self.position {
            TableCellBorderPosition::Top => {
                base.border_top(self.border_type, self.size, self.space, &self.color)
            }
            TableCellBorderPosition::Left => {
                base.border_left(self.border_type, self.size, self.space, &self.color)
            }
            TableCellBorderPosition::Bottom => {
                base.border_bottom(self.border_type, self.size, self.space, &self.color)
            }
            TableCellBorderPosition::Right => {
                base.border_right(self.border_type, self.size, self.space, &self.color)
            }
            TableCellBorderPosition::InsideH => {
                base.border_inside_h(self.border_type, self.size, self.space, &self.color)
            }
            TableCellBorderPosition::InsideV => {
                base.border_inside_v(self.border_type, self.size, self.space, &self.color)
            }
            TableCellBorderPosition::Tr2bl => {
                base.border_tr2bl(self.border_type, self.size, self.space, &self.color)
            }
            TableCellBorderPosition::Tl2br => {
                base.border_tl2br(self.border_type, self.size, self.space, &self.color)
            }
        };
        base.build()
    }
}

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TableCellBorders {
    top: Option<TableCellBorder>,
    left: Option<TableCellBorder>,
    bottom: Option<TableCellBorder>,
    right: Option<TableCellBorder>,
    inside_h: Option<TableCellBorder>,
    inside_v: Option<TableCellBorder>,
    tr2bl: Option<TableCellBorder>,
    tl2br: Option<TableCellBorder>,
}

impl Default for TableCellBorders {
    fn default() -> TableCellBorders {
        TableCellBorders {
            top: Some(TableCellBorder::new(TableCellBorderPosition::Top)),
            left: Some(TableCellBorder::new(TableCellBorderPosition::Left)),
            bottom: Some(TableCellBorder::new(TableCellBorderPosition::Bottom)),
            right: Some(TableCellBorder::new(TableCellBorderPosition::Right)),
            inside_h: Some(TableCellBorder::new(TableCellBorderPosition::InsideH)),
            inside_v: Some(TableCellBorder::new(TableCellBorderPosition::InsideV)),
            tr2bl: None,
            tl2br: None,
        }
    }
}

impl TableCellBorders {
    pub fn new() -> TableCellBorders {
        Default::default()
    }

    pub fn with_empty() -> TableCellBorders {
        TableCellBorders {
            top: None,
            left: None,
            bottom: None,
            right: None,
            inside_h: None,
            inside_v: None,
            tr2bl: None,
  