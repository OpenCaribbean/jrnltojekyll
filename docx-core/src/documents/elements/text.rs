use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::Deserialize;

use crate::documents::BuildXML;
use crate::escape::escape;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Text {
    pub text: String,
    pub preserve_space: bool,
}

impl Text {
    pub fn new(text: impl Into<String>) -> Text {
        Text {
            text: escape(&text.into()),
            preserve_space: true,
        }
    }

    pub(crate) fn without_escape(text: impl Into<String>) -> Text {
        Text {
            text: text.into(),
            preserve_space: true,
        }
    }
}

impl BuildXML for Text {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new().text(&self.text, true).build()
    }
}

impl Serialize for Te