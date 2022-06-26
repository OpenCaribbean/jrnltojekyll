
import { Paragraph } from "./paragraph";
import { ParagraphProperty } from "./paragraph-property";
import { Table } from "./table";
import { TableOfContents } from "./table-of-contents";
import { RunFonts } from "./run";
import { AbstractNumbering } from "./abstract-numbering";
import { Numbering } from "./numbering";
import { BookmarkStart } from "./bookmark-start";
import { BookmarkEnd } from "./bookmark-end";
import { Settings } from "./settings";
import { DocProps } from "./doc-props";
import { Style } from "./style";
import { Styles } from "./styles";
import { WebExtension } from "./webextension";
import { Footer } from "./footer";
import { Header } from "./header";
import { build } from "./builder";

import {
  SectionProperty,
  PageMargin,
  PageOrientationType,
} from "./section-property";
import { DocGridType, DocxJSON } from "./json";

import * as wasm from "./pkg";
import { Level } from "./level";

export class Docx {
  children: (
    | Paragraph
    | Table
    | BookmarkStart
    | BookmarkEnd
    | TableOfContents
  )[] = [];

  hasNumberings = false;
  abstractNumberings: AbstractNumbering[] = [];
  numberings: Numbering[] = [];
  settings: Settings = new Settings();
  docProps: DocProps = new DocProps();
  sectionProperty: SectionProperty = new SectionProperty();
  _taskpanes: boolean = false;
  webextensions: WebExtension[] = [];
  customItems: { id: string; xml: string }[] = [];
  styles = new Styles();

  addTableOfContents(t: TableOfContents) {
    this.children.push(t);
    return this;
  }

  addStyle(s: Style) {
    this.styles.styles.push(s);
    return this;
  }

  addParagraph(p: Paragraph) {
    if (p.hasNumberings) {
      this.hasNumberings = true;
    }
    this.children.push(p);
    return this;
  }

  addBookmarkStart(id: number, name: string) {
    this.children.push(new BookmarkStart(id, name));
    return this;
  }

  addBookmarkEnd(id: number) {
    this.children.push(new BookmarkEnd(id));
    return this;
  }

  addTable(t: Table) {
    if (t.hasNumberings) {
      this.hasNumberings = true;
    }
    this.children.push(t);
    return this;
  }

  addAbstractNumbering(num: AbstractNumbering) {
    this.abstractNumberings.push(num);
    return this;
  }

  addNumbering(num: Numbering) {
    this.numberings.push(num);
    return this;
  }

  docId(id: string) {
    this.settings.docId(id);
    return this;
  }

  defaultTabStop(stop: number) {
    this.settings.defaultTabStop(stop);
    return this;
  }

  createdAt(date: string) {
    this.docProps.createdAt(date);
    return this;
  }

  customProperty(name: string, item: string) {
    this.docProps.customProperty(name, item);
    return this;
  }

  updatedAt(date: string) {
    this.docProps.updatedAt(date);
    return this;
  }

  addDocVar(name: string, val: string) {
    this.settings.addDocVar(name, val);
    return this;
  }

  header(f: Header) {
    this.sectionProperty._header = f;
    return this;
  }

  firstHeader(h: Header) {
    this.sectionProperty._firstHeader = h;
    return this;
  }

  evenHeader(h: Header) {
    this.sectionProperty._evenHeader = h;
    return this;
  }

  footer(f: Footer) {
    this.sectionProperty._footer = f;
    return this;
  }

  firstFooter(f: Footer) {
    this.sectionProperty._firstFooter = f;
    return this;
  }

  evenFooter(f: Footer) {
    this.sectionProperty._evenFooter = f;
    return this;
  }

  pageSize(w: number, h: number) {
    this.sectionProperty.pageSize(w, h);
    return this;
  }

  pageMargin(margin: Partial<PageMargin>) {
    this.sectionProperty.pageMargin(margin);
    return this;
  }

  pageOrientation(o: PageOrientationType) {
    this.sectionProperty.pageOrientation(o);
    return this;
  }

  docGrid(type: DocGridType, linePitch?: number, charSpace?: number) {
    this.sectionProperty.docGrid(type, linePitch, charSpace);
    return this;
  }

  adjustLineHeightInTable() {
    this.settings.adjustLineHeightInTable();
    return this;
  }

  defaultSize(size: number) {
    this.styles.defaultSize(size);
    return this;
  }

  defaultFonts(fonts: RunFonts) {
    this.styles.defaultFonts(fonts);
    return this;
  }

  defaultSpacing(spacing: number) {
    this.styles.defaultSpacing(spacing);
    return this;
  }

  taskpanes() {
    this._taskpanes = true;
    return this;
  }

  webextension(e: WebExtension) {
    this.webextensions.push(e);
    return this;
  }

  addCustomItem(id: string, xml: string) {
    this.customItems.push({ id, xml });
    return this;
  }

  buildRunFonts = (fonts: RunFonts | undefined) => {
    let f = wasm.createRunFonts();
    if (fonts?._ascii) {
      f = f.ascii(fonts._ascii);
    }
    if (fonts?._hiAnsi) {
      f = f.hi_ansi(fonts._hiAnsi);
    }
    if (fonts?._cs) {
      f = f.cs(fonts._cs);
    }
    if (fonts?._eastAsia) {
      f = f.east_asia(fonts._eastAsia);
    }
    return f;
  };

  buildLineSpacing(p: ParagraphProperty): wasm.LineSpacing | null {
    const { lineSpacing } = p;
    if (lineSpacing == null) return null;
    let kind;
    switch (lineSpacing._lineRule) {
      case "atLeast": {
        kind = wasm.LineSpacingType.AtLeast;
        break;
      }
      case "auto": {
        kind = wasm.LineSpacingType.Auto;
        break;
      }
      case "exact": {
        kind = wasm.LineSpacingType.Exact;
        break;
      }
    }
    let spacing = wasm.createLineSpacing();
    if (lineSpacing._before != null) {
      spacing = spacing.before(lineSpacing._before);
    }

    if (lineSpacing._after != null) {
      spacing = spacing.after(lineSpacing._after);
    }

    if (lineSpacing._beforeLines != null) {
      spacing = spacing.before_lines(lineSpacing._beforeLines);
    }

    if (lineSpacing._afterLines != null) {
      spacing = spacing.after_lines(lineSpacing._afterLines);
    }

    if (lineSpacing._line != null) {
      spacing = spacing.line(lineSpacing._line);
    }

    if (kind != null) {
      spacing = spacing.line_rule(kind);
    }
    return spacing;
  }

  buildLevel(l: Level) {
    let level = wasm.createLevel(l.id, l.start, l.format, l.text, l.jc);