use std::fmt;
use std::str::FromStr;
use wasm_bindgen::prelude::*;

use super::errors;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum VertAlignType {
    Baseline,
    SuperScript,
    SubScript,
    Unsupported,
}

// <xsd:enumeration value="baseline"/>
// <xsd:enumeration value="superscript"/>
// <xsd:enumeration value="subscript"/>
impl fmt::Display for VertAlignType