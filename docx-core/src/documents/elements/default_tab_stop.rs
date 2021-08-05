use crate::documents::BuildXML;
use crate::xml_builder::*;

use serde::{Serialize, Serializer};

#[derive(Debug, Clone, PartialEq)]
pub struct DefaultTabStop {
    val: usize,
}

impl DefaultTabStop {
    pub fn new(val: usize) -> DefaultTabStop {
        DefaultTabStop { val }
    }
}

impl BuildXML for DefaultTabStop {
    fn bu