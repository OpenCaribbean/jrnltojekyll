use serde::Serialize;

use super::*;
use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug, Clone, Serialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RunProperty {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<RunStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sz: Option<Sz>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sz_cs: Option<SzCs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlight: Option<Highlight>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vert_align: Option<VertAlign>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underline: Option<Underline>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bold: Option<Bold>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bold_cs: Option<BoldCs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub italic: Option<Italic>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub italic_cs: Option<ItalicCs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vanish: Option<Vanish>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec_vanish: Option<SpecVanish>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_spacing: Option<CharacterSpacing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fonts: Option<RunFonts>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_border: Option<TextBorder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub del: Option<Delete>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ins: Option<Insert>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strike: Option<Strike>,
}

impl RunProperty {
    pub fn new() -> RunProperty {
        Default::default()
    }

    pub fn style(mut self, style_id: &str) -> Self {
        self.style = Some(RunStyle::new(style_id));
        self
    }

    pub fn size(mut self, size: usize) -> RunProperty {
        self.sz = Some(Sz::new(size));
        self.sz_cs = Some(SzCs::new(size));
        self
    }

    pub fn spacing(mut self, spacing: i32) -> RunProperty {
        self.chara