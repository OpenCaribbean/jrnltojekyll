use serde::Serialize;

use super::Comment;
use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct CommentRangeStart {
    pub id: usize,
    pub comment: Comment,
}

impl CommentRangeStart {
    pub fn new(comment: Comment) -> CommentRangeStart {
        CommentRangeStart {
            id: comment.id(),
            comment,
        }
    }

    pub(crate) fn comment(&mut self, comment: Comment) {
        self.comment = comment;
    }

    pub(crate) fn get_com