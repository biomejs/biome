type A = typeof import("a.json",{});
type B = typeof import("a.json",{with:{[a]:"1"}});
type C = typeof import("a.json",{with:{}}, 1);
type D = import("a",);
type E = import;
type F = typeof import;
