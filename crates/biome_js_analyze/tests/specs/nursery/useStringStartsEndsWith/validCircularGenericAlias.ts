// should not generate diagnostics

// A self-referential alias resolves to an opaque type; monomorphization must not loop on it.
type Id<T> = T;
type Loop = Id<Loop>;
declare const text: Id<Loop>;
text[0] === "a";
