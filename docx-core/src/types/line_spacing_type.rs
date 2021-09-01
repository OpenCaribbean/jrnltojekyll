use crate::types::errors;
use crate::TypeError;
use serde::*;
use std::str::FromStr;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deser