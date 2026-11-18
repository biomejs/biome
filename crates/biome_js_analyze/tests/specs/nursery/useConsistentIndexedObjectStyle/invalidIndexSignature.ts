/* should generate diagnostics */
type Basic = Record<string, any>;

type WithGeneric<T> = Record<string, T>;

type InGeneric = Generic<Record<string, any>>;

type UnionKey = Record<string | number, any>;

type ComputedKey = Record<Exclude<"a" | "b" | "c", "a">, any>;

type NumberKey = Record<number, any>;

type SymbolKey = Record<symbol, any>;

function parameter(arg: Record<string, any>) {}

function returnType(): Record<string, any> {}

type CommentInRecord = Record<string, /* preserve me */ number>;

interface Extending extends Record<string, any> {}

class Implementing implements Record<string, any> {}
