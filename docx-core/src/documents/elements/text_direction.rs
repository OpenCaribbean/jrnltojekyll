use serde::{Serialize, Serializer};

use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq)]
pub struct TextDirection {
    val: TextDirectionTy