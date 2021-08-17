
use crate::documents::BuildXML;
use crate::xml_builder::*;

use serde::{Serialize, Serializer};

#[derive(Debug, Clone, PartialEq)]
pub struct Zoom {
    val: usize,
}

impl Zoom {
    pub fn new(val: usize) -> Zoom {
        Zoom { val }
    }
}

impl BuildXML for Zoom {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.zoom(&format!("{}", self.val)).build()
    }
}

impl Serialize for Zoom {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.val as u64)