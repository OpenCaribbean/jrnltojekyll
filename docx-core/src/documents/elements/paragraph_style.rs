use serde::{Serialize, Serializer};

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ParagraphStyle {
    pub val: String,
}

impl Default for ParagraphStyle {
    fn default() -> Self {
        ParagraphStyle {
            val: "Normal".to_owned(),
        }
    }
}

// 17.9.23
// pStyle (Paragraph Style's Associated Numbering Level)
// This element specifies the name of a paragraph style which shall automatically this numbering level when
// applied to the contents of the document. When a paragraph style is defined to include a numbering definition,
// any numbering level defined by the numPr element (§17.3.1.19) shall be ignored, and instead this element shall
// specify the