// chain is-callee
const x = ((a) => (b) => c)(test);

// chain should break
const x =  ({prop}) => (b) => { c };
const x =  (a): string => b => c => d => e => f;
const x =  (a): string => b => ({test});


// break sequence body on new line
const x =  a => b => (aLongSequenceExpression, thatContinuesFurtherOnUntilItBreaks, expands);
