/* should generate diagnostics */
class C1 { private set a(value: number) {} }
class C2 { static set [Symbol.iterator](value: number) {} }
Object.defineProperty(obj, "a", { set(value) {} } as PropertyDescriptor);
declare class C3 { set a(value: number); }
declare class C4 { static set a(value: number); get a(): number; }
