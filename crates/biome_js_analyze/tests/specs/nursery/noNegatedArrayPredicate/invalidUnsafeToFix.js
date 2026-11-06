/* should generate diagnostics */
// Removing the `!` would remove the comment.
const a = ! /* comment */ array.some(element => test(element));
const b = !array.some(element => ! /* comment */ test(element));
// Removing the `!` would change how these statements are parsed.
foo()
![].some(element => test(element));
!{ array: [1] }.array.some(value => value);
!(array.some(element => test(element)));
![1].some(value => value) && ready;
foo
!async function () {}.items.some(value => value);
