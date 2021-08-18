use serde::{Deserialize, Serialize};

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Rels {
    pub rels: Vec<(String, String, String)>,
}

impl Rels {
    pub fn new() -> Rels {
        Default::default()
    }

    pub fn set_default(mut self) -> Self {
        self.rels.push((
            "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties"
                .to_owned(),
            "rId1".to_owned(),
            "docProps/core.xml".to_owned(),
        ));
        self.rels.push(
            ("http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties".to_owned(),
            "rId2".to_owned(), "docProps/app.xml".to_owned()),
        );
        self.rels.push((
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument"
                .to_owned(),
            "rId3".to_owned(),
            "word/document.xml".to_owned(),
        ));
        self.rels.push((
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/custom-properties"
                .to_owned(),
            "rId4".to_owned(),
            "docProps/custom.xml".to_owned(),
        ));
        self
    }

    pub fn add_taskpanes_rel(mut self) -> Self {
        self = self.add_rel(
            "http://schemas.microsoft.com/office/2011/relationships/webextensiontaskpanes",
            "word/webextensions/taskpanes.xml",
        );
        self
    }

    pub fn add_rel(mut self, rel_type: impl Into<String>, target: impl Into<String>) -> Self {
        self.rels.push((
            rel_type.into(),
            format!("rId{}", self.rels.len() + 1),
            target.into(),
        ));
        self
    }

    pub fn find_target(&self, rel_type: &str) -> Option<&(String, String, String)> {
        self.rels.iter().find(|rel| rel.0 == rel_type)
    }
}

impl Default for Rels {
    fn default() -> Self {
        Rels { rels: Vec::new() }
    }
}

impl BuildXML for Rels {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let mut b = b
            .declaration(No