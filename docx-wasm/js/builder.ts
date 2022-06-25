
import { Comment } from "./comment";
import { CommentEnd } from "./comment-end";
import { Paragraph } from "./paragraph";
import { Table } from "./table";
import { Run } from "./run";
import { Insert } from "./insert";
import { Delete } from "./delete";
import { BookmarkStart } from "./bookmark-start";
import { BookmarkEnd } from "./bookmark-end";
import { Hyperlink, convertHyperlinkType } from "./hyperlink";
import { setParagraphProperty } from "./paragraph-property";

import * as wasm from "./pkg";

type Child = | Paragraph
  | Table
  | Comment
  | Hyperlink;

function buildHyperlink(child: Hyperlink) {
  let hyperlink = wasm.createHyperlink(child.v, convertHyperlinkType(child));

  child.children.forEach((child) => {
    if (child instanceof Run) {
      const run = child.build();
      hyperlink = hyperlink.add_run(run);
    } else if (child instanceof Insert) {
      const insert = child.build();
      hyperlink = hyperlink.add_insert(insert);
    } else if (child instanceof Delete) {
      const del = child.build();
      hyperlink = hyperlink.add_delete(del);
    } else if (child instanceof BookmarkStart) {
      hyperlink = hyperlink.add_bookmark_start(child.id, child.name);
    } else if (child instanceof BookmarkEnd) {
      hyperlink = hyperlink.add_bookmark_end(child.id);
    } else if (child instanceof Comment) {
      hyperlink = hyperlink.add_comment_start(build(child));
    } else if (child instanceof CommentEnd) {
      hyperlink = hyperlink.add_comment_end(child.id);
    }
  });

  return hyperlink;
}

function buildParagraph(child: Paragraph) {
  let paragraph = wasm.createParagraph();
  child.children.forEach((child) => {
    if (child instanceof Run) {
      const run = child.build();
      paragraph = paragraph.add_run(run);
    } else if (child instanceof Insert) {
      const insert = child.build();
      paragraph = paragraph.add_insert(insert);
    } else if (child instanceof Delete) {
      const del = child.build();
      paragraph = paragraph.add_delete(del);
    } else if (child instanceof Hyperlink) {
      paragraph = paragraph.add_hyperlink(build(child));
    } else if (child instanceof BookmarkStart) {
      paragraph = paragraph.add_bookmark_start(child.id, child.name);
    } else if (child instanceof BookmarkEnd) {
      paragraph = paragraph.add_bookmark_end(child.id);
    } else if (child instanceof Comment) {
      const comment = build(child);
      paragraph = paragraph.add_comment_start(comment as wasm.Comment);
    } else if (child instanceof CommentEnd) {
      paragraph = paragraph.add_comment_end(child.id);
    }
  });

  paragraph = setParagraphProperty(paragraph, child.property);

  if (typeof child.property.styleId !== "undefined") {
    paragraph = paragraph.style(child.property.styleId);
  }

  if (child.property.runProperty.del) {
    paragraph = paragraph.delete(
      child.property.runProperty.del.author,
      child.property.runProperty.del.date
    );
  }

  if (child.property.runProperty.ins) {
    paragraph = paragraph.insert(
      child.property.runProperty.ins.author,
      child.property.runProperty.ins.date
    );
  }

  if (child.property.paragraphPropertyChange) {