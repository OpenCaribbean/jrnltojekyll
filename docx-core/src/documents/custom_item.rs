use crate::documents::BuildXML;
use crate::{ParseXmlError, XmlDocument};
use serde::ser::SerializeSeq;
use serde::Serialize;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct CustomItem(XmlDocument);

impl FromStr for CustomItem {
    type Err = ParseXmlError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(CustomItem(XmlDocument::from_str(s)?))
 