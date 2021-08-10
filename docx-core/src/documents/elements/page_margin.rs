use crate::documents::BuildXML;
use crate::types::PageMargin;
use crate::xml_builder::*;

// These values were based on microsoft office word2019 windows edition.
// <w:pgMar w:top="1985" w:right="1701" w:bottom="1701" w:left="1701" w:header="851" w:footer="992" w:gutter="0"/>
impl Default for PageMargin {
    fn default() -> PageMargin {
        PageMargin {
            top: 1985,
            left: 1701,
            bottom: 1701,
            right: 1701,
            header: 851,
            footer: 992,
            gutter: 0,
        }
    }
}

impl PageMargin {
    pub fn new() -> PageMargin {
        Default::default()
    }

    pub fn top(self, v: i32) -> PageMargin {
        PageMargin { top: v, 