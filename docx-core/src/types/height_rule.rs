use std::fmt;
use wasm_bindgen::prelude::*;

use serde::Serialize;

use super::errors;
use std::str::FromStr;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum HeightRule {
    Auto,
    AtLeast,
    Exact,
}

impl Default for HeightRule {
    fn default() -> Self {
        Self::AtLeast
    }
}

impl fmt::Display for HeightRule {
    fn 