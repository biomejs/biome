import {resolve} from "node:path";
import {fileURLToPath} from "node:url";
import {readFileSync, writeFileSync} from "node:fs"

const WASM_WEB_ROOT = resolve(fileURLToPath(import.meta.url), "../..", "@biomejs/wasm-web");
const WASM_WEB_ROOT_PACKAGE = resolve(WASM_WEB_ROOT, "package.json");
const manifest = readFileSync(WASM_WEB_ROOT_PACKAGE, "utf-8");
const manifestObject = JSON.parse(manifest);
manifestObject.name = "@biomejs/wasm-web";
console.log("Update manifest " + WASM_WEB_ROOT_PACKAGE);
writeFileSync(WASM_WEB_ROOT_PACKAGE, JSON.stringify(manifestObject, null, 2));


