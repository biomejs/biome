// Comments around ternary parts should be preserved
declare const x: string | null;
const a = x !== null ? /* non-null value */ x : /* fallback */ 'default';

// Comment before test
declare const y: string | null;
const b = /* check */ y === null ? 'fallback' : y;

// Negative form with comments on both branches
declare const z: string | null;
const c = z === null ? /* fallback */ 'default' : /* value */ z;
