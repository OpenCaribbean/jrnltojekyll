
use super::*;
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;

use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Run {
    pub run_property: RunProperty,
    pub children: Vec<RunChild>,
}

impl Default for Run {
    fn default() -> Self {
        let run_property = RunProperty::new();
        Self {
            run_property,
            children: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RunChild {
    Text(Text),
    DeleteText(DeleteText),
    Tab(Tab),
    Break(Break),
    Drawing(Box<Drawing>),
    Shape(Box<Shape>),
    CommentStart(Box<CommentRangeStart>),
    CommentEnd(CommentRangeEnd),
    FieldChar(FieldChar),
    InstrText(Box<InstrText>),
    DeleteInstrText(Box<DeleteInstrText>),
    // For reader
    InstrTextString(String),
}

impl Serialize for RunChild {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            RunChild::Text(ref s) => {
                let mut t = serializer.serialize_struct("Text", 2)?;
                t.serialize_field("type", "text")?;
                t.serialize_field("data", s)?;
                t.end()
            }
            RunChild::DeleteText(ref s) => {
                let mut t = serializer.serialize_struct("DeleteText", 2)?;
                t.serialize_field("type", "deleteText")?;
                t.serialize_field("data", s)?;
                t.end()
            }
            RunChild::Tab(_) => {
                let mut t = serializer.serialize_struct("Tab", 1)?;
                t.serialize_field("type", "tab")?;
                t.end()
            }
            RunChild::Break(ref s) => {
                let mut t = serializer.serialize_struct("Break", 2)?;
                t.serialize_field("type", "break")?;
                t.serialize_field("data", s)?;
                t.end()
            }
            RunChild::Drawing(ref s) => {
                let mut t = serializer.serialize_struct("Drawing", 2)?;
                t.serialize_field("type", "drawing")?;
                t.serialize_field("data", s)?;
                t.end()
            }
            RunChild::Shape(ref s) => {
                let mut t = serializer.serialize_struct("Shape", 2)?;
                t.serialize_field("type", "shape")?;
                t.serialize_field("data", s)?;
                t.end()
            }
            RunChild::CommentStart(ref r) => {
                let mut t = serializer.serialize_struct("CommentRangeStart", 2)?;
                t.serialize_field("type", "commentRangeStart")?;
                t.serialize_field("data", r)?;
                t.end()
            }
            RunChild::CommentEnd(ref r) => {
                let mut t = serializer.serialize_struct("CommentRangeEnd", 2)?;
                t.serialize_field("type", "commentRangeEnd")?;
                t.serialize_field("data", r)?;
                t.end()
            }
            RunChild::FieldChar(ref f) => {
                let mut t = serializer.serialize_struct("FieldChar", 2)?;
                t.serialize_field("type", "fieldChar")?;
                t.serialize_field("data", f)?;
                t.end()
            }
            RunChild::InstrText(ref i) => {
                let mut t = serializer.serialize_struct("InstrText", 2)?;
                t.serialize_field("type", "instrText")?;
                t.serialize_field("data", i)?;
                t.end()
            }
            RunChild::DeleteInstrText(ref i) => {
                let mut t = serializer.serialize_struct("DeleteInstrText", 2)?;
                t.serialize_field("type", "deleteInstrText")?;
                t.serialize_field("data", i)?;
                t.end()
            }
            RunChild::InstrTextString(ref i) => {
                let mut t = serializer.serialize_struct("InstrTextString", 2)?;
                t.serialize_field("type", "instrTextString")?;
                t.serialize_field("data", i)?;
                t.end()
            }
        }
    }
}

impl Run {
    pub fn new() -> Run {
        Run {
            ..Default::default()
        }
    }

    pub fn add_text(mut self, text: impl Into<String>) -> Run {
        self.children
            .push(RunChild::Text(Text::new(text.into().replace('\n', ""))));
        self
    }

    pub(crate) fn add_text_without_escape(mut self, text: impl Into<String>) -> Run {
        self.children.push(RunChild::Text(Text::without_escape(
            text.into().replace('\n', ""),
        )));
        self
    }

    pub fn add_delete_text(mut self, text: impl Into<String>) -> Run {
        self.children.push(RunChild::DeleteText(DeleteText::new(
            text.into().replace('\n', ""),
        )));
        self
    }

    pub(crate) fn add_delete_text_without_escape(mut self, text: impl Into<String>) -> Run {
        self.children
            .push(RunChild::DeleteText(DeleteText::without_escape(
                text.into().replace('\n', ""),
            )));
        self
    }

    pub fn add_field_char(mut self, t: crate::types::FieldCharType, dirty: bool) -> Run {
        let mut f = FieldChar::new(t);
        if dirty {
            f = f.dirty();
        };
        self.children.push(RunChild::FieldChar(f));
        self
    }

    pub fn add_instr_text(mut self, i: InstrText) -> Run {
        self.children.push(RunChild::InstrText(Box::new(i)));
        self
    }

    pub fn add_delete_instr_text(mut self, i: DeleteInstrText) -> Run {
        self.children.push(RunChild::DeleteInstrText(Box::new(i)));
        self
    }

    pub fn add_tab(mut self) -> Run {
        self.children.push(RunChild::Tab(Tab::new()));
        self
    }

    pub fn add_image(mut self, pic: Pic) -> Run {
        self.children
            .push(RunChild::Drawing(Box::new(Drawing::new().pic(pic))));
        self
    }

    pub(crate) fn add_drawing(mut self, d: Drawing) -> Run {
        self.children.push(RunChild::Drawing(Box::new(d)));
        self
    }

    // For now reader only
    //    pub(crate) fn add_shape(mut self, d: Shape) -> Run {
    //        self.children.push(RunChild::Shape(Box::new(d)));
    //        self
    //    }

    pub fn add_break(mut self, break_type: BreakType) -> Run {
        self.children.push(RunChild::Break(Break::new(break_type)));
        self
    }

    pub fn style(mut self, style_id: &str) -> Self {
        self.run_property = self.run_property.style(style_id);
        self
    }

    pub fn size(mut self, size: usize) -> Run {
        self.run_property = self.run_property.size(size);
        self
    }

    pub fn character_spacing(mut self, v: i32) -> Run {
        self.run_property = self.run_property.spacing(v);
        self
    }

    pub fn color(mut self, color: impl Into<String>) -> Run {
        self.run_property = self.run_property.color(color);
        self
    }
