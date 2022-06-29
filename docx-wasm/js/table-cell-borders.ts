import { TableCellBorderPosition, TableCellBorder } from "./table-cell-border";

export type PositionKeys =
  | "top"
  | "left"
  | "bottom"
  | "right"
  | "insideH"
  | "insideV"
  | "tl2br"
  | "tr2bl";

export class TableCellBorders {
  top: TableCellBorder | null = new TableCellBorder("top");
  left: TableCellBorder | null = new TableCellBorder("left");
  bottom: TableCellBorder | null = new TableCellBorder("bottom");
  right: TableCellBorder | null = new TableCellBorder("right");
  insideH: TableCellBorder | null = new TableCellBorder("insideH");
  insideV: TableCellBorder | null = new TableCellBorder("insideV");
  tl2br: TableCellBorder | null = null;
  tr2bl: TableCellBorder | null = null;

  set(border: TableCellBorder) {
    switch (border.position) {
      case "top":
        this.top = border;
      case "left":
        this.left = border;
      case "bottom":
        this.bottom = border;
      case "right":
        this.right = border;
      case "insideH":
        this.insideH = border;
      case "insideV":
        this.insideV = border;
      case "tl2br":
        this.tl2br = border;
      case "tr2bl":
        this.tr2bl = border;
    }
    return this;
  }

  clear(position: TableCellBorderPosition) {
    let nil = new TableCellBorder(position).border_type("nil");
    switch (position) {
      case "top":
        this.top = nil;
      case "left":
        this.left = nil;
      case "bottom":
        this.bottom = nil;
      case "right":
        this.right = nil;
      case "insideH":
        this.insideH = n