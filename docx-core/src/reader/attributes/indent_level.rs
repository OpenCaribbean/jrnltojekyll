use std::str::FromStr;

use xml::attribute::OwnedAttribute;

use super::super::errors::*;

pub fn read_indent_level(attrs: &[OwnedAttribute]) -> Result<usize, ReaderErro