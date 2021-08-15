
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;

use crate::documents::*;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq)]
pub enum TocContent {
    Paragraph(Box<Paragraph>),
    Table(Box<Table>),
}

impl Serialize for TocContent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            TocContent::Paragraph(ref p) => {
                let mut t = serializer.serialize_struct("Paragraph", 2)?;
                t.serialize_field("type", "paragraph")?;
                t.serialize_field("data", p)?;
                t.end()
            }
            TocContent::Table(ref c) => {
                let mut t = serializer.serialize_struct("Table", 2)?;
                t.serialize_field("type", "table")?;
                t.serialize_field("data", c)?;
                t.end()
            }
        }
    }
}

#[derive(Serialize, Debug, Clone, PartialEq, Default)]
pub struct TableOfContentsReviewData {
    pub author: String,
    pub date: String,
}

// https://c-rex.net/projects/samples/ooxml/e1/Part4/OOXML_P4_DOCX_TOCTOC_topic_ID0ELZO1.html
// This struct is only used by writers
#[derive(Serialize, Debug, Clone, PartialEq, Default)]
pub struct TableOfContents {
    pub instr: InstrToC,
    pub items: Vec<TableOfContentsItem>,
    // don't use
    pub auto: bool,
    pub dirty: bool,
    pub alias: Option<String>,
    pub page_ref_placeholder: Option<String>,
    // it is inserted in before toc.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub before_contents: Vec<TocContent>,
    // it is inserted in after toc.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub after_contents: Vec<TocContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<TableOfContentsReviewData>,
}

impl TableOfContents {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_instr_text(s: &str) -> Self {
        let instr = InstrToC::with_instr_text(s);
        Self {
            instr,
            ..Self::default()
        }
    }

    pub fn heading_styles_range(mut self, start: usize, end: usize) -> Self {
        self.instr = self.instr.heading_styles_range(start, end);
        self