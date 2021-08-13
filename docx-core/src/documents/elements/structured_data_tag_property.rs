use serde::Serialize;

use super::*;
use crate::documents::BuildXML;
// use crate::types::*;
use crate::xml_builder::*;

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StructuredDataTagProperty {
    pub run_property: RunProperty,
    pub data_binding: Option<DataBinding>,
    pub alias: Option<String>,
}

impl Default for StructuredDataTagProperty {
    fn default() -> Self {
        Self {
            run_property: RunProperty::new(),
            data_binding: None,
            alias: None,
        }
    }
}

impl StructuredDataTagProperty {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn data_binding(mut self, d: DataBinding) -> Self {
        self.data_binding = Some(d);
        self
    }

    pub fn alias(mut self, v: impl Into<String>) -> Self {
        self.alias = Some(v.into());
        self
    }
}

impl BuildXML for StructuredDataTagProperty {
    fn build(&self) -> Vec<u8> {
        let mut b = XMLBuilder::new()
            .open_structured_tag_property()
            .add_ch