
// @ts-ignore
let found;


// Some comment
// @ts-ignore
// Some other comment
let bar;


// Some comment
// @ts-ignore Some other comment
let someTextAfter;

/* @ts-ignore */ let inBlockComment;

// @ts-ignore twice in one comment @ts-ignore
let twiceInOneComment;

// @ts-ignore
// @ts-ignore
let twoComments;

/**
 * Some documentation
 * @ts-ignore */
let lastLineOfBlockComment;

/** @ts-ignore */
let inJsDocComment;

/// @ts-ignore
let tripleSlash;

//@ts-ignore
let noSpace;

// The note about @ts-ignore is not rewritten
// @ts-ignore
let proseAbove;

// @ts-ignore at the end of the file
