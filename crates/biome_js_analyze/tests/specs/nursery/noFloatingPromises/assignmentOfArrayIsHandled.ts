/* should not generate diagnostics */
let target;
// An array of promises assigned to a variable is, like a single promise,
// considered handled once it's an assignment -- this is unchanged by the
// early bail-out for assignment expressions in `NoFloatingPromises::run`
// (verified: this array-of-promises case was already not flagged before
// that bail-out was added, since the assignment expression's type isn't
// resolved to an array type by the underlying type inference).
target = [1, 2, 3].map(async (x) => x + 1);
