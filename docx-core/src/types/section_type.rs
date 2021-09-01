use serde::{Deserialize, Serialize};

//
// Please see CT_SectType
//
// <xsd:enumeration value="nextPage"/>
// <xsd:enumeration value="nextColumn"/>
// <xsd:enumeration value="continuous"/>
// <xsd:enumeration value="evenPage"/>
// <xsd:enumeration value="oddPage"/>

use std::fmt;
use std::str::FromStr;
use wasm_bindgen::prelude::*;

use super::errors;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum SectionType {
    NextPage,
    NextColumn,
    Continuous,
    EvenPage,
    OddPage,
}

impl fmt::Display for SectionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SectionType::NextPage => wri