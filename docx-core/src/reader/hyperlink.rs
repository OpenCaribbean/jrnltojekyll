use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use super::*;

use super::attributes::*;

impl ElementReader for Hyperlink {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut rid: Option<String> = read(attrs, "id");
        let mut anchor: Option<String> = read(attrs, "anchor");
        let history: Option<String> = read(attrs, "history");
        let mut link = Hyperlink {
            link: if anchor.is_some() {
                HyperlinkData::Anchor {
                    anchor: anchor.take().unwrap(),
                }
            } else {
                HyperlinkData::External {
                    rid: rid.take().unwrap_or_default(),
                    path: String::default(), // not used
                }
            },
            history: history.map(|h| usize::from_str(&h).unwrap_or(1)),
            children: vec![],
        };

        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();

                    match e {
                        XMLElement::Run => {
            