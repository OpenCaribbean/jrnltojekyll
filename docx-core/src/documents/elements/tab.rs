use serde::{Deserialize, Serialize};

use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Default)]
pub struct Tab {
    pub val: Option<TabValueType>,
    pub leader: Option<TabLeaderType>,
    pub pos: Option<usize>,
}

impl Tab {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn val(mut self, v: TabValueType) -> Self {
        self.val = Some(v);
        self
    }

    pub fn leader(mut self, v: TabLeaderType) -> Self {
        self.lead