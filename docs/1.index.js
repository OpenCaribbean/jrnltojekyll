(window["webpackJsonp"] = window["webpackJsonp"] || []).push([
  [1],
  {
    /***/ "./pkg/index.js":
      /*!**********************!*\
  !*** ./pkg/index.js ***!
  \**********************/
      /*! exports provided: createRun, createComment, createTableCell, createTableRow, createDocx, createLevel, createInsert, createTable, createNumbering, createDelete, createParagraph, TableAlignmentType, SpecialIndentKind, VMergeType, BreakType, FontPitchType, WidthType, BorderType, AlignmentType, StyleType, Comment, Delete, Docx, Insert, Level, Numbering, Paragraph, Run, Table, TableCell, TableRow, __wbindgen_string_new, __wbindgen_object_drop_ref, __wbindgen_throw, __wbindgen_rethrow */
      /***/ function(module, __webpack_exports__, __webpack_require__) {
        "use strict";
        eval(
          '__webpack_require__.r(__webpack_exports__);\n/* WEBPACK VAR INJECTION */(function(TextEncoder, TextDecoder) {/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "createRun", function() { return createRun; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "createComment", function() { return createComment; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "createTableCell", function() { return createTableCell; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "createTableRow", function() { return createTableRow; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "createDocx", function() { return createDocx; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "createLevel", function() { return createLevel; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "createInsert", function() { return createInsert; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "createTable", function() { return createTable; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "createNumbering", function() { return createNumbering; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "createDelete", function() { return createDelete; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "createParagraph", function() { return createParagraph; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "TableAlignmentType", function() { return TableAlignmentType; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "SpecialIndentKind", function() { return SpecialIndentKind; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "VMergeType", function() { return VMergeType; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "BreakType", function() { return BreakType; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "FontPitchType", function() { return FontPitchType; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "WidthType", function() { return WidthType; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "BorderType", function() { return BorderType; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "AlignmentType", function() { return AlignmentType; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "StyleType", function() { return StyleType; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "Comment", function() { return Comment; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "Delete", function() { return Delete; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "Docx", function() { return Docx; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "Insert", function() { return Insert; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "Level", function() { return Level; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "Numbering", function() { return Numbering; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "Paragraph", function() { return Paragraph; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "Run", function() { return Run; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "Table", function() { return Table; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "TableCell", function() { return TableCell; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "TableRow", function() { return TableRow; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbindgen_string_new", function() { return __wbindgen_string_new; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbindgen_object_drop_ref", function() { return __wbindgen_object_drop_ref; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbindgen_throw", function() { return __wbindgen_throw; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbindgen_rethrow", function() { return __wbindgen_rethrow; });\n/* harmony import */ var _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./index_bg.wasm */ "./pkg/index_bg.wasm");\n\n\n/**\n* @returns {Run}\n*/\nfunction createRun() {\n    const ret = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["createRun"]();\n    return Run.__wrap(ret);\n}\n\nfunction _assertNum(n) {\n    if (typeof(n) !== \'number\') throw new Error(\'expected a number argument\');\n}\n\nlet WASM_VECTOR_LEN = 0;\n\nlet cachedTextEncoder = new TextEncoder(\'utf-8\');\n\nconst encodeString = (typeof cachedTextEncoder.encodeInto === \'function\'\n    ? function (arg, view) {\n    return cachedTextEncoder.encodeInto(arg, view);\n}\n    : function (arg, view) {\n    const buf = cachedTextEncoder.encode(arg);\n    view.set(buf);\n    return {\n        read: arg.length,\n        written: buf.length\n    };\n});\n\nlet cachegetUint8Memory = null;\nfunction getUint8Memory() {\n    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["memory"].buffer) {\n        cachegetUint8Memory = new Uint8Array(_index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["memory"].buffer);\n    }\n    return cachegetUint8Memory;\n}\n\nfunction passStringToWasm(arg) {\n\n    if (typeof(arg) !== \'string\') throw new Error(\'expected a string argument\');\n\n    let len = arg.length;\n    let ptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_malloc"](len);\n\n    const mem = getUint8Memory();\n\n    let offset = 0;\n\n    for (; offset < len; offset++) {\n        const code = arg.charCodeAt(offset);\n    