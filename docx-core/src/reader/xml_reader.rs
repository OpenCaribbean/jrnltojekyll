use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::io::Read;
use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use crate::documents::BuildXML;
use crate::reader::{FromXML, ReaderError};
use crate::xml_builder::*;

use super::*;

pub fn read_content_type(attrs: &[OwnedAttribute]) -> Option<String> {
    for a in attrs {
        let local_name = &a.name.local_name;
        if local_name == "contentType" {
            return Some(a.value.to_owned());
        }
    }
    None
}

pub fn read_xml(xml: &str) -> Result<Docx, ReaderError> {
    let mut parser = EventReader::new(xml.as_bytes());
    // let rels =             Rels::from_xml(&data[..])?
    for e in parser {
        let mut rels = Rels::new();
        match e {
            Ok(XmlEvent::StartElement {
               