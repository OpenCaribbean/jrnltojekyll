use crate::documents::{BuildXML, Level};
use crate::xml_builder::*;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AbstractNumbering {
    id: usize,
    style_link: Option<String>,
    num_style_link: Opti