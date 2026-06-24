// should not generate diagnostics

// Mutually recursive aliases resolve to opaque types; monomorphization must not loop on them.
type Id<T> = T;
type A = Id<B>;
type B = Id<A>;
declare const text: Id<A>;
text[0] === "a";
