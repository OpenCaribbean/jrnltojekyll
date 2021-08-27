use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

use super::*;
use crate::types::*;

impl ElementReader for Table {
    fn read<R: Read>(r: &mut EventReader<R>, _: &[OwnedAttribute]) -> Result<Self, ReaderError> {
        let mut t = Table::without_borders(vec![]);
        let mut grid_col: Vec<usize> = vec![];
        loop {
            let e = r.next();
            match e {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();

                    ignore::ignore_element(e.clone(), XMLElement::TablePropertyChange, r);
                    ignore::ignore_element(e.clone(), XMLElement::TableGridChange, r);

                    match e {
                        XMLElement::TableRow => {
                            t = t.add_row(TableRow::read(r, &attributes)?);
                            continue;
                        }
      