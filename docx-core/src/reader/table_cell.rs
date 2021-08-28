use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use super::*;

impl ElementReader for TableCell {
    fn read<R: Read>(r: &mut EventReader<R>, _: &[OwnedAttribute]) -> Result<Self, ReaderError> {
        let mut cell = TableCell::new();
        loop {
            let e = r.next();

            match e {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    match e {
                        XMLElement::Paragraph => {
                            let p = Paragraph::read(r, &attributes)?;
                            cell = cell.add_paragraph(p);
                            continue;
                        }
                        XMLElement::TableCellProperty => {
                            if let Ok(p) = TableCellProperty::read(r, &attributes) {
                                cell.property = p;
                            }
                            continue;
                        }
                        XMLElement::Table => {
                            if let Ok(table) = Table::read(r, &attributes) {
                                cell = cell.add_table(table)
                            }
                        }
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if e == XMLElement::TableCell {
                     