use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;

use super::*;
use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Footer {
    pub has_numbering: bool,
    pub children: Vec<FooterChild>,
}

impl Footer {
    pub fn new() -> Footer {
        Default::default()
    }

    pub fn add_paragraph(mut self, p: Paragraph) -> Self {
        if p.has_numbering {
            self.has_numbering = true
        }
        self.children.push(FooterChild::Paragraph(Box::new(p)));
        self
    }

    pub fn add_table(mut self, t: Table) -> Self {
        if t.has_numbering {
            self.has_numbering = true
        }
        self.children.push(FooterChild::Table(Box::new(t)));
        self
    }
}

#[derive(Debug, Clone, Partial