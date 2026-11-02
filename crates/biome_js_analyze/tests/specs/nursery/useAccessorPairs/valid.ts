/* should not generate diagnostics */
abstract class C1 { abstract set a(value: number); }
abstract class C2 { abstract get a(): number; set a(value: number) {} }
interface I1 { set a(value: number); }
type T1 = { set a(value: number) };
declare class C3 { get a(): number; set a(value: number); }
declare class C4 { static get a(): number; static set a(value: number); }
