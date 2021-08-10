use serde::Serialize;

use crate::documents::*;
use crate::xml_builder::*;

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParagraphPropertyChange {
    pub author: String,
    pub date: String,
    pub proper