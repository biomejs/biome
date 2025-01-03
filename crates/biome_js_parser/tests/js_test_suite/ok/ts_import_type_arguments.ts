type A = typeof import(1);
type B = typeof import("a.json",{assert:{}});
type C = typeof import("a.json",{assert:{a:"1"}});
type D = typeof import("a.json",{assert:{"a":"1"}});
type E = typeof import("a.json",{assert:{a:"1", b:"2",}});
type F = import("foo", { with: { "resolution-mode": "import" } });
