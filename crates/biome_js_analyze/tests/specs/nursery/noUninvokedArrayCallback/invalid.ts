/* should generate diagnostics */
const a = new Array<number>(5).map((item) => item);
const b = (new Array(5) as number[]).map((item) => item);
const c = new Array(5)!.map((item) => item);
