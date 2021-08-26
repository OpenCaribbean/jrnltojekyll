use std::io::Read;
use xml::reader::{EventReader, XmlEvent};

use super::*;
use crate::reader::{FromXML, ReaderError};

use std::str::FromStr;

impl FromXML for Settings {
    fn from_xml<R: Read>(reader: R) -> Result<Self, ReaderError> {
        let mut parser = EventReader::new(reader);
        let mut settings = Self::default();

        loop {
            let e = parser.next();
            match e {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    match e {
                        XMLElement::DocId => {
                            for a in attributes {
                                if let Some(prefix) = a.name.prefix {
                                    let local_name = &a.name.local_name;
                                    // Ignore w14:val
                                    if local_name == "val" && prefix == "w15" {
                                        settings = settings.doc_id(
                                            &a.value.to_owned().replace("{", "").replace("}", ""),
                                        );
       