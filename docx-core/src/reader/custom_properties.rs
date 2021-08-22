use std::io::Read;
use std::str::FromStr;

use xml::reader::{EventReader, XmlEvent};

use super::*;

impl FromXML for CustomProps {
    fn from_xml<R: Read>(reader: R) -> Result<Self, ReaderError> {
        let mut r = EventReader::new(reader);
        // TODO: Fow now, support only string.
        let mut props = CustomProps::new();
        loop {
            let e = r.next();
            match e {
    