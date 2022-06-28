import { LevelJSON } from "./json";
import {
  createDefaultParagraphProperty,
  ParagraphProperty,
  SpecialIndentKind,
} from "./paragraph-property";
import { RunFonts, RunProperty } from "./run";

export type LevelSuffixType = "nothing" | "tab" | "space";

export class Level {
  id: number;
  start: number;
  format: string;
  text: string;
  jc: string;
  paragraphProperty: ParagraphProperty = createDefaultParagraphProperty();
  runProperty: R