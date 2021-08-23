#![allow(clippy::single_match)]

use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use crate::reader::*;

impl ElementReader for FontScheme {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut fs = FontScheme::new();
        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    match name.prefix.as_deref() {
                        Some("a") => {
                            let e = AXMLElement::from_str(&name.local_name).unwrap();
                            match e {
                                AXMLElement::MajorFont => {
                 