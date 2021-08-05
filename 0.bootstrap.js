(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "../pkg/conch_analyzer.js":
/*!********************************!*\
  !*** ../pkg/conch_analyzer.js ***!
  \********************************/
/*! exports provided: cmd_histogram */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _conch_analyzer_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./conch_analyzer_bg.wasm */ \"../pkg/conch_analyzer_bg.wasm\");\n/* harmony import */ var _conch_analyzer_bg_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./conch_analyzer_bg.js */ \"../pkg/conch_analyzer_bg.js\");\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"cmd_histogram\", function() { return _conch_analyzer_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"cmd_histogram\"]; });\n\n\n\n\n//# sourceURL=webpack:///../pkg/conch_analyzer.js?");

/***/ }),

/***/ "../pkg/conch_analyzer_bg.js":
/*!***********************************!*\
  !*** ../pkg/conch_analyzer_bg.js ***!
  \***********************************/
/*! exports provided: cmd_histogram */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* WEBPACK VAR INJECTION */(function(module) {/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"cmd_histogram\", function() { return cmd_histogram; });\n/* harmony import */ var _conch_analyzer_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./conch_analyzer_bg.wasm */ \"../pkg/conch_analyzer_bg.wasm\");\n\n\nlet WASM_VECTOR_LEN = 0;\n\nlet cachegetUint8Memory0 = null;\nfunction getUint8Memory0() {\n    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== _conch_analyzer_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint8Memory0 = new Uint8Array(_conch_analyzer_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint8Memory0;\n}\n\nconst lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;\n\nlet cachedTextEncoder = new lTextEncoder('utf-8');\n\nconst encodeString = (typeof cachedTextEncoder.encodeInto === 'function'\n    ? function (arg, view) {\n    return cachedTextEncoder.encodeInto(arg, view);\n}\n    : function (arg, view) {\n    const buf = cachedTextEncoder.encode(arg);\n    view.set(buf);\n    return {\n        read: arg.length,\n        written: buf.length\n    };\n});\n\nfunction passStringToWasm0(arg, malloc, realloc) {\n\n    if (realloc === undefined) {\n        const buf = cachedTextEncoder.encode(arg);\n        const ptr = malloc(buf.length);\n        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);\n        WASM_VECTOR_LEN = buf.length;\n        return ptr;\n    }\n\n    let len = arg.length;\n    let ptr = malloc(len);\n\n    const mem = getUint8Memory0();\n\n    let offset = 0;\n\n    for (; offset < len; offset++) {\n        const code = arg.charCodeAt(offset);\n        if (code > 0x7F) break;\n        mem[ptr + offset] = code;\n    }\n\n    if (offset !== len) {\n        if (offset !== 0) {\n            arg = arg.slice(offset);\n        }\n        ptr = realloc(ptr, len, len = offset + arg.length * 3);\n        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);\n        const ret = encodeString(arg, view);\n\n        offset += ret.written;\n    }\n\n    WASM_VECTOR_LEN = offset;\n    return ptr;\n}\n\nlet cachegetInt32Memory0 = null;\nfunction getInt32Memory0() {\n    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== _conch_analyzer_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetInt32Memory0 = new Int32Array(_conch_analyzer_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetInt32Memory0;\n}\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nfunction getStringFromWasm0(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n/**\n* @param {string} s\n* @returns {string}\n*/\nfunction cmd_histogram(s) {\n    try {\n        const retptr = _conch_analyzer_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_add_to_stack_pointer\"](-16);\n        var ptr0 = passStringToWasm0(s, _conch_analyzer_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _conch_analyzer_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n        var len0 = WASM_VECTOR_LEN;\n        _conch_analyzer_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"cmd_histogram\"](retptr, ptr0, len0);\n        var r0 = getInt32Memory0()[retptr / 4 + 0];\n        var r1 = getInt32Memory0()[retptr / 4 + 1];\n        return getStringFromWasm0(r0, r1);\n    } finally {\n        _conch_analyzer_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_add_to_stack_pointer\"](16);\n        _conch_analyzer_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](r0, r1);\n    }\n}\n\n\n/* WEBPACK VAR INJECTION */}.call(this, __webpack_require__(/*! ./../www/node_modules/webpack/buildin/harmony-module.js */ \"./node_modules/webpack/buildin/harmony-module.js\")(module)))\n\n//# sourceURL=webpack:///../pkg/conch_analyzer_bg.js?");

/***/ }),

/***/ "../pkg/conch_analyzer_bg.wasm":
/*!*************************************!*\
  !*** ../pkg/conch_analyzer_bg.wasm ***!
  \*************************************/
/*! exports provided: memory, cmd_histogram, __wbindgen_add_to_stack_pointer, __wbindgen_malloc, __wbindgen_realloc, __wbindgen_free */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///../pkg/conch_analyzer_bg.wasm?");

