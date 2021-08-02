use super::Comment;
use crate::documents::BuildXML;
use crate::xml_builder::*;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Comments {
    pub(crate) comments: Vec<Comment>,
}

impl Comments {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn inner(&self) -> &[Comment] {
        &self.comments
    }

    pub fn into_inner(self) -> Vec<Comment> {
        self.comments
    }

    pub(crate) fn add_comments(&mut self, comments: Vec<Comment>) {
        self.comments = comments;
    }
}

impl Default for Comments {
    fn default() -> Self {
        Self { comments: vec![] }
    }
}

impl BuildXML for Comments {
    fn build(&self) -> Vec<u8> {
        let mut b = XMLBuilder::new().declaration(Some(true)).open_comments();
        for c in &self.comments {
            b = b.add_child(c)
        }
        b.close().build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_comments() {
        let b = Comments::new().build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:comments xmlns:o="urn:schemas-microsoft-com:office:office" xmlns: