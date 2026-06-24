// `Index<0>` substitutes to the literal `0`, so this reads the first filtered element.
type Index<T extends number> = T;
const first: Index<0> = 0;
[1, 2, 3].filter(x => x > 1)[first];
