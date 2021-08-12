use super::*;
use crate::documents::BuildXML;
use crate::xml_builder::*;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Section {
    property: SectionProperty,
}

impl Section {
    pub fn new() -> Section {
        Default::default()
    }
}

impl Default for Section {
    fn default() -> Self {
        Self {
            property: SectionProperty::new(),
        }
    }
}

impl BuildXML for Section {
    fn build(&self) -> Vec<u8> {
        let id = crate::generate_para_id();

        XMLBuilder::new()
            .open_paragraph(&id)
            .open_paragraph_property()
            .add_child(&self.property)
            .close()
            .close()
            .build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    