// Test case 1: Basic type alias with extends and trailing comment
type A<B, C extends D> =
    // Some comment
    undefined;

// Test case 2: Type alias with extends, comment after = on new line
type A2<B, C extends D> =
    // Some comment
    undefined;

// Test case 3: Multiple type parameters with extends
type E<F, G extends H, I extends J> =
    // Multiple comments
    // On multiple lines
    string;

// Test case 4: Type alias without extends (should work already)
type K<L, M> =
    // Works without extends
    // Some comment
    undefined;

// Test case 5: Type alias with default value and comment
type N<O extends P = Q> =
    // Comment with default value
    R;

// Test case 6: Type alias with both extends and default
type S<T extends U = V, W extends X = Y> =
    // Comment with both extends and default
    Z;

// Test case 7: Complex right-hand side with extends and comment
type AA<BB, CC extends DD> =
    // Comment before object type
    {
        property: string;
    };

// Test case 8: Union type with extends and comment
type EE<FF, GG extends HH> =
    // Comment before union
    | string
    | number
    | boolean;

// Test case 9: Conditional type with extends and comment
type II<JJ, KK extends LL> =
    // Comment before conditional
    MM extends NN ? OO : PP;

// Test case 10: Type alias with multiple extends and long comment
type QQ<RR extends SS, TT extends UU, VV extends WW> =
    // Very long comment that spans
    // multiple lines to test
    // proper indentation
    XX;