/***/ }),

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var conch_analyzer__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! conch-analyzer */ \"../pkg/conch_analyzer.js\");\n\n\nlet nixos_installer_script = `#!/bin/sh\n\n# This script installs the Nix package manager on your system by\n# downloading a binary distribution and running its installer script\n# (which in turn creates and populates /nix).\n\n{ # Prevent execution if this script was only partially downloaded\noops() {\n    echo \"$0:\" \"$@\" >&2\n    exit 1\n}\n\numask 0022\n\ntmpDir=\"$(mktemp -d -t nix-binary-tarball-unpack.XXXXXXXXXX || \\\n          oops \"Can't create temporary directory for downloading the Nix binary tarball\")\"\ncleanup() {\n    rm -rf \"$tmpDir\"\n}\ntrap cleanup EXIT INT QUIT TERM\n\nrequire_util() {\n    command -v \"$1\" > /dev/null 2>&1 ||\n        oops \"you do not have '$1' installed, which I need to $2\"\n}\n\ncase \"$(uname -s).$(uname -m)\" in\n    Linux.x86_64) system=x86_64-linux; hash=5260ea5bb30bf3a7d023869644f28fae8b510b91e50f031cf72085f229dc769f;;\n    Linux.i?86) system=i686-linux; hash=16cda1322ea63effa6f8ef018f6240240e3d8478011ce060ffba64356f758fe4;;\n    Linux.aarch64) system=aarch64-linux; hash=a3be84110647d3aeadef0f8b398c59f71aaf44250ab7ec99464e9781d77b33bb;;\n    Darwin.x86_64) system=x86_64-darwin; hash=bb99ed5a18383133ec5c63677fed5cc199dbbc1ef98d0ac312afe3a27d6380ad;;\n    Darwin.arm64) system=aarch64-darwin; hash=5078b0a1685682b7d8c44d91bd34ee6979959505641662c37c225a0b7ed7eab3;;\n    *) oops \"sorry, there is no binary distribution of Nix for your platform\";;\nesac\n\nurl=\"https://releases.nixos.org/nix/nix-2.3.14/nix-2.3.14-$system.tar.xz\"\n\ntarball=\"$tmpDir/$(basename \"$tmpDir/nix-2.3.14-$system.tar.xz\")\"\n\nrequire_util curl \"download the binary tarball\"\nrequire_util tar \"unpack the binary tarball\"\nif [ \"$(uname -s)\" != \"Darwin\" ]; then\n    require_util xz \"unpack the binary tarball\"\nfi\n\necho \"downloading Nix 2.3.14 binary tarball for $system from '$url' to '$tmpDir'...\"\ncurl -L \"$url\" -o \"$tarball\" || oops \"failed to download '$url'\"\n\nif command -v sha256sum > /dev/null 2>&1; then\n    hash2=\"$(sha256sum -b \"$tarball\" | cut -c1-64)\"\nelif command -v shasum > /dev/null 2>&1; then\n    hash2=\"$(shasum -a 256 -b \"$tarball\" | cut -c1-64)\"\nelif command -v openssl > /dev/null 2>&1; then\n    hash2=\"$(openssl dgst -r -sha256 \"$tarball\" | cut -c1-64)\"\nelse\n    oops \"cannot verify the SHA-256 hash of '$url'; you need one of 'shasum', 'sha256sum', or 'openssl'\"\nfi\n\nif [ \"$hash\" != \"$hash2\" ]; then\n    oops \"SHA-256 hash mismatch in '$url'; expected $hash, got $hash2\"\nfi\n\nunpack=$tmpDir/unpack\nmkdir -p \"$unpack\"\ntar -xJf \"$tarball\" -C \"$unpack\" || oops \"failed to unpack '$url'\"\n\nscript=$(echo \"$unpack\"/*/install)\n\n[ -e \"$script\" ] || oops \"installation script is missing from the binary tarball!\"\n\"$script\" \"$@\"\n\n} # End of wrapping\n`;\n\nfunction makePre(t) {\n  let p = document.createElement('pre');\n  p.appendChild(document.createTextNode(t));\n\n  return p\n}\n\nfunction analyzeScript(contents) {\n  let td = document.createElement('td');\n  td.appendChild(makePre(conch_analyzer__WEBPACK_IMPORTED_MODULE_0__[\"cmd_histogram\"](contents)))\n\n  let tr = document.createElement('tr');\n  tr.appendChild(td)\n\n  let tbody = document.getElementById('results');\n  tbody.innerHTML = ''\n  tbody.appendChild(tr)\n}\n\nconst scriptInput = document.getElementById('script-input');\nscriptInput.innerHTML = nixos_installer_script;\n\nconst analyzeButton = document.getElementById('analyze')\nanalyzeButton.onclick = function() {\n  analyzeScript(scriptInput.value)\n}\n\nanalyzeButton.click()\n\n\n//# sourceURL=webpack:///./index.js?");

/***/ }),

/***/ "./node_modules/webpack/buildin/harmony-module.js":
/*!*******************************************!*\
  !*** (webpack)/buildin/harmony-module.js ***!
  \*******************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = function(originalModule) {\n\tif (!originalModule.webpackPolyfill) {\n\t\tvar module = Object.create(originalModule);\n\t\t// module.parent = undefined by default\n\t\tif (!module.children) module.children = [];\n\t\tObject.defineProperty(module, \"loaded\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.l;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"id\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.i;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"exports\", {\n\t\t\tenumerable: true\n\t\t});\n\t\tmodule.webpackPolyfill = 1;\n\t}\n\treturn module;\n};\n\n\n//# sourceURL=webpack:///(webpack)/buildin/harmony-module.js?");

/***/ })

}]);