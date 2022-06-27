import { ParagraphJSON } from "./paragraph";
import { BorderJSON } from "./border";
import { HeightRule } from "../table-row";
import { TextDirectionType } from "../table-cell";
import { ShadingJSON } from "./shading";
import { TableLayoutType } from "../table";
import { DeleteJSONData, InsertJSONData } from "..";

export type TableCellChildJSON = ParagraphJSON | TableJSON;

export type WidthType = "dxa" | "auto" | "pct" | "nil";

export { TextDirectionType } from "../table-cell";

export { Height