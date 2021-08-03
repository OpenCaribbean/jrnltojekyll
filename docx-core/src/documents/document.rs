
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;

use super::*;
use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    pub children: Vec<DocumentChild>,
    pub section_property: SectionProperty,
    pub has_numbering: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DocumentChild {
    Paragraph(Box<Paragraph>),
    Table(Box<Table>),
    BookmarkStart(BookmarkStart),
    BookmarkEnd(BookmarkEnd),
    CommentStart(Box<CommentRangeStart>),
    CommentEnd(CommentRangeEnd),
    StructuredDataTag(Box<StructuredDataTag>),
    TableOfContents(Box<TableOfContents>),
}

impl Serialize for DocumentChild {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            DocumentChild::Paragraph(ref p) => {
                let mut t = serializer.serialize_struct("Paragraph", 2)?;
                t.serialize_field("type", "paragraph")?;
                t.serialize_field("data", p)?;
                t.end()
            }
            DocumentChild::Table(ref c) => {
                let mut t = serializer.serialize_struct("Table", 2)?;
                t.serialize_field("type", "table")?;
                t.serialize_field("data", c)?;
                t.end()
            }
            DocumentChild::BookmarkStart(ref c) => {
                let mut t = serializer.serialize_struct("BookmarkStart", 2)?;
                t.serialize_field("type", "bookmarkStart")?;
                t.serialize_field("data", c)?;
                t.end()
            }
            DocumentChild::BookmarkEnd(ref c) => {
                let mut t = serializer.serialize_struct("BookmarkEnd", 2)?;
                t.serialize_field("type", "bookmarkEnd")?;
                t.serialize_field("data", c)?;
                t.end()
            }
            DocumentChild::CommentStart(ref r) => {
                let mut t = serializer.serialize_struct("CommentRangeStart", 2)?;
                t.serialize_field("type", "commentRangeStart")?;
                t.serialize_field("data", r)?;
                t.end()
            }
            DocumentChild::CommentEnd(ref r) => {
                let mut t = serializer.serialize_struct("CommentRangeEnd", 2)?;
                t.serialize_field("type", "commentRangeEnd")?;
                t.serialize_field("data", r)?;
                t.end()
            }
            DocumentChild::StructuredDataTag(ref r) => {
                let mut t = serializer.serialize_struct("StructuredDataTag", 2)?;
                t.serialize_field("type", "structuredDataTag")?;
                t.serialize_field("data", r)?;
                t.end()
            }
            DocumentChild::TableOfContents(ref r) => {
                let mut t = serializer.serialize_struct("TableOfContents", 2)?;
                t.serialize_field("type", "tableOfContents")?;
                t.serialize_field("data", r)?;
                t.end()
            }
        }
    }
}

impl Default for Document {
    fn default() -> Self {
        Self {
            children: Vec::new(),
            section_property: SectionProperty::new(),
            has_numbering: false,
        }
    }
}

impl Document {
    pub fn new() -> Document {
        Default::default()
    }

    pub fn add_paragraph(mut self, p: Paragraph) -> Self {
        if p.has_numbering {
            self.has_numbering = true
        }
        self.children.push(DocumentChild::Paragraph(Box::new(p)));
        self
    }

    pub fn add_table(mut self, t: Table) -> Self {
        if t.has_numbering {
            self.has_numbering = true
        }
        self.children.push(DocumentChild::Table(Box::new(t)));
        self
    }

    pub fn add_bookmark_start(mut self, id: usize, name: impl Into<String>) -> Self {
        self.children
            .push(DocumentChild::BookmarkStart(BookmarkStart::new(id, name)));
        self
    }

    pub fn add_bookmark_end(mut self, id: usize) -> Self {
        self.children
            .push(DocumentChild::BookmarkEnd(BookmarkEnd::new(id)));
        self
    }

    pub fn add_comment_start(mut self, comment: Comment) -> Self {
        self.children.push(DocumentChild::CommentStart(Box::new(
            CommentRangeStart::new(comment),
        )));
        self
    }

    pub fn add_comment_end(mut self, id: usize) -> Self {
        self.children
            .push(DocumentChild::CommentEnd(CommentRangeEnd::new(id)));
        self
    }

    pub fn page_size(mut self, size: PageSize) -> Self {
        self.section_property = self.section_property.page_size(size);
        self
    }