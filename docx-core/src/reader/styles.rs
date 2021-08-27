use std::io::Read;
use xml::reader::{EventReader, XmlEvent};

use super::*;
use crate::reader::{FromXML, ReaderError};

use std::str::FromStr;

impl FromXML for Styles {
    fn from_xml<R: Read>(reader: R) -> Result<Self, ReaderError> {
        let mut parser = EventReader::new(reader);
        let mut styles = Self::default();
        loop {
            let e = parser.next();
            match e {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    match e {
                        XMLElement::Style => {
                            if let Ok(s) = Style::read(&mut parser, &attributes) {
                                styles = styles.add_style(s);
                            }
                            continue;
                        }
                        XMLElement::DocDefaults => {
                            if let Ok(d) = DocDefaults::read(&mut parser, &attributes) {
                                styles = styles.doc_defaults(d);
                            }
                            continue;
                        }
                        _ => {}
                    }
                }
   