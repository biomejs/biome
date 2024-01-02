// Ensure that the type annotation preserves the semicolon in the type
// annotation, even though the parent group breaks. This was only an issue when
// `semicolon: AsNeeded` was set, since the semicolon became conditional on
// whether the group was printed inline.
//
// https://github.com/biomejs/biome/issues/1356.

foo((args: {a: string; b: string}) => {
    return a;
  })