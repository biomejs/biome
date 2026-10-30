/* should not generate diagnostics */
declare function fooSync(): void;
type Reader = typeof fs.readFileSync;
(fs.readFileSync as Reader);
