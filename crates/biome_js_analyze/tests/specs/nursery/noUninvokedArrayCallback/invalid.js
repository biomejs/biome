/* should generate diagnostics */
const a = new Array(5).map(_ => {});
const b = new Array(5).filter(function (_) {});
const c = new Array(5)['every'](function (_) {});
const d = Array(5).map(() => 0);
new Array(5).forEach((_, index) => console.log(index));
const e = new Array(3).some(Boolean);
const f = new Array(3).reduce((acc, item) => acc + item, 0);
const g = new Array(3).reduceRight((acc, item) => acc + item, 0);
const h = new Array(3).flatMap((item) => [item]);
const i = new Array(3).sort((x, y) => x - y);
const j = new Array(3).toSorted((x, y) => x - y);
const k = (new Array(3)).map((item) => item);
const l = new Array((3)).map((item) => item);
const m = new Array(3)?.map((item) => item);
const n = globalThis.Array(3).map((item) => item);
const o = new window.Array(3).map((item) => item);
const p = new Array(0).map((item) => item);
const q = new Array(5).map(createElement);
