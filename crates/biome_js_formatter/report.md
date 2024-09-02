## Overall Metrics

**Average compatibility**: 96.92

<details>
    <summary>Definition</summary>

    $$average = \frac\{\sum_{file}^\{files}compatibility_\{file}}\{files}$$
</details>

**Compatible lines**: 97.67

<details>
    <summary>Definition</summary>

    $$average = \frac{\sum_{file}^{files}matching\_lines_{file}}{max(lines_{rome}, lines_{prettier})}$$
</details>

[Metric definition discussion](https://github.com/rome/tools/issues/2555#issuecomment-1124787893)

## Test cases

### js/array-spread/multiple.js

**Prettier Similarity**: 100.00%


### js/arrays/empty.js

**Prettier Similarity**: 100.00%


### js/arrays/holes-in-args.js

**Prettier Similarity**: 100.00%


### js/arrays/issue-10159.js

**Prettier Similarity**: 100.00%


### js/arrays/last.js

**Prettier Similarity**: 100.00%


### js/arrays/nested.js

**Prettier Similarity**: 100.00%


### js/arrays/numbers-in-args.js

**Prettier Similarity**: 100.00%


### js/arrays/numbers-in-assignment.js

**Prettier Similarity**: 100.00%


### js/arrays/numbers-negative-comment-after-minus.js

**Prettier Similarity**: 100.00%


### js/arrays/numbers-negative.js

**Prettier Similarity**: 100.00%


### js/arrays/numbers-trailing-comma.js

**Prettier Similarity**: 100.00%


### js/arrays/numbers-with-holes.js
```diff
 const numberWithHoles1 = [
   7234932941,
   7234932722,
   7234932312,
   ,
   // comment before a hole 1
   7234932841,
   ,
   7234932843,
   ,
   // comment after a hole 1
   7234932436,
 ];
 
 const numberWithHoles2 = [
   0x234932941,
   0x234932722,
   0x234932312,
-
   ,
   // comment before a hole 2
   0x234932841,
   ,
   0x234932843,
   ,
+
   // comment after a hole 2
   0x234932436,
 ];

```

**Prettier Similarity**: 96.43%


### js/arrays/numbers-with-trailing-comments.js

**Prettier Similarity**: 100.00%


### js/arrays/numbers-with-tricky-comments.js

**Prettier Similarity**: 100.00%


### js/arrays/numbers1.js

**Prettier Similarity**: 100.00%


### js/arrays/numbers2.js

**Prettier Similarity**: 100.00%


### js/arrays/numbers3.js

**Prettier Similarity**: 100.00%


### js/arrays/preserve_empty_lines.js

**Prettier Similarity**: 100.00%


### js/arrow-call/arrow_call.js

**Prettier Similarity**: 100.00%


### js/arrow-call/class-property.js

**Prettier Similarity**: 100.00%


### js/arrows/arrow-chain-with-trailing-comments.js

**Prettier Similarity**: 100.00%


### js/arrows/arrow_function_expression.js

**Prettier Similarity**: 100.00%


### js/arrows/assignment-chain-with-arrow-chain.js

**Prettier Similarity**: 100.00%


### js/arrows/block_like.js

**Prettier Similarity**: 100.00%


### js/arrows/call.js

**Prettier Similarity**: 100.00%


### js/arrows/chain-as-arg.js

**Prettier Similarity**: 100.00%


### js/arrows/chain-in-logical-expression.js

**Prettier Similarity**: 100.00%


### js/arrows/comment.js

**Prettier Similarity**: 100.00%


### js/arrows/curried.js

**Prettier Similarity**: 100.00%


### js/arrows/currying-2.js

**Prettier Similarity**: 100.00%


### js/arrows/currying-3.js

**Prettier Similarity**: 100.00%


### js/arrows/currying-4.js
```diff
 Y(() => (a ? b : c));
 
 Y(() => () => (a ? b : c));
 
 Y(() => () => () => (a ? b : c));
 
 Y(() =>
   longlonglonglonglonglonglonglonglonglongCondition
     ? "Prettier is an opinionated code formatter."
     : "Prettier takes your code and reprints it from scratch by taking the line length into account.",
 );
 
 Y(
   () => () =>
     longlonglonglonglonglonglonglonglonglongCondition
       ? "Prettier is an opinionated code formatter."
       : "Prettier takes your code and reprints it from scratch by taking the line length into account.",
 );
 
 Y(
   () => () => () =>
     longlonglonglonglonglonglonglonglonglongCondition
       ? "Prettier is an opinionated code formatter."
       : "Prettier takes your code and reprints it from scratch by taking the line length into account.",
 );
 
 const x1 = () => [
   "The",
   "green",
   "dragon",
   "liked",
   "to",
   "knit",
   "sweaters",
   "for",
   "the",
   "fluffy",
   "clouds",
   "in",
   "the",
   "sky.",
 ];
 
 const x2 = () => () => [
   "The",
   "green",
   "dragon",
   "liked",
   "to",
   "knit",
   "sweaters",
   "for",
   "the",
   "fluffy",
   "clouds",
   "in",
   "the",
   "sky.",
 ];
 
 const x3 = () => () => () => [
   "The",
   "green",
   "dragon",
   "liked",
   "to",
   "knit",
   "sweaters",
   "for",
   "the",
   "fluffy",
   "clouds",
   "in",
   "the",
   "sky.",
 ];
 
 f((a) => (1, 2, 3) /* a */);
 f((a) => (b) => (1, 2, 3) /* b */ /* a */);
-f((a) => (b) => (c) => (1, 2, 3) /* c */ /* b */ /* a */);
+f((a) => (b) => (c) => (1, 2, 3) /* b */ /* c */ /* a */);
 
 f((a) => (1 ? 2 : 3) /* a */);
 f((a) => (b) => (1 ? 2 : 3) /* b */ /* a */);
-f((a) => (b) => (c) => (1 ? 2 : 3) /* c */ /* b */ /* a */);
+f((a) => (b) => (c) => (1 ? 2 : 3) /* b */ /* c */ /* a */);
 
 a(
   "",
   "",
   ({}) =>
     () =>
     () =>
     () =>
     () =>
     () =>
     () =>
       test,
 );
 a(
   "",
   "",
   ({}) =>
     () =>
     () =>
     () =>
     () =>
     () =>
     () =>
       test ? 1 : 2,
 );

```

**Prettier Similarity**: 98.17%


### js/arrows/currying.js

**Prettier Similarity**: 100.00%


### js/arrows/issue-1389-curry.js

**Prettier Similarity**: 100.00%


### js/arrows/issue-4166-curry.js

**Prettier Similarity**: 100.00%


### js/arrows/long-call-no-args.js

**Prettier Similarity**: 100.00%


### js/arrows/long-contents.js

**Prettier Similarity**: 100.00%


### js/arrows/newline-before-arrow/newline-before-arrow.js
```diff
-async (x) => x;
+async;
+x;
+=> x

```

**Prettier Similarity**: 0.00%


### js/arrows/parens.js

**Prettier Similarity**: 100.00%


### js/arrows/semi/semi.js

**Prettier Similarity**: 100.00%


### js/assignment-comments/call.js

**Prettier Similarity**: 100.00%


### js/assignment-comments/call2.js

**Prettier Similarity**: 100.00%


### js/assignment-comments/function.js

**Prettier Similarity**: 100.00%


### js/assignment-comments/identifier.js

**Prettier Similarity**: 100.00%


### js/assignment-comments/number.js

**Prettier Similarity**: 100.00%


### js/assignment-comments/string.js

**Prettier Similarity**: 100.00%


### js/assignment-expression/assignment_expression.js

**Prettier Similarity**: 100.00%


### js/assignment/binaryish.js

**Prettier Similarity**: 100.00%


### js/assignment/call-with-template.js

**Prettier Similarity**: 100.00%


### js/assignment/chain-two-segments.js

**Prettier Similarity**: 100.00%


### js/assignment/chain.js

**Prettier Similarity**: 100.00%


### js/assignment/destructuring-array.js

**Prettier Similarity**: 100.00%


### js/assignment/destructuring-heuristic.js

**Prettier Similarity**: 100.00%


### js/assignment/destructuring.js

**Prettier Similarity**: 100.00%


### js/assignment/discussion-15196.js

**Prettier Similarity**: 100.00%


### js/assignment/issue-10218.js

**Prettier Similarity**: 100.00%


### js/assignment/issue-1419.js

**Prettier Similarity**: 100.00%


### js/assignment/issue-15534.js

**Prettier Similarity**: 100.00%


### js/assignment/issue-1966.js

**Prettier Similarity**: 100.00%


### js/assignment/issue-2184.js

**Prettier Similarity**: 100.00%


### js/assignment/issue-2482-1.js

**Prettier Similarity**: 100.00%


### js/assignment/issue-2482-2.js

**Prettier Similarity**: 100.00%


### js/assignment/issue-2540.js

**Prettier Similarity**: 100.00%


### js/assignment/issue-3819.js

**Prettier Similarity**: 100.00%


### js/assignment/issue-4094.js

**Prettier Similarity**: 100.00%


### js/assignment/issue-5610.js

**Prettier Similarity**: 100.00%


### js/assignment/issue-6922.js

**Prettier Similarity**: 100.00%


### js/assignment/issue-7091.js

**Prettier Similarity**: 100.00%


### js/assignment/issue-7572.js

**Prettier Similarity**: 100.00%


### js/assignment/issue-7961.js

**Prettier Similarity**: 100.00%


### js/assignment/issue-8218.js

**Prettier Similarity**: 100.00%


### js/assignment/lone-arg.js

**Prettier Similarity**: 100.00%


### js/assignment/sequence.js

**Prettier Similarity**: 100.00%


### js/assignment/unary.js

**Prettier Similarity**: 100.00%


### js/async/async-iteration.js

**Prettier Similarity**: 100.00%


### js/async/async-shorthand-method.js

**Prettier Similarity**: 100.00%


### js/async/await-parse.js

**Prettier Similarity**: 100.00%


### js/async/conditional-expression.js

**Prettier Similarity**: 100.00%


### js/async/exponentiation.js

**Prettier Similarity**: 100.00%


### js/async/inline-await.js

**Prettier Similarity**: 100.00%


### js/async/nested.js

**Prettier Similarity**: 100.00%


### js/async/nested2.js

**Prettier Similarity**: 100.00%


### js/async/parens.js

**Prettier Similarity**: 100.00%


### js/async/simple-nested-await.js

**Prettier Similarity**: 100.00%


### js/big-int/literal.js

**Prettier Similarity**: 100.00%


### js/binary-expressions/arrow.js

**Prettier Similarity**: 100.00%


### js/binary-expressions/bitwise-flags.js

**Prettier Similarity**: 100.00%


### js/binary-expressions/call.js

**Prettier Similarity**: 100.00%


### js/binary-expressions/comment.js

**Prettier Similarity**: 100.00%


### js/binary-expressions/equality.js

**Prettier Similarity**: 100.00%


### js/binary-expressions/exp.js

**Prettier Similarity**: 100.00%


### js/binary-expressions/if.js

**Prettier Similarity**: 100.00%


### js/binary-expressions/in_instanceof.js

**Prettier Similarity**: 100.00%


### js/binary-expressions/inline-jsx.js

**Prettier Similarity**: 100.00%


### js/binary-expressions/inline-object-array.js

**Prettier Similarity**: 100.00%


### js/binary-expressions/jsx_parent.js

**Prettier Similarity**: 100.00%


### js/binary-expressions/like-regexp.js

**Prettier Similarity**: 100.00%


### js/binary-expressions/math.js

**Prettier Similarity**: 100.00%


### js/binary-expressions/return.js

**Prettier Similarity**: 100.00%


### js/binary-expressions/short-right.js

**Prettier Similarity**: 100.00%


### js/binary-expressions/test.js

**Prettier Similarity**: 100.00%


### js/binary-expressions/unary.js

**Prettier Similarity**: 100.00%


### js/binary_math/parens.js

**Prettier Similarity**: 100.00%


### js/bracket-spacing/array.js

**Prettier Similarity**: 100.00%


### js/bracket-spacing/object.js

**Prettier Similarity**: 100.00%


### js/break-calls/break.js

**Prettier Similarity**: 100.00%


### js/break-calls/parent.js

**Prettier Similarity**: 100.00%


### js/break-calls/react.js
```diff
 function helloWorld() {
   useEffect(() => {
     // do something
   }, [props.value]);
   useEffect(() => {
     // do something
   }, [
     props.value,
     props.value,
     props.value,
     props.value,
     props.value,
     props.value,
     props.value,
     props.value,
     props.value,
     props.value,
     props.value,
   ]);
 }
 
 function helloWorldWithReact() {
   React.useEffect(() => {
     // do something
   }, [props.value]);
   React.useEffect(() => {
     // do something
   }, [
     props.value,
     props.value,
     props.value,
     props.value,
     props.value,
     props.value,
     props.value,
     props.value,
     props.value,
     props.value,
     props.value,
   ]);
 }
 
 function MyComponent(props) {
   useEffect(
     () => {
       console.log("some code", props.foo);
     },
 
     // We need to disable the eslint warning here,
     // because of some complicated reason.
     // eslint-disable line react-hooks/exhaustive-deps
     [],
   );
 
   return null;
 }
 
 function Comp1() {
   const { firstName, lastName } = useMemo(
     () => parseFullName(fullName),
     [fullName],
   );
 }
 
 function Comp2() {
   const { firstName, lastName } = useMemo(
     () => func(),
     [
       props.value,
       props.value,
       props.value,
       props.value,
       props.value,
       props.value,
       props.value,
       props.value,
       props.value,
       props.value,
       props.value,
     ],
   );
 }
 
 function Comp3() {
   const { firstName, lastName } = useMemo(
     (aaa, bbb, ccc, ddd, eee, fff, ggg, hhh, iii, jjj, kkk) =>
       func(aaa, bbb, ccc, ddd, eee, fff, ggg, hhh, iii, jjj, kkk),
     [foo, bar, baz],
   );
 }
 
 function Comp4() {
   const { firstName, lastName } = useMemo(
     () =>
       (foo && bar && baz) ||
       baz ||
       (foo && baz(foo) + bar(foo) + foo && bar && baz) ||
       baz ||
       (foo && baz(foo) + bar(foo)),
     [foo, bar, baz],
   );
 }
 
 function Comp5() {
   const { firstName, lastName } = useMemo(() => func(), [foo]);
 }
 
 function Component1() {
-  useImperativeHandle(ref, () => {
-    /* Function body */
-  }, []);
-  useImperativeHandle(ref, () => {
-    /* Function body */
-  }, [props.value]);
-  useImperativeHandle(ref, () => {
-    /* Function body */
-  }, [
-    props.value,
-    props.value,
-    props.value,
-    props.value,
-    props.value,
-    props.value,
-    props.value,
-    props.value,
-    props.value,
-    props.value,
-    props.value,
-  ]);
+  useImperativeHandle(
+    ref,
+    () => {
+      /* Function body */
+    },
+    [],
+  );
+  useImperativeHandle(
+    ref,
+    () => {
+      /* Function body */
+    },
+    [props.value],
+  );
+  useImperativeHandle(
+    ref,
+    () => {
+      /* Function body */
+    },
+    [
+      props.value,
+      props.value,
+      props.value,
+      props.value,
+      props.value,
+      props.value,
+      props.value,
+      props.value,
+      props.value,
+      props.value,
+      props.value,
+    ],
+  );
 }
 
 function Component2() {
-  React.useImperativeHandle(ref, () => {
-    /* Function body */
-  }, []);
-  React.useImperativeHandle(ref, () => {
-    /* Function body */
-  }, [props.value]);
-  React.useImperativeHandle(ref, () => {
-    /* Function body */
-  }, [
-    props.value,
-    props.value,
-    props.value,
-    props.value,
-    props.value,
-    props.value,
-    props.value,
-    props.value,
-    props.value,
-    props.value,
-    props.value,
-  ]);
+  React.useImperativeHandle(
+    ref,
+    () => {
+      /* Function body */
+    },
+    [],
+  );
+  React.useImperativeHandle(
+    ref,
+    () => {
+      /* Function body */
+    },
+    [props.value],
+  );
+  React.useImperativeHandle(
+    ref,
+    () => {
+      /* Function body */
+    },
+    [
+      props.value,
+      props.value,
+      props.value,
+      props.value,
+      props.value,
+      props.value,
+      props.value,
+      props.value,
+      props.value,
+      props.value,
+      props.value,
+    ],
+  );
 }
 
 function Component3() {
-  useImperativeHandle(ref, () => {
-    /* Function body */
-  }, []);
+  useImperativeHandle(
+    ref,
+    () => {
+      /* Function body */
+    },
+    [],
+  );
 }

```

**Prettier Similarity**: 61.17%


### js/break-calls/reduce.js

**Prettier Similarity**: 100.00%


### js/call/first-argument-expansion/expression-2nd-arg.js

**Prettier Similarity**: 100.00%


### js/call/first-argument-expansion/issue-12892.js

**Prettier Similarity**: 100.00%


### js/call/first-argument-expansion/issue-13237.js

**Prettier Similarity**: 100.00%


### js/call/first-argument-expansion/issue-14454.js

**Prettier Similarity**: 100.00%


### js/call/first-argument-expansion/issue-2456.js

**Prettier Similarity**: 100.00%


### js/call/first-argument-expansion/issue-4401.js

**Prettier Similarity**: 100.00%


### js/call/first-argument-expansion/issue-5172.js

**Prettier Similarity**: 100.00%


### js/call/first-argument-expansion/jsx.js

**Prettier Similarity**: 100.00%


### js/call/first-argument-expansion/test.js

**Prettier Similarity**: 100.00%


### js/call/no-argument/special-cases.js

**Prettier Similarity**: 100.00%


### js/chain-expression/call-expression.js

**Prettier Similarity**: 100.00%


### js/chain-expression/issue-15785-1.js

**Prettier Similarity**: 100.00%


### js/chain-expression/issue-15785-2.js

**Prettier Similarity**: 100.00%


### js/chain-expression/issue-15785-3.js

**Prettier Similarity**: 100.00%


### js/chain-expression/issue-15912.js

**Prettier Similarity**: 100.00%


### js/chain-expression/issue-15916.js

**Prettier Similarity**: 100.00%


### js/chain-expression/member-expression.js

**Prettier Similarity**: 100.00%


### js/chain-expression/test-2.js

**Prettier Similarity**: 100.00%


### js/chain-expression/test-3.js

**Prettier Similarity**: 100.00%


### js/chain-expression/test-4.js

**Prettier Similarity**: 100.00%


### js/chain-expression/test.js

**Prettier Similarity**: 100.00%


### js/class-comment/class-property.js

**Prettier Similarity**: 100.00%


### js/class-comment/misc.js

**Prettier Similarity**: 100.00%


### js/class-comment/superclass.js

**Prettier Similarity**: 100.00%


### js/class-extends/complex.js

**Prettier Similarity**: 100.00%


### js/class-extends/extends.js

**Prettier Similarity**: 100.00%


### js/class-static-block/class-static-block.js

**Prettier Similarity**: 100.00%


### js/class-static-block/with-line-breaks.js

**Prettier Similarity**: 100.00%


### js/classes-private-fields/optional-chaining.js

**Prettier Similarity**: 100.00%


### js/classes-private-fields/private_fields.js

**Prettier Similarity**: 100.00%


### js/classes-private-fields/with_comments.js

**Prettier Similarity**: 100.00%


### js/classes/asi.js

**Prettier Similarity**: 100.00%


### js/classes/assignment.js

**Prettier Similarity**: 100.00%


### js/classes/binary.js

**Prettier Similarity**: 100.00%


### js/classes/call.js

**Prettier Similarity**: 100.00%


### js/classes/class-fields-features.js

**Prettier Similarity**: 100.00%


### js/classes/empty.js

**Prettier Similarity**: 100.00%


### js/classes/keyword-property/async.js

**Prettier Similarity**: 100.00%


### js/classes/keyword-property/computed.js

**Prettier Similarity**: 100.00%


### js/classes/keyword-property/get.js

**Prettier Similarity**: 100.00%


### js/classes/keyword-property/private.js

**Prettier Similarity**: 100.00%


### js/classes/keyword-property/set.js

**Prettier Similarity**: 100.00%


### js/classes/keyword-property/static-async.js

**Prettier Similarity**: 100.00%


### js/classes/keyword-property/static-get.js

**Prettier Similarity**: 100.00%


### js/classes/keyword-property/static-set.js

**Prettier Similarity**: 100.00%


### js/classes/keyword-property/static-static.js

**Prettier Similarity**: 100.00%


### js/classes/keyword-property/static.js

**Prettier Similarity**: 100.00%


### js/classes/member.js

**Prettier Similarity**: 100.00%


### js/classes/method.js

**Prettier Similarity**: 100.00%


### js/classes/new.js

**Prettier Similarity**: 100.00%


### js/classes/property.js

**Prettier Similarity**: 100.00%


### js/classes/super.js

**Prettier Similarity**: 100.00%


### js/classes/ternary.js

**Prettier Similarity**: 100.00%


### js/classes/top-level-super/example.js

**Prettier Similarity**: 100.00%


### js/comments-closure-typecast/binary-expr.js

**Prettier Similarity**: 100.00%


### js/comments-closure-typecast/closure-compiler-type-cast.js

**Prettier Similarity**: 100.00%


### js/comments-closure-typecast/comment-in-the-middle.js

**Prettier Similarity**: 100.00%


### js/comments-closure-typecast/comment-placement.js

**Prettier Similarity**: 100.00%


### js/comments-closure-typecast/extra-spaces-and-asterisks.js

**Prettier Similarity**: 100.00%


### js/comments-closure-typecast/iife-issue-5850-isolated.js

**Prettier Similarity**: 100.00%


### js/comments-closure-typecast/iife.js

**Prettier Similarity**: 100.00%


### js/comments-closure-typecast/issue-4124.js

**Prettier Similarity**: 100.00%


### js/comments-closure-typecast/issue-8045.js

**Prettier Similarity**: 100.00%


### js/comments-closure-typecast/issue-9358.js

**Prettier Similarity**: 100.00%


### js/comments-closure-typecast/member.js

**Prettier Similarity**: 100.00%


### js/comments-closure-typecast/nested.js

**Prettier Similarity**: 100.00%


### js/comments-closure-typecast/non-casts.js

**Prettier Similarity**: 100.00%


### js/comments-closure-typecast/object-with-comment.js

**Prettier Similarity**: 100.00%


### js/comments-closure-typecast/satisfies.js

**Prettier Similarity**: 100.00%


### js/comments-closure-typecast/superclass.js

**Prettier Similarity**: 100.00%


### js/comments-closure-typecast/ways-to-specify-type.js

**Prettier Similarity**: 100.00%


### js/comments/15661.js

**Prettier Similarity**: 100.00%


### js/comments/arrow.js

**Prettier Similarity**: 100.00%


### js/comments/assignment-pattern.js

**Prettier Similarity**: 100.00%


### js/comments/before-comma.js

**Prettier Similarity**: 100.00%


### js/comments/binary-expressions-block-comments.js

**Prettier Similarity**: 100.00%


### js/comments/binary-expressions-parens.js

**Prettier Similarity**: 100.00%


### js/comments/binary-expressions-single-comments.js

**Prettier Similarity**: 100.00%


### js/comments/binary-expressions.js

**Prettier Similarity**: 100.00%


### js/comments/blank.js

**Prettier Similarity**: 100.00%


### js/comments/break-continue-statements.js

**Prettier Similarity**: 100.00%


### js/comments/call_comment.js

**Prettier Similarity**: 100.00%


### js/comments/class.js

**Prettier Similarity**: 100.00%


### js/comments/dangling.js

**Prettier Similarity**: 100.00%


### js/comments/dangling_array.js

**Prettier Similarity**: 100.00%


### js/comments/dangling_for.js

**Prettier Similarity**: 100.00%


### js/comments/dynamic_imports.js

**Prettier Similarity**: 100.00%


### js/comments/emoji.js

**Prettier Similarity**: 100.00%


### js/comments/empty-statements.js
```diff
-a; /* a */ // b
-/* c */
-foo; // first
-// second
-// third
-function x() {} // first
-// second
+a; /* a */ /* c */ // b
+
+foo; // first // second // third
+
+function x() {} // first // second
+
 a =
-  b + // 1
-  // 2
-  c + // 3
-  // 4
-  d + // 5
-  /* 6 */
+  b + // 1 // 2
+  c + // 3 // 4
+  d /* 6 */ + // 5
   e; // 7

```

**Prettier Similarity**: 13.33%


### js/comments/export-and-import.js

**Prettier Similarity**: 100.00%


### js/comments/export.js
```diff
 export //comment
- {};
+{};
 
 export /* comment */ {};
 
 const foo = "";
 export {
   foo, // comment
 };
 
 const bar = "";
 export {
   // comment
   bar,
 };
 
 const fooo = "";
 const barr = "";
 export {
   fooo, // comment
   barr, // comment
 };
 
 const foooo = "";
 const barrr = "";
 export {
   foooo,
   // comment
   barrr as baz,
 } from "foo";
 
 const fooooo = "";
 const barrrr = "";
 export {
   fooooo,
   // comment
   barrrr as bazz,
 };

```

**Prettier Similarity**: 97.37%


### js/comments/first-line.js

**Prettier Similarity**: 100.00%


### js/comments/flow-types/inline.js

**Prettier Similarity**: 100.00%


### js/comments/function-declaration.js

**Prettier Similarity**: 100.00%


### js/comments/function/between-parentheses-and-function-body.js

**Prettier Similarity**: 100.00%


### js/comments/html-like/comment.js
```diff
 <!--
-alert(1); 
+alert(1)
 -->

```

**Prettier Similarity**: 66.67%


### js/comments/if.js

**Prettier Similarity**: 100.00%


### js/comments/issue-3532.js

**Prettier Similarity**: 100.00%


### js/comments/issues.js

**Prettier Similarity**: 100.00%


### js/comments/jsdoc-nestled-dangling.js

**Prettier Similarity**: 100.00%


### js/comments/jsdoc-nestled.js

**Prettier Similarity**: 100.00%


### js/comments/jsdoc.js

**Prettier Similarity**: 100.00%


### js/comments/jsx.js

**Prettier Similarity**: 100.00%


### js/comments/last-arg.js

**Prettier Similarity**: 100.00%


### js/comments/multi-comments-2.js

**Prettier Similarity**: 100.00%


### js/comments/multi-comments-on-same-line-2.js

**Prettier Similarity**: 100.00%


### js/comments/multi-comments-on-same-line.js
```diff
 /*========= All on same line =========*/
 a;
 /*1*/ /*2*/ /*3*/
 b;
 
 a; /*1*/ /*2*/ /*3*/
 b;
 
 a;
 /*1*/ /*2*/ /*3*/ b;
 
 a;
 /*
 1*/ /*2*/ /*3
  */
 b;
 
 a; /*
 1*/ /*2*/ /*3
  */
 b;
 
-a;
-/*
+a; /*
 1*/ /*2*/ /*3
- */ b;
+ */
+b;
 
 /*========= First two on same line =========*/
 a;
 /*1*/ /*2*/
 /*3*/
 b;
 
 a; /*1*/ /*2*/
 /*3*/
 b;
 
 a;
 /*1*/ /*2*/
 /*3*/ b;
 
 a;
 /*
 1*/ /*2*/
 /*3
  */
 b;
 
 a; /*
 1*/ /*2*/
 /*3
  */
 b;
 
 a; /*
 1*/ /*2*/
 /*3
  */ b;
 
 /*========= Last two on same line =========*/
 a;
 /*1*/
 /*2*/ /*3*/
 b;
 
 a; /*1*/
 /*2*/ /*3*/
 b;
 
 a;
 /*1*/
 /*2*/ /*3*/ b;
 
 a;
 /*
 1*/
 /*2*/ /*3
  */
 b;
 
 a; /*
 1*/
 /*2*/ /*3
  */
 b;
 
 a; /*
 1*/
 /*2*/ /*3
  */ b;

```

**Prettier Similarity**: 96.67%


### js/comments/multi-comments.js

**Prettier Similarity**: 100.00%


### js/comments/preserve-new-line-last.js

**Prettier Similarity**: 100.00%


### js/comments/return-statement.js
```diff
 function jsx() {
   return (
     // Comment
     <div />
   );
 }
 
 function unary() {
   return (
     // Comment
     !!x
   );
 }
 
 function numericLiteralNoParen() {
   return 1337; // Comment
 }
 
 function logical() {
   return (
     // Reason for 42
     42 && 84
   );
 }
 
 function binary() {
   return (
     // Reason for 42
     42 * 84
   );
 }
 
 function binaryInBinaryLeft() {
   return (
     // Reason for 42
     42 * 84 + 2
   );
 }
 
 function binaryInBinaryRight() {
   return (
     // Reason for 42
     42 + 84 * 2
   );
 }
 
 function conditional() {
   return (
     // Reason for 42
     42 ? 1 : 2
   );
 }
 
 function binaryInConditional() {
   return (
     // Reason for 42
     42 * 3 ? 1 : 2
   );
 }
 
 function call() {
   return (
     // Reason for a
     a()
   );
 }
 
 function memberInside() {
   return (
     // Reason for a.b
     a.b.c
   );
 }
 
 function memberOutside() {
   return (
     // Reason for a
     a.b.c
   );
 }
 
 function memberInAndOutWithCalls() {
-  return aFunction
-    .b // Reason for a
-    ()
-    .c.d();
+  return (
+    // Reason for a
+    aFunction
+      .b()
+      .c.d()
+  );
 }
 
 function excessiveEverything() {
   return (
     // Reason for stuff
     a.b() * 3 + 4 ? ((a`hi`, 1) ? 1 : 1) : 1
   );
 }
 
 // See https://github.com/prettier/prettier/issues/2392
 // function sequenceExpression() {
 //   return (
 //     // Reason for a
 //     a
 //   ), b
 // }
 
 function sequenceExpressionInside() {
   return (
     // Reason for a
     a, b
   );
 }
 
 function taggedTemplate() {
   return (
     // Reason for a
     a`b`
   );
 }
 
 function inlineComment() {
   return /* hi */ 42 || 42;
 }
 
 function multilineBlockSameLine() {
   return (
     /**
      * @type {string}
      */ "result"
   );
 }
 
 function multilineBlockNextLine() {
   return (
     /**
      * @type {string}
      */
     "result"
   );
 }
 
 function multilineBlockSameLineJsx() {
   return (
     /**
      * JSX Same line
      */ <div></div>
   );
 }
 
 function multilineBlockNextLineJsx() {
   return (
     /**
      * JSX Next line
      */
     <div></div>
   );
 }
 
 function singleLineBlockSameLine() {
   return /** Result -> */ "result";
 }
 
 function singleLineBlockNextLine() {
   return (
     /** Result below */
     "result"
   );
 }

```

**Prettier Similarity**: 96.41%


### js/comments/single-star-jsdoc.js

**Prettier Similarity**: 100.00%


### js/comments/switch.js

**Prettier Similarity**: 100.00%


### js/comments/tagged-template-literal.js
```diff
 foo``; // comment
 
 foo // comment
 ``;
 
 foo // comment
 `
 `;
 
-foo/* comment */ `
+foo /* comment */`
 `;
 
-foo/* comment */ `
+foo /* comment */`
 `;

```

**Prettier Similarity**: 85.71%


### js/comments/template-literal.js

**Prettier Similarity**: 100.00%


### js/comments/trailing-jsdocs.js

**Prettier Similarity**: 100.00%


### js/comments/trailing_space.js

**Prettier Similarity**: 100.00%


### js/comments/try.js

**Prettier Similarity**: 100.00%


### js/comments/variable_declarator.js

**Prettier Similarity**: 100.00%


### js/comments/while.js

**Prettier Similarity**: 100.00%


### js/computed-props/classes.js

**Prettier Similarity**: 100.00%


### js/conditional/comments.js
```diff
 var inspect =
   4 === util.inspect.length
     ? // node <= 0.8.x
       function (v, colors) {
         return util.inspect(v, void 0, void 0, colors);
       }
     : // node > 0.8.x
       function (v, colors) {
         return util.inspect(v, { colors: colors });
       };
 
 var inspect =
   4 === util.inspect.length
     ? // node <= 0.8.x
       function (v, colors) {
         return util.inspect(v, void 0, void 0, colors);
       }
     : // node > 0.8.x
       function (v, colors) {
         return util.inspect(v, { colors: colors });
       };
 
 const extractTextPluginOptions = shouldUseRelativeAssetPaths
   ? // Making sure that the publicPath goes back to to build folder.
     { publicPath: Array(cssFilename.split("/").length).join("../") }
   : {};
 
 const extractTextPluginOptions2 = shouldUseRelativeAssetPaths
   ? // Making sure that the publicPath goes back to to build folder.
     { publicPath: Array(cssFilename.split("/").length).join("../") }
   : {};
 
 const extractTextPluginOptions3 = shouldUseRelativeAssetPaths // Making sure that the publicPath goes back to to build folder.
   ? { publicPath: Array(cssFilename.split("/").length).join("../") }
   : {};
 
 const { configureStore } =
   process.env.NODE_ENV === "production"
     ? require("./configureProdStore") // a
     : require("./configureDevStore"); // b
 
 test /* comment
   comment
       comment
 */
   ? foo
   : bar;
 
 test
   ? /* comment
           comment
     comment
           comment
   */
     foo
   : bar;
 
 test
   ? /* comment
        comment
        comment
        comment
     */
     foo
   : test
     ? /* comment
   comment
     comment */
       foo
     : bar;
 
 test ? /* comment */ foo : bar;
 
 test
   ? foo
   : /* comment
          comment
      comment
            comment
     */
     bar;
 
 test
   ? foo
   : /* comment
          comment
      comment
            comment
       A newline will be added after this comment, unfortunately – but it can be removed manually, see next statement.
     */
     test
     ? foo
     : /* comment
   comment
     comment
    */
       bar;
 
 // It is at least possible to delete the extra newline that was
 // unfortunately added before the second condition above:
 test
-  ? foo /* comment
+  ? foo
+  : /* comment
          comment
      comment
            comment
     */
-  : test
+    test
     ? foo
     : /* comment
   comment
     comment
    */
       bar;
 
 test ? foo : /* comment */ bar;
 
 test
   ? test /* c
 c */
     ? foo
     : bar
   : bar;

```

**Prettier Similarity**: 97.56%


### js/conditional/new-expression.js

**Prettier Similarity**: 100.00%


### js/conditional/new-ternary-examples.js

**Prettier Similarity**: 100.00%


### js/conditional/new-ternary-spec.js

**Prettier Similarity**: 100.00%


### js/conditional/no-confusing-arrow.js

**Prettier Similarity**: 100.00%


### js/conditional/postfix-ternary-regressions.js
```diff
 // concatened string in consequent should be visually distinguishable from alternate
 // … or maybe this is okay, because the colon is enough?
 const avatar = has_ordered
   ? "https://marmelab.com/posters/avatar/longer-word-that-breaks-consequent-" +
     numberOfCustomers +
     ".jpeg"
   : undefined;
 
 // Similarly, in the alternate:
 const redirectUrl = pathName
   ? pathName
   : nextPathName + nextSearch ||
     defaultAuthParams.afterLoginUrl.makeThisLongerSoItBreaks;
 
 // And another, more pathological case of the above:
 const isEmpty = (obj) =>
   obj instanceof Date
     ? false
     : obj === "" ||
       obj === null ||
       obj === undefined ||
       obj === somethingThatIsLonger ||
       shallowEqual(obj, {});
 
 // Again, this case is a bit hard to distinguish the alternate.
 const eventsFromOrders =
   orderIds && orders
     ? orderIds.map((id) => ({
         type: "order",
         date: orders[id].date,
         data: orders[id],
       }))
     : [];
 
 // Kinda weird to have dedents to the level of "return" in a function.
 function foo() {
   return !linkTo
     ? false
     : typeof linkTo === "function"
       ? linkTo(record, reference)
       : linkToRecord(rootPath, sourceId, linkTo_as_string);
 }
 function foo2() {
   return React.isValidElement(emptyText)
     ? React.cloneElement(emptyText)
     : emptyText === ""
       ? " " // em space, forces the display of an empty line of normal height
       : translate(emptyText, { _: emptyText });
 }
 
 // Function call ideally wouldnt break break
 const matchingReferencesError = isMatchingReferencesError(matchingReferences)
   ? translate(matchingReferences.error, {
       _: matchingReferences.error,
     })
   : null;
 
 // This one is kinda confusing any way you slice it…
 const obj = {
   error:
     matchingReferencesError &&
     (!input.value ||
       (input.value && selectedReferencesDataStatus === REFERENCES_STATUS_EMPTY))
       ? translate("ra.input.references.all_missing", {
           _: "ra.input.references.all_missing",
         })
       : null,
 };
 
 // I think we should indent after the inner || on this, and do better wtih the parens around the &&
 const obj2 = {
   warning:
     matchingReferencesError ||
     (input.value && selectedReferencesDataStatus !== REFERENCES_STATUS_READY)
       ? matchingReferencesError ||
         translate("ra.input.references.many_missing", {
           _: "ra.input.references.many_missing",
         })
       : null,
 };
 
 // The boolean conditions in the test should look cohesive.
 const selectedReferencesDataStatus =
   !isEmpty(value) && typeof value === "string" && !pattern.test(value)
     ? getMessage(message, { pattern }, value, values)
     : undefined;
 
 // Would be nice if these two nested ternaries didn't look like a single one.
 resolveRedirectTo(
   redirectTo,
   basePath,
   payload
     ? payload.id || (payload.data ? payload.data.id : null)
     : requestPayload
       ? requestPayload.id
       : null,
   payload && payload.data
     ? payload.data
     : requestPayload && requestPayload.data
       ? requestPayload.data
       : null,
 );
 
 const delayedDataProvider = new Proxy(restProvider, {
   get: (target, name, self) =>
-    name === "then" // as we await for the dataProvider, JS calls then on it. We must trap that call or else the dataProvider will be called with the then method
-      ? self
+    name === "then"
+      ? // as we await for the dataProvider, JS calls then on it. We must trap that call or else the dataProvider will be called with the then method
+        self
       : (resource, params) =>
           new Promise((resolve) =>
             setTimeout(
               () => resolve(restProvider[name](resource, params)),
               500,
             ),
           ),
 });
 
 function foo4() {
   return !match || match.length < 5
     ? line
     : match[1] + match[2] + match[3] + match[4];
 }
 
 function foo5() {
   return !match || match.length < 5
     ? foo(line)
     : match[1] + match[2] + match[3] + match[4];
 }
 
 function foo6() {
   return !match || match.length < 5
     ? linethatisverylongandbreaksthelinehooray
     : match[1] + match[2] + match[3] + match[4];
 }
 
 function foo7() {
   return !match || match.length < 5
     ? linethatisverylongandbreaksthelinehoorayjustabitlonger
     : match[1] + match[2] + match[3] + match[4];
 }
 
 const badComments = schema.model
   ? schema
   : // If model is an array where the items schema is a referred model then we need to use that
     schema.type === "array"
     ? schema.items
     : schema;
 
 const anotherBadComment = refModel
   ? // If we're in a shared params file then reference the model name directly
     inSharedParamsFile
     ? refModel
     : // If we're not in a shared params file then reference the in-file type
       classRef()
   : // We don't have a model name, use the in-file name
     classRef();

```

**Prettier Similarity**: 98.08%


### js/cursor/comments-1.js

**Prettier Similarity**: 100.00%


### js/cursor/comments-2.js

**Prettier Similarity**: 100.00%


### js/cursor/comments-3.js

**Prettier Similarity**: 100.00%


### js/cursor/comments-4.js

**Prettier Similarity**: 100.00%


### js/cursor/cursor-0.js

**Prettier Similarity**: 100.00%


### js/cursor/cursor-1.js

**Prettier Similarity**: 100.00%


### js/cursor/cursor-10.js

**Prettier Similarity**: 100.00%


### js/cursor/cursor-11.js

**Prettier Similarity**: 100.00%


### js/cursor/cursor-12.js

**Prettier Similarity**: 100.00%


### js/cursor/cursor-13.js

**Prettier Similarity**: 100.00%


### js/cursor/cursor-2.js

**Prettier Similarity**: 100.00%


### js/cursor/cursor-3.js

**Prettier Similarity**: 100.00%


### js/cursor/cursor-4.js

**Prettier Similarity**: 100.00%


### js/cursor/cursor-5.js

**Prettier Similarity**: 100.00%


### js/cursor/cursor-6.js

**Prettier Similarity**: 100.00%


### js/cursor/cursor-7.js

**Prettier Similarity**: 100.00%


### js/cursor/cursor-8.js

**Prettier Similarity**: 100.00%


### js/cursor/cursor-9.js

**Prettier Similarity**: 100.00%


### js/cursor/cursor-emoji.js

**Prettier Similarity**: 100.00%


### js/cursor/file-start-with-comment-1.js

**Prettier Similarity**: 100.00%


### js/cursor/file-start-with-comment-2.js

**Prettier Similarity**: 100.00%


### js/cursor/file-start-with-comment-3.js

**Prettier Similarity**: 100.00%


### js/cursor/range-0.js

**Prettier Similarity**: 100.00%


### js/cursor/range-1.js

**Prettier Similarity**: 100.00%


### js/cursor/range-2.js

**Prettier Similarity**: 100.00%


### js/cursor/range-3.js

**Prettier Similarity**: 100.00%


### js/cursor/range-4.js

**Prettier Similarity**: 100.00%


### js/cursor/range-5.js

**Prettier Similarity**: 100.00%


### js/cursor/range-6.js

**Prettier Similarity**: 100.00%


### js/cursor/range-7.js

**Prettier Similarity**: 100.00%


### js/cursor/range-8.js

**Prettier Similarity**: 100.00%


### js/decorator-auto-accessors/basic.js

**Prettier Similarity**: 100.00%


### js/decorator-auto-accessors/comments.js

**Prettier Similarity**: 100.00%


### js/decorator-auto-accessors/computed.js

**Prettier Similarity**: 100.00%


### js/decorator-auto-accessors/not-accessor-method.js

**Prettier Similarity**: 100.00%


### js/decorator-auto-accessors/not-accessor-property.js

**Prettier Similarity**: 100.00%


### js/decorator-auto-accessors/private.js

**Prettier Similarity**: 100.00%


### js/decorator-auto-accessors/static-computed.js

**Prettier Similarity**: 100.00%


### js/decorator-auto-accessors/static-private.js

**Prettier Similarity**: 100.00%


### js/decorator-auto-accessors/static.js

**Prettier Similarity**: 100.00%


### js/decorator-auto-accessors/with-semicolon-1.js

**Prettier Similarity**: 100.00%


### js/decorator-auto-accessors/with-semicolon-2.js

**Prettier Similarity**: 100.00%


### js/decorators-export/after_export.js

**Prettier Similarity**: 100.00%


### js/decorators-export/before_export.js

**Prettier Similarity**: 100.00%


### js/decorators/class-expression/arguments.js

**Prettier Similarity**: 100.00%


### js/decorators/class-expression/class-expression.js

**Prettier Similarity**: 100.00%


### js/decorators/class-expression/member-expression.js

**Prettier Similarity**: 100.00%


### js/decorators/class-expression/super-class.js

**Prettier Similarity**: 100.00%


### js/decorators/classes.js

**Prettier Similarity**: 100.00%


### js/decorators/comments.js

**Prettier Similarity**: 100.00%


### js/decorators/member-expression.js

**Prettier Similarity**: 100.00%


### js/decorators/methods.js

**Prettier Similarity**: 100.00%


### js/decorators/mixed.js

**Prettier Similarity**: 100.00%


### js/decorators/mobx.js

**Prettier Similarity**: 100.00%


### js/decorators/multiline.js

**Prettier Similarity**: 100.00%


### js/decorators/multiple.js

**Prettier Similarity**: 100.00%


### js/decorators/parens.js

**Prettier Similarity**: 100.00%


### js/decorators/redux.js

**Prettier Similarity**: 100.00%


### js/destructuring-ignore/ignore.js

**Prettier Similarity**: 100.00%


### js/destructuring/destructuring.js

**Prettier Similarity**: 100.00%


### js/destructuring/issue-5988.js

**Prettier Similarity**: 100.00%


### js/directives/escaped.js

**Prettier Similarity**: 100.00%


### js/directives/issue-7346.js

**Prettier Similarity**: 100.00%


### js/directives/last-line-0.js

**Prettier Similarity**: 100.00%


### js/directives/last-line-1.js

**Prettier Similarity**: 100.00%


### js/directives/last-line-2.js

**Prettier Similarity**: 100.00%


### js/directives/newline.js

**Prettier Similarity**: 100.00%


### js/directives/no-newline.js

**Prettier Similarity**: 100.00%


### js/directives/test.js

**Prettier Similarity**: 100.00%


### js/dynamic-import/assertions.js

**Prettier Similarity**: 100.00%


### js/dynamic-import/test.js

**Prettier Similarity**: 100.00%


### js/empty-paren-comment/class-property.js

**Prettier Similarity**: 100.00%


### js/empty-paren-comment/class.js

**Prettier Similarity**: 100.00%


### js/empty-paren-comment/empty_paren_comment.js

**Prettier Similarity**: 100.00%


### js/empty-statement/body.js

**Prettier Similarity**: 100.00%


### js/empty-statement/no-newline.js

**Prettier Similarity**: 100.00%


### js/end-of-line/example.js

**Prettier Similarity**: 100.00%


### js/es6modules/export_default_arrow_expression.js

**Prettier Similarity**: 100.00%


### js/es6modules/export_default_call_expression.js

**Prettier Similarity**: 100.00%


### js/es6modules/export_default_class_declaration.js

**Prettier Similarity**: 100.00%


### js/es6modules/export_default_class_expression.js

**Prettier Similarity**: 100.00%


### js/es6modules/export_default_function_declaration.js

**Prettier Similarity**: 100.00%


### js/es6modules/export_default_function_declaration_async.js

**Prettier Similarity**: 100.00%


### js/es6modules/export_default_function_declaration_named.js

**Prettier Similarity**: 100.00%


### js/es6modules/export_default_function_expression.js

**Prettier Similarity**: 100.00%


### js/es6modules/export_default_function_expression_named.js

**Prettier Similarity**: 100.00%


### js/es6modules/export_default_new_expression.js

**Prettier Similarity**: 100.00%


### js/explicit-resource-management/for-await-using-of-comments.js

**Prettier Similarity**: 100.00%


### js/explicit-resource-management/invalid-duplicate-using-bindings.js

**Prettier Similarity**: 100.00%


### js/explicit-resource-management/invalid-script-top-level-using-binding.js

**Prettier Similarity**: 100.00%


### js/explicit-resource-management/using-declarations.js

**Prettier Similarity**: 100.00%


### js/explicit-resource-management/valid-await-expr-using-in.js

**Prettier Similarity**: 100.00%


### js/explicit-resource-management/valid-await-expr-using-instanceof.js

**Prettier Similarity**: 100.00%


### js/explicit-resource-management/valid-await-expr-using.js

**Prettier Similarity**: 100.00%


### js/explicit-resource-management/valid-await-using-asi-assignment.js

**Prettier Similarity**: 100.00%


### js/explicit-resource-management/valid-await-using-binding-basic.js

**Prettier Similarity**: 100.00%


### js/explicit-resource-management/valid-await-using-binding-escaped.js
```diff
 async function f() {
-  await using ab = c;
+  await using \u0061b = c;
 }

```

**Prettier Similarity**: 66.67%


### js/explicit-resource-management/valid-await-using-binding-non-bmp.js

**Prettier Similarity**: 100.00%


### js/explicit-resource-management/valid-await-using-binding-using.js

**Prettier Similarity**: 100.00%


### js/explicit-resource-management/valid-await-using-comments.js
```diff
 async function f() {
   {
-    /*0*/ await using /*1*/ /*2*/ b /*3*/ = /*4*/ f(); /*5*/
+    /*0*/ await using /*1*/ /*2*/ b /*3*/ = /*4*/ f() /*5*/;
   }
   {
     /*0*/ for (
       /*1*/ /*2*/ await using /*3*/ /*4*/ b /*5*/ =
           /*6*/ x /*7*/ /*8*/ /*9*/ /*10*/;
       ;
-
     );
   }
   {
     /*0*/ for (/*1*/ /*2*/ await using /*3*/ /*4*/ b /*5*/ of /*6*/ x /*7*/ /*8*/);
   }
   {
     /*0*/ for await (/*1*/ /*2*/ /*3*/ await using /*4*/ /*5*/ b /*6*/ of /*7*/ x /*8*/ /*9*/);
   }
 }

```

**Prettier Similarity**: 89.47%


### js/explicit-resource-management/valid-for-await-using-binding-escaped-of-of.js

**Prettier Similarity**: 100.00%


### js/explicit-resource-management/valid-for-using-binding-escaped-of-of.js

**Prettier Similarity**: 100.00%


### js/explicit-resource-management/valid-for-using-binding-of-of.js

**Prettier Similarity**: 100.00%


### js/explicit-resource-management/valid-for-using-declaration.js

**Prettier Similarity**: 100.00%


### js/explicit-resource-management/valid-using-as-identifier-computed-member.js

**Prettier Similarity**: 100.00%


### js/explicit-resource-management/valid-using-as-identifier-expression-statement.js

**Prettier Similarity**: 100.00%


### js/explicit-resource-management/valid-using-as-identifier-for-await-of.js

**Prettier Similarity**: 100.00%


### js/explicit-resource-management/valid-using-as-identifier-for-in.js

**Prettier Similarity**: 100.00%


### js/explicit-resource-management/valid-using-as-identifier-for-init.js

**Prettier Similarity**: 100.00%


### js/explicit-resource-management/valid-using-as-identifier-for-of.js

**Prettier Similarity**: 100.00%


### js/explicit-resource-management/valid-using-as-identifier-in.js

**Prettier Similarity**: 100.00%


### js/explicit-resource-management/valid-using-binding-basic.js

**Prettier Similarity**: 100.00%


### js/explicit-resource-management/valid-using-binding-escaped.js
```diff
 {
-  using ab = c;
+  using \u0061b = c;
 }

```

**Prettier Similarity**: 66.67%


### js/explicit-resource-management/valid-using-binding-non-bmp.js

**Prettier Similarity**: 100.00%


### js/explicit-resource-management/valid-using-binding-using.js

**Prettier Similarity**: 100.00%


### js/export-default/binary_and_template.js

**Prettier Similarity**: 100.00%


### js/export-default/body.js

**Prettier Similarity**: 100.00%


### js/export-default/class_instance.js

**Prettier Similarity**: 100.00%


### js/export-default/function_in_template.js

**Prettier Similarity**: 100.00%


### js/export-default/function_tostring.js

**Prettier Similarity**: 100.00%


### js/export-default/iife.js

**Prettier Similarity**: 100.00%


### js/export-star/export-star-as-default.js

**Prettier Similarity**: 100.00%


### js/export-star/export-star-as-reserved-word.js

**Prettier Similarity**: 100.00%


### js/export-star/export-star-as-string.js

**Prettier Similarity**: 100.00%


### js/export-star/export-star-as-string2.js

**Prettier Similarity**: 100.00%


### js/export-star/export-star-as.js

**Prettier Similarity**: 100.00%


### js/export-star/export-star.js

**Prettier Similarity**: 100.00%


### js/export/blank-line-between-specifiers.js
```diff
 export {
   // a
   foo1,
-
   // b
   bar1,
   baz1,
 } from "mod";
 
 const foo2 = 1;
 const bar2 = 1;
 const baz2 = 1;
 
 export {
   // a
   foo2,
-
   // b
   bar2,
   baz2,
 };

```

**Prettier Similarity**: 90.48%


### js/export/bracket.js

**Prettier Similarity**: 100.00%


### js/export/empty.js

**Prettier Similarity**: 100.00%


### js/export/same-local-and-exported.js

**Prettier Similarity**: 100.00%


### js/export/test.js

**Prettier Similarity**: 100.00%


### js/export/undefined.js

**Prettier Similarity**: 100.00%


### js/expression_statement/no_regression.js

**Prettier Similarity**: 100.00%


### js/expression_statement/use_strict.js

**Prettier Similarity**: 100.00%


### js/for-await/for-await.js

**Prettier Similarity**: 100.00%


### js/for-of/async-identifier.js

**Prettier Similarity**: 100.00%


### js/for/comment.js

**Prettier Similarity**: 100.00%


### js/for/continue-and-break-comment-1.js

**Prettier Similarity**: 100.00%


### js/for/continue-and-break-comment-2.js

**Prettier Similarity**: 100.00%


### js/for/continue-and-break-comment-without-blocks.js
```diff
 for (;;) continue;
 // comment
 
 for (;;) break;
 // comment
 
 for (const f of []) continue;
 // comment
 
 for (const f of []) break;
 // comment
 
 for (const f in {}) continue;
 // comment
 
 for (const f in {}) break;
 // comment
 
 for (;;) continue; // comment
 
 for (;;) break; // comment
 
 for (const f of []) continue; // comment
 
 for (const f of []) break; // comment
 
 for (const f in {}) continue; // comment
 
 for (const f in {}) break; // comment
 
 for (;;) continue; /* comment */
 
 for (;;) break; /* comment */
 
 for (const f of []) continue; /* comment */
 
 for (const f of []) break; /* comment */
 
 for (const f in {}) continue; /* comment */
 
 for (const f in {}) break; /* comment */
 
 for (;;) continue;
 /* comment */
 
 for (;;) break;
 /* comment */
 
 for (const f of []) continue;
 /* comment */
 
 for (const f of []) break;
 /* comment */
 
 for (const f in {}) continue;
 /* comment */
 
 for (const f in {}) break;
 /* comment */
 
-label1: for (;;) continue label1 /* comment */;
+label1: for (;;) continue label1; /* comment */
 
 label1: for (;;) continue label1;
 /* comment */
 
 label1: for (;;) continue label1; // comment
 
 label1: for (;;) continue label1;
 // comment

```

**Prettier Similarity**: 98.55%


### js/for/for-in-with-initializer.js
```diff
 // https://github.com/babel/babel/blob/HEAD/packages/babel-generator/test/fixtures/parentheses/in-inside-for/input.js
 
 for (var a = (b in c) in {});
-for (var a = (1 || b in c) in {});
-for (var a = (1 + (2 || b in c)) in {});
-for (var a = (() => b in c) in {});
-for (var a = (1 || (() => b in c)) in {});
-for (var a = (() => {
+for (var a = 1 || (b in c) in {});
+for (var a = 1 + (2 || (b in c)) in {});
+for (var a = () => (b in c) in {});
+for (var a = 1 || (() => (b in c)) in {});
+for (var a = () => {
   b in c;
-}) in {});
-for (var a = ([b in c]) in {});
-for (var a = ({ b: b in c }) in {});
+} in {});
+for (var a = [(b in c)] in {});
+for (var a = { b: (b in c) } in {});
 // Meriyah can't parse
 // for (var a = (x = b in c) => {} in {});
-for (var a = (class extends (b in c) {}) in {});
-for (var a = (function (x = b in c) {}) in {});
+for (var a = class extends (b in c) {} in {});
+for (var a = function (x = (b in c)) {} in {});

```

**Prettier Similarity**: 37.50%


### js/for/for.js

**Prettier Similarity**: 100.00%


### js/for/in.js

**Prettier Similarity**: 100.00%


### js/for/parentheses.js
```diff
 // https://github.com/babel/babel/blob/HEAD/packages/babel-generator/test/fixtures/parentheses/in-inside-for/input.js
 
 for (var a = (b in c); ; );
 for (var a = 1 || (b in c); ; );
 for (var a = 1 + (2 || (b in c)); ; );
 for (var a = () => (b in c); ; );
 for (var a = 1 || (() => (b in c)); ; );
 for (
   var a = () => {
-    (b in c);
+    b in c;
   };
   ;
-
 );
 for (var a = [(b in c)]; ; );
 for (var a = { b: (b in c) }; ; );
 for (var a = (x = (b in c)) => {}; ; );
 for (var a = class extends (b in c) {}; ; );
 for (var a = function (x = (b in c)) {}; ; );
 
 for (var a in b in c);
 for (var a in 1 || b in c);
 for (var a in 1 + (2 || b in c));
 for (var a in () => b in c);
 for (var a in 1 || (() => b in c));
 for (var a in () => {
   b in c;
 });
 for (var a in [b in c]);
 for (var a in { b: b in c });
 for (var a in (x = b in c) => {});
 for (var a in class extends (b in c) {});
 for (var a in function (x = b in c) {});
 
 for (; (a = b in c); );
 for (; (a = 1 || b in c); );
 for (; (a = 1 + (2 || b in c)); );
 for (; (a = () => b in c); );
 for (; (a = 1 || (() => b in c)); );
 for (
   ;
   (a = () => {
     b in c;
   });
-
 );
 for (; (a = [b in c]); );
 for (; (a = { b: b in c }); );
 for (; (a = (x = b in c) => {}); );
 for (; (a = class extends (b in c) {}); );
 for (; (a = function (x = b in c) {}); );

```

**Prettier Similarity**: 94.12%


### js/for/var.js

**Prettier Similarity**: 100.00%


### js/function-comments/params-trail-comments.js

**Prettier Similarity**: 100.00%


### js/function-first-param/function_expression.js

**Prettier Similarity**: 100.00%


### js/function-single-destructuring/array.js

**Prettier Similarity**: 100.00%


### js/function-single-destructuring/object.js

**Prettier Similarity**: 100.00%


### js/function/function_expression.js

**Prettier Similarity**: 100.00%


### js/function/issue-10277.js

**Prettier Similarity**: 100.00%


### js/functional-composition/functional_compose.js

**Prettier Similarity**: 100.00%


### js/functional-composition/gobject_connect.js

**Prettier Similarity**: 100.00%


### js/functional-composition/lodash_flow.js

**Prettier Similarity**: 100.00%


### js/functional-composition/lodash_flow_right.js

**Prettier Similarity**: 100.00%


### js/functional-composition/mongo_connect.js

**Prettier Similarity**: 100.00%


### js/functional-composition/pipe-function-calls-with-comments.js

**Prettier Similarity**: 100.00%


### js/functional-composition/pipe-function-calls.js

**Prettier Similarity**: 100.00%


### js/functional-composition/ramda_compose.js

**Prettier Similarity**: 100.00%


### js/functional-composition/ramda_pipe.js

**Prettier Similarity**: 100.00%


### js/functional-composition/redux_compose.js

**Prettier Similarity**: 100.00%


### js/functional-composition/redux_connect.js

**Prettier Similarity**: 100.00%


### js/functional-composition/reselect_createselector.js

**Prettier Similarity**: 100.00%


### js/functional-composition/rxjs_pipe.js

**Prettier Similarity**: 100.00%


### js/generator/anonymous.js

**Prettier Similarity**: 100.00%


### js/generator/async.js

**Prettier Similarity**: 100.00%


### js/generator/function-name-starts-with-get.js

**Prettier Similarity**: 100.00%


### js/identifier/for-of/await.js

**Prettier Similarity**: 100.00%


### js/identifier/for-of/let.js

**Prettier Similarity**: 100.00%


### js/identifier/parentheses/const.js

**Prettier Similarity**: 100.00%


### js/identifier/parentheses/let.js

**Prettier Similarity**: 100.00%


### js/if/comment_before_else.js

**Prettier Similarity**: 100.00%


### js/if/else.js

**Prettier Similarity**: 100.00%


### js/if/expr_and_same_line_comments.js
```diff
 if (a === 0)
   doSomething(); // comment A1
 else if (a === 1)
   doSomethingElse(); // comment B1
 else if (a === 2) doSomethingElse(); // comment C1
 
 if (a === 0) doSomething(); /* comment A2 */
 else if (a === 1) doSomethingElse(); /* comment B2 */
 else if (a === 2) doSomethingElse(); /* comment C2 */
 
 if (a === 0)
   expr; // comment A3
 else if (a === 1)
   expr; // comment B3
 else if (a === 2) expr; // comment C3
 
 if (a === 0) expr; /* comment A4 */
 else if (a === 1) expr; /* comment B4 */
 else if (a === 2) expr; /* comment C4 */
 
 if (a === 0)
   looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong; // comment A5
 else if (a === 1)
   looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong; // comment B5
 else if (a === 2)
   looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong; // comment C5
 
 function a() {
   if (a) return; /* comment 6a */
   else return 2;
 
   if (a) return 1; /* comment 6b */
   else return 2;
 
   if (a) throw e; /* comment 6d */
   else return 2;
 
   if (a) var a = 1; /* comment 6e */
   else return 2;
 
   if (a)
-    if (b /* comment 6f */);
+    if (b); /* comment 6f */
     else return 2;
 }

```

**Prettier Similarity**: 97.73%


### js/if/if_comments.js

**Prettier Similarity**: 100.00%


### js/if/issue-15168.js

**Prettier Similarity**: 100.00%


### js/if/non-block.js
```diff
 if (foo)
   for (i = 2; i > 0; i--) console.log(i); // foo
 else bar();
 
 if (foo)
   do {
     console.log(i);
-  } while (i--);
-// foo
+  } while (i--); // foo
 else bar();

```

**Prettier Similarity**: 80.00%


### js/if/trailing_comment.js

**Prettier Similarity**: 100.00%


### js/ignore/class-expression-decorator.js
```diff
-// prettier-ignore
 (
+  // prettier-ignore
   @decorator
   class {}
 );

```

**Prettier Similarity**: 80.00%


### js/ignore/decorator.js

**Prettier Similarity**: 100.00%


### js/ignore/ignore-2.js

**Prettier Similarity**: 100.00%


### js/ignore/ignore.js

**Prettier Similarity**: 100.00%


### js/ignore/issue-10661.js

**Prettier Similarity**: 100.00%


### js/ignore/issue-11077.js
```diff
 function HelloWorld(x) {
-  // prettier-ignore
-  // eslint-disable-next-line
-  (x.a |
-    x.b).call(null);
+  (
+    // prettier-ignore
+    // eslint-disable-next-line
+    x.a |
+    x.b
+  ).call(null)
 }
 
 function HelloWorld(x) {
   // prettier-ignore
+
   (
     // eslint-disable-next-line
     x.a |
     x.b
   ).call(null)
 }

```

**Prettier Similarity**: 61.11%


### js/ignore/issue-13737.js
```diff
 oneArgument(
   // prettier-ignore
   (0, 1),
 );
 
 a =
-  // prettier-ignore
-  (0, 1);
+  (
+ // prettier-ignore
+ (0, 1)
+);

```

**Prettier Similarity**: 60.00%


### js/ignore/issue-14404.js
```diff
 async function foo() {
-  // prettier-ignore
-  // b
-  (await thing()).blah;
+  (
+  	// prettier-ignore
+  	// b
+	await thing()
+).blah
 }

```

**Prettier Similarity**: 28.57%


### js/ignore/issue-9335.js

**Prettier Similarity**: 100.00%


### js/ignore/issue-9877.js
```diff
 export default function test() {
   return {
     matrix: // prettier-ignore
-      new Float32Array([
-      0, 0,
-      1, 0,
-      1, 1,
-      0, 1
-    ]),
+      new Float32Array([0, 0, 1, 0, 1, 1, 0, 1]),
   };
 }

```

**Prettier Similarity**: 45.45%


### js/ignore/semi/asi.js

**Prettier Similarity**: 100.00%


### js/ignore/semi/directive.js

**Prettier Similarity**: 100.00%


### js/import-assertions/bracket-spacing/dynamic-import.js

**Prettier Similarity**: 100.00%


### js/import-assertions/bracket-spacing/empty.js
```diff
-export * as bar from "bar.json" assert {};
+export * as bar from "bar.json";

```

**Prettier Similarity**: 0.00%


### js/import-assertions/bracket-spacing/re-export.js

**Prettier Similarity**: 100.00%


### js/import-assertions/bracket-spacing/static-import.js

**Prettier Similarity**: 100.00%


### js/import-assertions/dynamic-import.js

**Prettier Similarity**: 100.00%


### js/import-assertions/empty.js
```diff
 export * as foo from "foo.json";
-export * as bar from "bar.json" assert {};
-export * as baz from "baz.json" /* comment */ assert {};
+export * as bar from "bar.json";
+export * as baz from "baz.json" /* comment */;
 
 import * as foo from "foo.json";
-import * as bar from "bar.json" assert {};
-import * as baz from "baz.json" /* comment */ assert {};
+import * as bar from "bar.json";
+import * as baz from "baz.json" /* comment */;

```

**Prettier Similarity**: 42.86%


### js/import-assertions/multi-types.js

**Prettier Similarity**: 100.00%


### js/import-assertions/non-type.js

**Prettier Similarity**: 100.00%


### js/import-assertions/not-import-assertions.js

**Prettier Similarity**: 100.00%


### js/import-assertions/re-export.js

**Prettier Similarity**: 100.00%


### js/import-assertions/static-import.js

**Prettier Similarity**: 100.00%


### js/import-assertions/without-from.js

**Prettier Similarity**: 100.00%


### js/import-attributes/bracket-spacing/dynamic-import.js

**Prettier Similarity**: 100.00%


### js/import-attributes/bracket-spacing/empty.js
```diff
-export * as bar from "bar.json" with {};
+export * as bar from "bar.json";

```

**Prettier Similarity**: 0.00%


### js/import-attributes/bracket-spacing/re-export.js

**Prettier Similarity**: 100.00%


### js/import-attributes/bracket-spacing/static-import.js

**Prettier Similarity**: 100.00%


### js/import-attributes/dynamic-import.js

**Prettier Similarity**: 100.00%


### js/import-attributes/empty.js
```diff
 export * as foo from "foo.json";
-export * as bar from "bar.json" with {};
-export * as baz from "baz.json" /* comment */ with {};
+export * as bar from "bar.json";
+export * as baz from "baz.json" /* comment */;
 
 import * as foo from "foo.json";
-import * as bar from "bar.json" with {};
-import * as baz from "baz.json" /* comment */ with {};
+import * as bar from "bar.json";
+import * as baz from "baz.json" /* comment */;

```

**Prettier Similarity**: 42.86%


### js/import-attributes/keyword-detect.js
```diff
-import "./test.json" /* assert */ /* assert */ with { type: "json" };
-import {} from "./test.json" /* assert */ /* assert */ with { type: "json" };
-import "./test.json" /* with */ /* with */ assert { type: "json" };
-import {} from "./test.json" /* with */ /* with */ assert { type: "json" };
+import "./test.json" /* assert */ with { /* assert */ type: "json" };
+import {} from "./test.json" /* assert */ with { /* assert */ type: "json" };
+import "./test.json" /* with */ assert { /* with */ type: "json" };
+import {} from "./test.json" /* with */ assert { /* with */ type: "json" };
 
-export {} from "./test.json" /* assert */ /* assert */ with { type: "json" };
-export {} from "./test.json" /* with */ /* with */ assert { type: "json" };
+export {} from "./test.json" /* assert */ with { /* assert */ type: "json" };
+export {} from "./test.json" /* with */ assert { /* with */ type: "json" };
 
-export * from "./test.json" /* assert */ /* assert */ with { type: "json" };
-export * from "./test.json" /* with */ /* with */ assert { type: "json" };
+export * from "./test.json" /* assert */ with { /* assert */ type: "json" };
+export * from "./test.json" /* with */ assert { /* with */ type: "json" };

```

**Prettier Similarity**: 20.00%


### js/import-attributes/long-sources.js
```diff
 import a10 from "./aaaaaaaaaa.json" with { type: "json" };
 import a20 from "./aaaaaaaaaaaaaaaaaaaa.json" with { type: "json" };
 import a30 from "./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json" with { type: "json" };
-import a40 from "./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json" with { type: "json" };
-import a50 from "./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json" with { type: "json" };
-import a60 from "./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json" with { type: "json" };
-import a70 from "./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json" with { type: "json" };
-import a80 from "./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json" with { type: "json" };
+import a40 from "./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json" with {
+  type: "json",
+};
+import a50 from "./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json" with {
+  type: "json",
+};
+import a60 from "./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json" with {
+  type: "json",
+};
+import a70 from "./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json" with {
+  type: "json",
+};
+import a80 from "./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json" with {
+  type: "json",
+};
 
 import("./aaaaaaaaaa.json", { with: { type: "json" } });
 import("./aaaaaaaaaaaaaaaaaaaa.json", { with: { type: "json" } });
 import("./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json", { with: { type: "json" } });
 import("./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json", {
   with: { type: "json" },
 });
 import("./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json", {
   with: { type: "json" },
 });
 import("./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json", {
   with: { type: "json" },
 });
 import(
   "./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json",
   { with: { type: "json" } }
 );
 import(
   "./aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json",
   { with: { type: "json" } }
 );

```

**Prettier Similarity**: 61.54%


### js/import-attributes/multi-types.js

**Prettier Similarity**: 100.00%


### js/import-attributes/non-type.js

**Prettier Similarity**: 100.00%


### js/import-attributes/quote-props/quoted-keys.js

**Prettier Similarity**: 100.00%


### js/import-attributes/re-export.js

**Prettier Similarity**: 100.00%


### js/import-attributes/static-import.js

**Prettier Similarity**: 100.00%


### js/import-attributes/without-from.js

**Prettier Similarity**: 100.00%


### js/import-meta/import_meta.js

**Prettier Similarity**: 100.00%


### js/import/brackets.js

**Prettier Similarity**: 100.00%


### js/import/comments.js

**Prettier Similarity**: 100.00%


### js/import/empty-import.js
```diff
 import {} from "@types/googlemaps";
 import "a";
 import /* comment */ "a";
 import // comment
 "a";
+import {/* comment */} from "a";
+import /* comment */ {} from "a";
+import {} /* comment */ from "a";
 import {} from /* comment */ "a";
-import {} from /* comment */ "a";
-import {} from /* comment */ "a";
-import {} from /* comment */ "a";
-import {} from /* comment */ /* comment */ /* comment */ /* comment */ "a";
+import /* comment */ {/* comment */} /* comment */ from /* comment */ "a";
+import {
+  // comment
+} from "a";
+import // comment
+{} from "a";
 import {} from // comment
 "a";
 import {} from // comment
 "a";
-import {} from // comment
-"a";
-import {} from // comment
-"a";
-import {} from // comment
-// comment
-// comment
+import // comment
+{
+  // comment
+} from // comment
 // comment
 "a";
 
 import // {} from
 "a";
 import {} from // comment ends with from
 "a";
 import {} from /* comment ends with from */ "a";
 import {} from // comment not ends with from ___
 "a";
 import {} from /* comment not ends with from ___ */ "a";
 
 import // comment ends with from
 "a";
 import /* comment ends with from */ "a";
 import // comment not ends with from ___
 "a";
 import /* comment not ends with from ___ */ "a";

```

**Prettier Similarity**: 68.29%


### js/import/inline.js

**Prettier Similarity**: 100.00%


### js/import/long-line.js

**Prettier Similarity**: 100.00%


### js/import/multiple_standalones.js

**Prettier Similarity**: 100.00%


### js/import/same-local-and-imported.js

**Prettier Similarity**: 100.00%


### js/in/arrow-function-invalid.js

**Prettier Similarity**: 100.00%


### js/in/arrow-function.js

**Prettier Similarity**: 100.00%


### js/invalid-code/duplicate_bindings.js

**Prettier Similarity**: 100.00%


### js/label/block-statement-and-regexp.js

**Prettier Similarity**: 100.00%


### js/label/comment.js

**Prettier Similarity**: 100.00%


### js/label/empty_label.js

**Prettier Similarity**: 100.00%


### js/last-argument-expansion/arrow.js

**Prettier Similarity**: 100.00%


### js/last-argument-expansion/assignment-pattern.js

**Prettier Similarity**: 100.00%


### js/last-argument-expansion/break-parent.js

**Prettier Similarity**: 100.00%


### js/last-argument-expansion/dangling-comment-in-arrow-function.js
```diff
-foo(() =>
-  // foo
-  {},
+foo(
+  (
+    // foo
+  ) => {},
 );

```

**Prettier Similarity**: 20.00%


### js/last-argument-expansion/edge_case.js

**Prettier Similarity**: 100.00%


### js/last-argument-expansion/empty-lines.js

**Prettier Similarity**: 100.00%


### js/last-argument-expansion/empty-object.js

**Prettier Similarity**: 100.00%


### js/last-argument-expansion/function-body-in-mode-break.js

**Prettier Similarity**: 100.00%


### js/last-argument-expansion/function-expression-issue-2239.js

**Prettier Similarity**: 100.00%


### js/last-argument-expansion/function-expression.js

**Prettier Similarity**: 100.00%


### js/last-argument-expansion/issue-10708.js

**Prettier Similarity**: 100.00%


### js/last-argument-expansion/issue-7518.js

**Prettier Similarity**: 100.00%


### js/last-argument-expansion/jsx.js

**Prettier Similarity**: 100.00%


### js/last-argument-expansion/number-only-array.js

**Prettier Similarity**: 100.00%


### js/last-argument-expansion/object.js

**Prettier Similarity**: 100.00%


### js/last-argument-expansion/overflow.js

**Prettier Similarity**: 100.00%


### js/line-suffix-boundary/boundary.js

**Prettier Similarity**: 100.00%


### js/line/windows.js

**Prettier Similarity**: 100.00%


### js/literal-numeric-separator/test.js

**Prettier Similarity**: 100.00%


### js/literal/number.js

**Prettier Similarity**: 100.00%


### js/logical-assignment/logical-assignment.js

**Prettier Similarity**: 100.00%


### js/logical_expressions/issue-7024.js

**Prettier Similarity**: 100.00%


### js/logical_expressions/logical_expression_operators.js

**Prettier Similarity**: 100.00%


### js/member/conditional.js

**Prettier Similarity**: 100.00%


### js/member/expand.js

**Prettier Similarity**: 100.00%


### js/member/logical.js

**Prettier Similarity**: 100.00%


### js/method-chain/13018.js

**Prettier Similarity**: 100.00%


### js/method-chain/assignment-lhs.js

**Prettier Similarity**: 100.00%


### js/method-chain/bracket_0-1.js

**Prettier Similarity**: 100.00%


### js/method-chain/bracket_0.js

**Prettier Similarity**: 100.00%


### js/method-chain/break-last-call.js

**Prettier Similarity**: 100.00%


### js/method-chain/break-last-member.js

**Prettier Similarity**: 100.00%


### js/method-chain/break-multiple.js

**Prettier Similarity**: 100.00%


### js/method-chain/comment.js

**Prettier Similarity**: 100.00%


### js/method-chain/complex-args.js

**Prettier Similarity**: 100.00%


### js/method-chain/computed-merge.js

**Prettier Similarity**: 100.00%


### js/method-chain/computed.js

**Prettier Similarity**: 100.00%


### js/method-chain/conditional.js

**Prettier Similarity**: 100.00%


### js/method-chain/cypress.js

**Prettier Similarity**: 100.00%


### js/method-chain/d3.js

**Prettier Similarity**: 100.00%


### js/method-chain/first_long.js

**Prettier Similarity**: 100.00%


### js/method-chain/fluent-configuration.js

**Prettier Similarity**: 100.00%


### js/method-chain/inline_merge.js

**Prettier Similarity**: 100.00%


### js/method-chain/issue-11298.js

**Prettier Similarity**: 100.00%


### js/method-chain/issue-3594.js

**Prettier Similarity**: 100.00%


### js/method-chain/issue-3621.js

**Prettier Similarity**: 100.00%


### js/method-chain/issue-4125.js

**Prettier Similarity**: 100.00%


### js/method-chain/logical.js

**Prettier Similarity**: 100.00%


### js/method-chain/multiple-members.js

**Prettier Similarity**: 100.00%


### js/method-chain/object-literal.js

**Prettier Similarity**: 100.00%


### js/method-chain/pr-7889.js

**Prettier Similarity**: 100.00%


### js/method-chain/print-width-120/constructor.js

**Prettier Similarity**: 100.00%


### js/method-chain/print-width-120/issue-7884.js

**Prettier Similarity**: 100.00%


### js/method-chain/short-names.js

**Prettier Similarity**: 100.00%


### js/method-chain/simple-args.js

**Prettier Similarity**: 100.00%


### js/method-chain/square_0.js

**Prettier Similarity**: 100.00%


### js/method-chain/test.js

**Prettier Similarity**: 100.00%


### js/method-chain/this.js

**Prettier Similarity**: 100.00%


### js/module-string-names/module-string-names-export.js

**Prettier Similarity**: 100.00%


### js/module-string-names/module-string-names-import.js

**Prettier Similarity**: 100.00%


### js/new-expression/call.js

**Prettier Similarity**: 100.00%


### js/new-expression/new_expression.js

**Prettier Similarity**: 100.00%


### js/new-expression/with-member-expression.js

**Prettier Similarity**: 100.00%


### js/new-target/outside-functions.js

**Prettier Similarity**: 100.00%


### js/new-target/range.js

**Prettier Similarity**: 100.00%


### js/newline/backslash_2028.js

**Prettier Similarity**: 100.00%


### js/newline/backslash_2029.js

**Prettier Similarity**: 100.00%


### js/no-semi/class.js

**Prettier Similarity**: 100.00%


### js/no-semi/comments.js

**Prettier Similarity**: 100.00%


### js/no-semi/issue2006.js

**Prettier Similarity**: 100.00%


### js/no-semi/no-semi.js

**Prettier Similarity**: 100.00%


### js/no-semi/private-field.js

**Prettier Similarity**: 100.00%


### js/non-strict/argument-name-clash.js

**Prettier Similarity**: 100.00%


### js/non-strict/keywords.js

**Prettier Similarity**: 100.00%


### js/non-strict/octal-number.js

**Prettier Similarity**: 100.00%


### js/nullish-coalescing/nullish_coalesing_operator.js

**Prettier Similarity**: 100.00%


### js/numeric-separators/number.js

**Prettier Similarity**: 100.00%


### js/object-colon-bug/bug.js

**Prettier Similarity**: 100.00%


### js/object-prop-break-in/comment.js

**Prettier Similarity**: 100.00%


### js/object-prop-break-in/long-value.js

**Prettier Similarity**: 100.00%


### js/object-prop-break-in/short-keys.js

**Prettier Similarity**: 100.00%


### js/object-prop-break-in/test.js

**Prettier Similarity**: 100.00%


### js/object-property-comment/after-key.js

**Prettier Similarity**: 100.00%


### js/object-property-ignore/ignore.js

**Prettier Similarity**: 100.00%


### js/object-property-ignore/issue-5678.js

**Prettier Similarity**: 100.00%


### js/objects/assignment-expression/object-property.js
```diff
 a = {
-  [(this.resource = resource)]: 1,
+  [this.resource = resource]: 1,
 };

```

**Prettier Similarity**: 66.67%


### js/objects/assignment-expression/object-value.js

**Prettier Similarity**: 100.00%


### js/objects/bigint-key.js

**Prettier Similarity**: 100.00%


### js/objects/escape-sequence-key.js

**Prettier Similarity**: 100.00%


### js/objects/expand.js

**Prettier Similarity**: 100.00%


### js/objects/getter-setter.js

**Prettier Similarity**: 100.00%


### js/objects/method.js

**Prettier Similarity**: 100.00%


### js/objects/range.js

**Prettier Similarity**: 100.00%


### js/objects/right-break.js

**Prettier Similarity**: 100.00%


### js/optional-catch-binding/optional_catch_binding.js

**Prettier Similarity**: 100.00%


### js/optional-chaining-assignment/valid-complex-case.js

**Prettier Similarity**: 100.00%


### js/optional-chaining-assignment/valid-lhs-eq.js

**Prettier Similarity**: 100.00%


### js/optional-chaining-assignment/valid-lhs-plus-eq.js

**Prettier Similarity**: 100.00%


### js/optional-chaining/chaining.js

**Prettier Similarity**: 100.00%


### js/optional-chaining/comments.js

**Prettier Similarity**: 100.00%


### js/optional-chaining/eval.js

**Prettier Similarity**: 100.00%


### js/performance/nested-real.js

**Prettier Similarity**: 100.00%


### js/performance/nested.js

**Prettier Similarity**: 100.00%


### js/preserve-line/argument-list.js

**Prettier Similarity**: 100.00%


### js/preserve-line/comments.js

**Prettier Similarity**: 100.00%


### js/preserve-line/member-chain.js
```diff
 fooBar
   .doSomething("Hello World")
   .doAnotherThing("Foo", { foo: bar })
 
   // App configuration.
   .doOneMoreThing(config)
 
   .run(() => console.log("Bar"));
 
 bigDeal
 
   .doSomething("Hello World")
 
   // Hello world
   .doAnotherThing("Foo", { foo: bar })
 
   // App configuration.
   .doOneMoreThing(config)
 
   .run(() => console.log("Bar"));
 
 foo.bar.baz
 
   .doSomething("Hello World")
 
   // Hello world
   .foo.bar.doAnotherThing("Foo", { foo: bar })
 
   .doOneMoreThing(config)
   .bar.run(() => console.log("Bar"));
 
 (somethingGood ? thisIsIt : maybeNot)
 
   // Hello world
   .doSomething("Hello World")
 
   .doAnotherThing("Foo", { foo: bar }) // Run this
   .run(() => console.log("Bar")); // Do this
 
 helloWorld
 
   .text()
 
   .then((t) => t);
 
 (
   veryLongVeryLongVeryLong ||
   anotherVeryLongVeryLongVeryLong ||
   veryVeryVeryLongError
 )
 
   .map((tickets) => TicketRecord.createFromSomeLongString())
 
   .filter((obj) => !!obj);
 
 const sel = this.connections
 
   .concat(this.activities.concat(this.operators))
   .filter((x) => x.selected);
 
-Object.entries(obj)
+Object.entries(obj).forEach((e) => console.log(e));
 
-  .forEach((e) => console.log(e));
-
-this.fetch("/foo")
-
-  .then((response) => response.json());
+this.fetch("/foo").then((response) => response.json());

```

**Prettier Similarity**: 91.04%


### js/preserve-line/parameter-list.js

**Prettier Similarity**: 100.00%


### js/private-in/private-in.js

**Prettier Similarity**: 100.00%


### js/quote-props/classes.js

**Prettier Similarity**: 100.00%


### js/quote-props/numeric-separator.js

**Prettier Similarity**: 100.00%


### js/quote-props/objects.js

**Prettier Similarity**: 100.00%


### js/quote-props/with_member_expressions.js

**Prettier Similarity**: 100.00%


### js/quote-props/with_numbers.js

**Prettier Similarity**: 100.00%


### js/quotes/functions.js

**Prettier Similarity**: 100.00%


### js/quotes/objects.js
```diff
 const obj = {
   a: true,
   b: true,
-  "𐊧": true,
+  𐊧: true,
 };

```

**Prettier Similarity**: 80.00%


### js/quotes/strings.js

**Prettier Similarity**: 100.00%


### js/range/array.js

**Prettier Similarity**: 100.00%


### js/range/boundary-2.js
```diff
 function a(
 ){
-  a();
-  b();
-  c();
-  d();
+a (
+);
+b();
+c(); d(
+);
+
 }

```

**Prettier Similarity**: 33.33%


### js/range/boundary-3.js
```diff
 a (
 );
-b (
-);                 c (
-); d(
+b();
+c(); d(
 );

```

**Prettier Similarity**: 50.00%


### js/range/boundary.js
```diff
-foo = 1.0000;bar = 1.0;
-baz = 1.0;
+foo = 1.0000;bar = 1.0;baz=1.0000;
 // The range will be 13~26
 // `foo` ends at 13, should not format
 // `bar` ends at 26, should format

```

**Prettier Similarity**: 60.00%


### js/range/class-declaration.js
```diff
+
+
 class a {
   b() {}
 }
 
-let x;
+let    x

```

**Prettier Similarity**: 57.14%


### js/range/different-levels.js

**Prettier Similarity**: 100.00%


### js/range/directive.js

**Prettier Similarity**: 100.00%


### js/range/function-body.js

**Prettier Similarity**: 100.00%


### js/range/function-declaration.js

**Prettier Similarity**: 100.00%


### js/range/ignore-indentation.js

**Prettier Similarity**: 100.00%


### js/range/issue-3789-1.js

**Prettier Similarity**: 100.00%


### js/range/issue-3789-2.js

**Prettier Similarity**: 100.00%


### js/range/issue-4206-1.js

**Prettier Similarity**: 100.00%


### js/range/issue-4206-2.js

**Prettier Similarity**: 100.00%


### js/range/issue-4206-3.js

**Prettier Similarity**: 100.00%


### js/range/issue-4206-4.js

**Prettier Similarity**: 100.00%


### js/range/large-dict.js

**Prettier Similarity**: 100.00%


### js/range/module-export1.js

**Prettier Similarity**: 100.00%


### js/range/module-export2.js

**Prettier Similarity**: 100.00%


### js/range/module-export3.js

**Prettier Similarity**: 100.00%


### js/range/module-import.js

**Prettier Similarity**: 100.00%


### js/range/multiple-statements.js

**Prettier Similarity**: 100.00%


### js/range/multiple-statements2.js
```diff
 call(
   1, 2,3
 );
 
 call(1, 2, 3);
 
 call(1, 2, 3);
 
-call(1, 2, 3);
+call(
+  1, 2,3
+);

```

**Prettier Similarity**: 72.73%


### js/range/nested-print-width.js

**Prettier Similarity**: 100.00%


### js/range/nested.js

**Prettier Similarity**: 100.00%


### js/range/nested2.js

**Prettier Similarity**: 100.00%


### js/range/nested3.js
```diff
 try {
-  1;
-  if (condition) {
-    body;
-  }
+1;
+if (condition) {
+  body;
+}
 }
 catch (err) {}

```

**Prettier Similarity**: 42.86%


### js/range/object-expression.js

**Prettier Similarity**: 100.00%


### js/range/object-expression2.js

**Prettier Similarity**: 100.00%


### js/range/range-end.js

**Prettier Similarity**: 100.00%


### js/range/range-start.js

**Prettier Similarity**: 100.00%


### js/range/range.js

**Prettier Similarity**: 100.00%


### js/range/start-equals-end.js

**Prettier Similarity**: 100.00%


### js/range/try-catch.js

**Prettier Similarity**: 100.00%


### js/range/whitespace.js
```diff
- 

```

**Prettier Similarity**: 0.00%


### js/regex/d-flag.js

**Prettier Similarity**: 100.00%


### js/regex/multiple-flags.js

**Prettier Similarity**: 100.00%


### js/regex/regexp-modifiers.js

**Prettier Similarity**: 100.00%


### js/regex/test.js

**Prettier Similarity**: 100.00%


### js/regex/v-flag.js

**Prettier Similarity**: 100.00%


### js/require-amd/named-amd-module.js

**Prettier Similarity**: 100.00%


### js/require-amd/non-amd-define.js

**Prettier Similarity**: 100.00%


### js/require-amd/require.js

**Prettier Similarity**: 100.00%


### js/require/require.js

**Prettier Similarity**: 100.00%


### js/reserved-word/interfaces.js

**Prettier Similarity**: 100.00%


### js/rest/trailing-commas.js

**Prettier Similarity**: 100.00%


### js/return-outside-function/return-outside-function.js
```diff
-return (
-  someVeryLongStringA &&
-  someVeryLongStringB &&
-  someVeryLongStringC &&
-  someVeryLongStringD
-);
+return someVeryLongStringA && someVeryLongStringB && someVeryLongStringC && someVeryLongStringD

```

**Prettier Similarity**: 0.00%


### js/return/binaryish.js

**Prettier Similarity**: 100.00%


### js/return/comment.js

**Prettier Similarity**: 100.00%


### js/sequence-break/break.js

**Prettier Similarity**: 100.00%


### js/sequence-expression/export-default.js

**Prettier Similarity**: 100.00%


### js/sequence-expression/ignore.js
```diff
-+(
-  // prettier-ignore
-  ((
++// prettier-ignore
+(
+    (
       first
     )
     ,
     (
       last
-    ))
-);
+    )
+  );

```

**Prettier Similarity**: 50.00%


### js/sequence-expression/parenthesized.js
```diff
 console.log(
   /* 1 */
+
   /* 2 */
+
   /* 3 */
   (first,
   /* 4 */
   /* 5 */
   /* 6 */
+
   /* 7 */
   last),
   /* 8 */
   /* 9 */
   /* 10 */
 );

```

**Prettier Similarity**: 82.35%


### js/shebang/shebang-newline.js

**Prettier Similarity**: 100.00%


### js/shebang/shebang.js

**Prettier Similarity**: 100.00%


### js/sloppy-mode/delete-variable.js

**Prettier Similarity**: 100.00%


### js/sloppy-mode/eval-arguments-binding.js

**Prettier Similarity**: 100.00%


### js/sloppy-mode/eval-arguments.js

**Prettier Similarity**: 100.00%


### js/sloppy-mode/function-declaration-in-if.js

**Prettier Similarity**: 100.00%


### js/sloppy-mode/function-declaration-in-while.js
```diff
-while (false) function foo() {}
+while (false) function foo(){}

```

**Prettier Similarity**: 0.00%


### js/sloppy-mode/labeled-function-declaration.js

**Prettier Similarity**: 100.00%


### js/spread/spread.js

**Prettier Similarity**: 100.00%


### js/strings/escaped.js

**Prettier Similarity**: 100.00%


### js/strings/multiline-literal.js

**Prettier Similarity**: 100.00%


### js/strings/non-octal-eight-and-nine.js

**Prettier Similarity**: 100.00%


### js/strings/strings.js

**Prettier Similarity**: 100.00%


### js/strings/template-literals.js

**Prettier Similarity**: 100.00%


### js/switch/comments.js

**Prettier Similarity**: 100.00%


### js/switch/comments2.js

**Prettier Similarity**: 100.00%


### js/switch/empty_lines.js

**Prettier Similarity**: 100.00%


### js/switch/empty_statement.js

**Prettier Similarity**: 100.00%


### js/switch/empty_switch.js

**Prettier Similarity**: 100.00%


### js/switch/switch.js

**Prettier Similarity**: 100.00%


### js/tab-width/class.js

**Prettier Similarity**: 100.00%


### js/tab-width/nested-functions.spec.js

**Prettier Similarity**: 100.00%


### js/template-align/indent.js

**Prettier Similarity**: 100.00%


### js/template-literals/binary-exporessions.js

**Prettier Similarity**: 100.00%


### js/template-literals/conditional-expressions.js

**Prettier Similarity**: 100.00%


### js/template-literals/expressions.js
```diff
 const long1 = `long ${
   a.b //comment
 } long longlong ${a.b.c.d.e} long longlong ${a.b.c.d.e} long longlong ${a.b.c.d.e} long long`;
 const long2 = `long ${a.b.c.d.e} long longlong ${loooooooooooooooooong} long longlong ${loooooooooooooooooong} long longlong ${loooooooooooooooooong} long long`;
 
 const long3 = `long long long long long long long long long long long ${a.b.c.d.e} long long long long long long long long long long long long long`;
 
 const description = `The value of the ${cssName} css of the ${this._name} element`;
 
 const foo = `such a long template string ${foo.bar.baz} that prettier will want to wrap it`;
 
 const shouldWrapForNow = `such a long template string ${foo().bar.baz} that prettier will want to wrap it`;
 
 const shouldNotWrap = `simple expressions should not break ${this} ${variable} ${a.b.c} ${this.b.c} ${a[b].c} ${a.b[c]} ${a.b["c"]} ${a?.b?.c}`;
 
 console.log(
   chalk.white(
     `Covered Lines below threshold: ${coverageSettings.lines}%. Actual: ${coverageSummary.total.lines.pct}%`,
   ),
 );
 
 x = `mdl-textfield mdl-js-textfield ${className} ${
   content.length > 0 ? "is-dirty" : ""
 } combo-box__input`;
 
 function testing() {
   const p = {};
   // faking some tabs since I can't paste my real code in
   if (true) {
     if (false) {
       return `${process.env.OPENID_URL}/something/something/something?${Object.keys(
         p,
       )
         .map((k) => `${encodeURIComponent(k)}=${encodeURIComponent(p[k])}`)
         .join("&")}`;
     }
   }
 }
 
 console.log(
   `Trying update appcast for ${app.name} (${app.cask.appcast}) -> (${app.cask.appcastGenerated})`,
 );
 
 console.log(
   `brew cask audit --download ${_.map(definitions, "caskName").join(" ")}`,
 );
 
 console.log(
   `\nApparently jetbrains changed the release artifact for ${app.name}@${app.jetbrains.version}.\n`,
 );
 
 descirbe("something", () => {
   test(`{pass: false} expect(${small}).toBeGreaterThanOrEqual(${big})`, () => {});
 });
 
 throw new Error(
   `pretty-format: Option "theme" has a key "${key}" whose value "${value}" is undefined in ansi-styles.`,
 );
 
-a = `${[
-  [1, 2, 3],
-  [4, 5, 6],
-]}`;
+a = `${[[1, 2, 3], [4, 5, 6]]}`;

```

**Prettier Similarity**: 93.65%


### js/template-literals/indention.js

**Prettier Similarity**: 100.00%


### js/template-literals/logical-expressions.js

**Prettier Similarity**: 100.00%


### js/template-literals/sequence-expressions.js

**Prettier Similarity**: 100.00%


### js/template/arrow.js

**Prettier Similarity**: 100.00%


### js/template/call.js

**Prettier Similarity**: 100.00%


### js/template/comment.js

**Prettier Similarity**: 100.00%


### js/template/faulty-locations.js

**Prettier Similarity**: 100.00%


### js/template/graphql.js

**Prettier Similarity**: 100.00%


### js/template/indent.js

**Prettier Similarity**: 100.00%


### js/template/inline.js

**Prettier Similarity**: 100.00%


### js/template/parenthesis.js

**Prettier Similarity**: 100.00%


### js/ternaries/binary.js

**Prettier Similarity**: 100.00%


### js/ternaries/func-call.js

**Prettier Similarity**: 100.00%


### js/ternaries/indent-after-paren.js

**Prettier Similarity**: 100.00%


### js/ternaries/indent.js

**Prettier Similarity**: 100.00%


### js/ternaries/nested-in-condition.js

**Prettier Similarity**: 100.00%


### js/ternaries/nested.js

**Prettier Similarity**: 100.00%


### js/ternaries/parenthesis.js

**Prettier Similarity**: 100.00%


### js/ternaries/parenthesis/await-expression.js
```diff
-stopDirectory = await (
-  useCache ? memoizedFindProjectRoot : findProjectRootWithoutCache
-)(path.dirname(path.resolve(filePath)));
+stopDirectory = await (useCache
+  ? memoizedFindProjectRoot
+  : findProjectRootWithoutCache)(path.dirname(path.resolve(filePath)));

```

**Prettier Similarity**: 0.00%


### js/ternaries/test.js

**Prettier Similarity**: 100.00%


### js/test-declarations/angular_async.js

**Prettier Similarity**: 100.00%


### js/test-declarations/angular_fakeAsync.js

**Prettier Similarity**: 100.00%


### js/test-declarations/angular_waitForAsync.js

**Prettier Similarity**: 100.00%


### js/test-declarations/angularjs_inject.js
```diff
 beforeEach(inject(($fooService, $barService) => {
   // code
 }));
 
 afterAll(inject(($fooService, $barService) => {
   console.log("Hello");
 }));
 
 it("should create the app", inject(($fooService, $barService) => {
   //code
 }));
 
 it("does something really long and complicated so I have to write a very long name for the test", inject(() => {
   // code
 }));
 
-it("does something really long and complicated so I have to write a very long name for the test", inject((
-  $fooServiceLongName,
-  $barServiceLongName,
-) => {
+it("does something really long and complicated so I have to write a very long name for the test", inject(($fooServiceLongName, $barServiceLongName) => {
   // code
 }));
 
 /*
  * isTestCall(parent) should only be called when parent exists
  * and parent.type is CallExpression. This test makes sure that
  * no errors are thrown when calling isTestCall(parent)
  */
 function x() {
   inject(() => {});
 }

```

**Prettier Similarity**: 87.10%


### js/test-declarations/jest-each-template-string.js

**Prettier Similarity**: 100.00%


### js/test-declarations/jest-each.js

**Prettier Similarity**: 100.00%


### js/test-declarations/optional.js
```diff
-describe?.(
-  "some string some string some string some string some string some string some string some string",
-  (done) => {},
-);
+describe?.("some string some string some string some string some string some string some string some string", (done) => {});

```

**Prettier Similarity**: 0.00%


### js/test-declarations/test_declarations.js

**Prettier Similarity**: 100.00%


### js/throw_statement/binaryish.js

**Prettier Similarity**: 100.00%


### js/throw_statement/comment.js

**Prettier Similarity**: 100.00%


### js/throw_statement/jsx.js

**Prettier Similarity**: 100.00%


### js/top-level-await/example.js

**Prettier Similarity**: 100.00%


### js/top-level-await/in-expression.js

**Prettier Similarity**: 100.00%


### js/trailing-comma/dynamic-import.js

**Prettier Similarity**: 100.00%


### js/trailing-comma/es5.js

**Prettier Similarity**: 100.00%


### js/trailing-comma/function-calls.js

**Prettier Similarity**: 100.00%


### js/trailing-comma/jsx.js

**Prettier Similarity**: 100.00%


### js/trailing-comma/object.js

**Prettier Similarity**: 100.00%


### js/trailing-comma/trailing_whitespace.js

**Prettier Similarity**: 100.00%


### js/try/catch.js

**Prettier Similarity**: 100.00%


### js/try/empty.js

**Prettier Similarity**: 100.00%


### js/try/try.js

**Prettier Similarity**: 100.00%


### js/unary-expression/comments.js

**Prettier Similarity**: 100.00%


### js/unary-expression/urnary_expression.js

**Prettier Similarity**: 100.00%


### js/unary/object.js

**Prettier Similarity**: 100.00%


### js/unary/series.js

**Prettier Similarity**: 100.00%


### js/unicode/combining-characters.js

**Prettier Similarity**: 100.00%


### js/unicode/keys.js

**Prettier Similarity**: 100.00%


### js/unicode/nbsp-jsx.js

**Prettier Similarity**: 100.00%


### js/update-expression/update_expression.js

**Prettier Similarity**: 100.00%


### js/variable_declarator/multiple.js

**Prettier Similarity**: 100.00%


### js/variable_declarator/string.js

**Prettier Similarity**: 100.00%


### js/while/indent.js

**Prettier Similarity**: 100.00%


### js/with/indent.js

**Prettier Similarity**: 100.00%


### js/yield/arrow.js

**Prettier Similarity**: 100.00%


### js/yield/conditional.js

**Prettier Similarity**: 100.00%


### js/yield/jsx-without-parenthesis.js

**Prettier Similarity**: 100.00%


### js/yield/jsx.js

**Prettier Similarity**: 100.00%


### jsx/attr-element/attr-element.js

**Prettier Similarity**: 100.00%


### jsx/binary-expressions/relational-operators.js

**Prettier Similarity**: 100.00%


### jsx/comments/eslint-disable.js

**Prettier Similarity**: 100.00%


### jsx/comments/in-attributes.js
```diff
 <div attr=/* comment */ "foo"></div>;
 
 <div attr=/* comment */
 "foo"></div>;
 
-<div attr /* comment */="foo"></div>;
+<div attr=/* comment */
+"foo"></div>;
 
 <div attr=/* comment */ "foo"></div>;
 
 <div attr=// comment
 "foo"></div>;
 
-<div attr="foo"></div>; // comment
+<div attr=// comment
+"foo"></div>;

```

**Prettier Similarity**: 73.33%


### jsx/comments/in-end-tag.js
```diff
 /* =========== before slash =========== */
 <a></
   // line
   a
 >;
 <a></ /* block */
 a>;
 
 <></
   // line
 >;
 <></ /* block */>;
 
 /* =========== after slash =========== */
 <a></
   // line
   a
 >;
 <a></ /* block */
 a>;
 
 <></
   // line
 >;
 <></ /* block */>;
 
 /* =========== after name =========== */
-<a></a>; // line
+<a></a // line
+>;
 <a></a /* block */>;
 
 /* =========== block =========== */
 <a></a /* block */>;
 <></ /* block */>;
 
 /* =========== multiple ===========  */
 <a></
   // line 1
   // line 2
   a
 >;
 <a></ /* block1 */ /* block2 */
 a>;
 <a></
   /* block */ // line
   a
 >;
 
 <></
   // line 1
   // line 2
 >;
 <></ /* block1 */
   /* block2 */>;
 <></
   /* block */
   // line
 >;

```

**Prettier Similarity**: 96.55%


### jsx/comments/in-tags.js

**Prettier Similarity**: 100.00%


### jsx/comments/jsx-tag-comment-after-prop.js

**Prettier Similarity**: 100.00%


### jsx/comments/like-a-comment-in-jsx-text.js

**Prettier Similarity**: 100.00%


### jsx/cursor/in-jsx-text.js

**Prettier Similarity**: 100.00%


### jsx/deprecated-jsx-bracket-same-line-option/jsx.js

**Prettier Similarity**: 100.00%


### jsx/embed/css-embed.js

**Prettier Similarity**: 100.00%


### jsx/escape/escape.js

**Prettier Similarity**: 100.00%


### jsx/escape/nbsp.js

**Prettier Similarity**: 100.00%


### jsx/fbt/test.js
```diff
 x = (
   <fbt>
-    <fbt:param>First</fbt:param>,
-    <fbt:param>Second</fbt:param>
+    <fbt:param>First</fbt:param>,<fbt:param>Second</fbt:param>
   </fbt>
 );
 
 x = (
   <fbt>
-    <fbt:param>First</fbt:param>
-    ,
-    <fbt:param>Second</fbt:param>
+    <fbt:param>First</fbt:param>,<fbt:param>Second</fbt:param>
   </fbt>
 );
 
 x = (
   <fbt>
     <fbt:param>First</fbt:param>,<fbt:param>Second</fbt:param>
   </fbt>
 );
 
 x = (
   <fbt>
     <fbt:param>First</fbt:param>,<fbt:param>Second</fbt:param>
   </fbt>
 );
 
 x = (
   <fbt desc="example 1">
     Prefix comes before
     <fbt:param>
       <b>suffix</b>
     </fbt:param>
   </fbt>
 );
 
 x = (
   <fbt desc="example 2">
     Prefix comes before
     <fbt:param name="bold stuff">
       <b>suffix</b>
     </fbt:param>
     <fbt:param name="a link">
       <link href="#">suffix</link>
     </fbt:param>
   </fbt>
 );
 
 x = (
   <fbt desc="example 3">
     Count Chocula knows the the number
     <fbt:enum enum-range={["one", "two", "three"]} value={getValue()} />
     is awesome
   </fbt>
 );
 
 x = (
   <fbt>
     {hour}:{minute}:{second}
   </fbt>
 );
 
 x = (
   <fbt>
-    {hour}
-    :
-    {minute}
-    :
-    {second}
+    {hour}:{minute}:{second}
   </fbt>
 );
 
 x = (
   <fbt>
-    {hour}:
-    {minute}:
-    {second}
+    {hour}:{minute}:{second}
   </fbt>
 );
 
 first = (
   <fbt>
-    Text<br />
-    More text<br />
-    And more<br />
+    Text
+    <br />
+    More text
+    <br />
+    And more
+    <br />
   </fbt>
 );
 
 second = (
   <fbt>
-    Text<br />More text<br />And more<br />
+    Text
+    <br />
+    More text
+    <br />
+    And more
+    <br />
   </fbt>
 );
 
 third = (
   <fbt>
     Text
     <br />
     More text
     <br />
     And more
     <br />
   </fbt>
 );

```

**Prettier Similarity**: 83.65%


### jsx/fragment/fragment.js

**Prettier Similarity**: 100.00%


### jsx/ignore/jsx_ignore.js

**Prettier Similarity**: 100.00%


### jsx/jsx/array-iter.js

**Prettier Similarity**: 100.00%


### jsx/jsx/arrow.js

**Prettier Similarity**: 100.00%


### jsx/jsx/attr-comments.js

**Prettier Similarity**: 100.00%


### jsx/jsx/await.js
```diff
 async function testFunction() {
   const short = (
     <>
       {await Promise.all(hierarchyCriticism)}
       {await hierarchyCriticism.ic.me.oa.p}
       {await hierarchyCriticism}
 
       {Promise.all(hierarchyCriticism)}
       {hierarchyCriticism.ic.me.oa.p}
       {hierarchyCriticism}
     </>
   );
 
   const long = (
     <>
-      {await Promise.all(
-        hierarchyCriticismIncongruousCooperateMaterialEducationOriginalArticulateParameter,
-      )}
       {
+        await Promise.all(
+          hierarchyCriticismIncongruousCooperateMaterialEducationOriginalArticulateParameter,
+        )
+      }
+      {
         await hierarchyCriticism.IncongruousCooperate.MaterialEducation
           .OriginalArticulate.Parameter
       }
       {
         await hierarchyCriticismIncongruousCooperateMaterialEducationOriginalArticulateParameter
       }
 
       {Promise.all(
         hierarchyCriticismIncongruousCooperateMaterialEducationOriginalArticulateParameter,
       )}
       {
         hierarchyCriticism.IncongruousCooperate.MaterialEducation
           .OriginalArticulate.Parameter
       }
       {
         hierarchyCriticismIncongruousCooperateMaterialEducationOriginalArticulateParameter
       }
     </>
   );
 
   const jsx = (
     <>
-      {await (
-        <IncongruousCooperate>
-          material education original articulate parameter
-        </IncongruousCooperate>
-      )}
+      {
+        await (
+          <IncongruousCooperate>
+            material education original articulate parameter
+          </IncongruousCooperate>
+        )
+      }
     </>
   );
 }

```

**Prettier Similarity**: 77.36%


### jsx/jsx/conditional-expression.js

**Prettier Similarity**: 100.00%


### jsx/jsx/expression.js

**Prettier Similarity**: 100.00%


### jsx/jsx/flow_fix_me.js

**Prettier Similarity**: 100.00%


### jsx/jsx/html_escape.js

**Prettier Similarity**: 100.00%


### jsx/jsx/hug.js

**Prettier Similarity**: 100.00%


### jsx/jsx/logical-expression.js

**Prettier Similarity**: 100.00%


### jsx/jsx/object-property.js

**Prettier Similarity**: 100.00%


### jsx/jsx/open-break.js

**Prettier Similarity**: 100.00%


### jsx/jsx/parens.js

**Prettier Similarity**: 100.00%


### jsx/jsx/quotes.js
```diff
 <div id="&quot;'<>&amp;quot;" />;
 <div id='"&#39;<>&amp;quot;' />;
 <div id={"'\"&quot;<>&amp;quot;"} />;
 <div id="123" />;
-<div id='&#39;"' />;
+<div id="&#39;&quot;" />;
 <div id={"'\"\\'"} />;
 <div
   single="foo"
   single2={"foo"}
   double="bar"
   double2={"bar"}
   singleDouble='"'
   singleDouble2={'"'}
   doubleSingle="'"
   doubleSingle2={"'"}
   singleEscaped={"'"}
-  singleEscaped2="'"
+  singleEscaped2="&apos;"
   doubleEscaped={'"'}
-  doubleEscaped2='"'
+  doubleEscaped2="&quot;"
   singleBothEscaped={"'\""}
-  singleBothEscaped2="'&quot;"
-  singleBoth="' &quot;"
+  singleBothEscaped2='&apos;"'
+  singleBoth='&apos; "'
   singleBoth2={"' \""}
-  singleBoth3="' ' &quot;"
+  singleBoth3='&apos; &apos; "'
   doubleBoth="&quot; '"
   doubleBoth2={"\" '"}
-  doubleBoth3="&quot; ' '"
+  doubleBoth3="&quot; &apos; '"
 />;
 
 <p>
   GitHub Desktop has encountered an unrecoverable error and will need to 1231231
   restart. This has been reported to the team, but if youencounter this121312331
   repeatedly please report this issue to the GitHub 12312312312312313{"  "}{" "}
 </p>;

```

**Prettier Similarity**: 79.41%


### jsx/jsx/regex.js

**Prettier Similarity**: 100.00%


### jsx/jsx/return-statement.js

**Prettier Similarity**: 100.00%


### jsx/jsx/self-closing.js

**Prettier Similarity**: 100.00%


### jsx/jsx/spacing.js

**Prettier Similarity**: 100.00%


### jsx/jsx/template-literal-in-attr.js

**Prettier Similarity**: 100.00%


### jsx/jsx/ternary.js

**Prettier Similarity**: 100.00%


### jsx/last-line/last_line.js

**Prettier Similarity**: 100.00%


### jsx/last-line/single_prop_multiline_string.js

**Prettier Similarity**: 100.00%


### jsx/multiline-assign/test.js

**Prettier Similarity**: 100.00%


### jsx/namespace/jsx_namespaced_name.js

**Prettier Similarity**: 100.00%


### jsx/newlines/test.js

**Prettier Similarity**: 100.00%


### jsx/newlines/windows.js

**Prettier Similarity**: 100.00%


### jsx/optional-chaining/optional-chaining.jsx

**Prettier Similarity**: 100.00%


### jsx/significant-space/comments.js

**Prettier Similarity**: 100.00%


### jsx/significant-space/test.js

**Prettier Similarity**: 100.00%


### jsx/single-attribute-per-line/single-attribute-per-line.js

**Prettier Similarity**: 100.00%


### jsx/split-attrs/test.js

**Prettier Similarity**: 100.00%


### jsx/spread/attribute.js

**Prettier Similarity**: 100.00%


### jsx/spread/child.js

**Prettier Similarity**: 100.00%


### jsx/stateless-arrow-fn/test.js

**Prettier Similarity**: 100.00%


### jsx/text-wrap/test.js

**Prettier Similarity**: 100.00%


### typescript/abstract-class/export-default.ts

**Prettier Similarity**: 100.00%


### typescript/abstract-construct-types/abstract-construct-types.ts

**Prettier Similarity**: 100.00%


### typescript/abstract-property/semicolon.ts

**Prettier Similarity**: 100.00%


### typescript/ambient/ambient.ts

**Prettier Similarity**: 100.00%


### typescript/angular-component-examples/15934-computed.component.ts

**Prettier Similarity**: 100.00%


### typescript/angular-component-examples/15934.component.ts

**Prettier Similarity**: 100.00%


### typescript/angular-component-examples/15969-computed.component.ts

**Prettier Similarity**: 100.00%


### typescript/angular-component-examples/test.component.ts

**Prettier Similarity**: 100.00%


### typescript/argument-expansion/argument_expansion.ts

**Prettier Similarity**: 100.00%


### typescript/argument-expansion/arrow-with-return-type.ts

**Prettier Similarity**: 100.00%


### typescript/array/comment.ts

**Prettier Similarity**: 100.00%


### typescript/array/key.ts

**Prettier Similarity**: 100.00%


### typescript/arrow/16067.ts
```diff
 const foo1 =
   // comment
-
-    <T,>() =>
+    <T>() =>
     () =>
       1;
 
 const foo2 =
   // comment
   () => () => 1;
 
 const foo3 =
   // comment
-  <T,>() => 1;
+  <T>() => 1;
 
 foo(
   // comment
-  <T,>() =>
+  <T>() =>
     () =>
       1,
 );
 
 a ||
   // comment
-  (<T,>() =>
+  (<T>() =>
     () =>
       1);
 
 void (
   // comment
-  (<T,>() =>
+  (<T>() =>
     () =>
       1)
 );
 
 cond
   ? // comment
-    <T,>() =>
+    <T>() =>
       () =>
         1
   : // comment
-    <T,>() =>
+    <T>() =>
       () =>
         1;
 
 foo4 =
   // comment
-
-    <T,>() =>
+    <T>() =>
     () =>
       1;

```

**Prettier Similarity**: 80.39%


### typescript/arrow/arrow_regression.ts

**Prettier Similarity**: 100.00%


### typescript/arrow/comments.ts
```diff
 const fn1 = () => {
   return;
-}; /* foo */
+} /* foo */;
 
 const fn2 = () => {
   return;
 };
 
 // foo

```

**Prettier Similarity**: 88.89%


### typescript/arrow/issue-6107-curry.ts

**Prettier Similarity**: 100.00%


### typescript/arrows/arrow_function_expression.ts

**Prettier Similarity**: 100.00%


### typescript/arrows/short_body.ts

**Prettier Similarity**: 100.00%


### typescript/arrows/type_params.ts

**Prettier Similarity**: 100.00%


### typescript/as/array-pattern.ts

**Prettier Similarity**: 100.00%


### typescript/as/as.ts

**Prettier Similarity**: 100.00%


### typescript/as/assignment.ts

**Prettier Similarity**: 100.00%


### typescript/as/assignment2.ts

**Prettier Similarity**: 100.00%


### typescript/as/export_default_as.ts

**Prettier Similarity**: 100.00%


### typescript/as/expression-statement.ts

**Prettier Similarity**: 100.00%


### typescript/as/long-identifiers.ts

**Prettier Similarity**: 100.00%


### typescript/as/nested-await-and-as.ts

**Prettier Similarity**: 100.00%


### typescript/as/return.ts

**Prettier Similarity**: 100.00%


### typescript/as/ternary.ts

**Prettier Similarity**: 100.00%


### typescript/assert/comment.ts

**Prettier Similarity**: 100.00%


### typescript/assert/index.ts

**Prettier Similarity**: 100.00%


### typescript/assignment/issue-10846.ts

**Prettier Similarity**: 100.00%


### typescript/assignment/issue-10848.tsx

**Prettier Similarity**: 100.00%


### typescript/assignment/issue-10850.ts

**Prettier Similarity**: 100.00%


### typescript/assignment/issue-12413.ts

**Prettier Similarity**: 100.00%


### typescript/assignment/issue-2322.ts

**Prettier Similarity**: 100.00%


### typescript/assignment/issue-2482.ts

**Prettier Similarity**: 100.00%


### typescript/assignment/issue-2485.ts

**Prettier Similarity**: 100.00%


### typescript/assignment/issue-3122.ts

**Prettier Similarity**: 100.00%


### typescript/assignment/issue-5370.ts
```diff
-const durabilityMetricsSelectable: Immutable.OrderedSet<SomeReportingMetric> =
-  myExperienceSelectable.concat(otherDurabilityMetricsSelectable);
+const durabilityMetricsSelectable: Immutable.OrderedSet<
+  SomeReportingMetric,
+> = myExperienceSelectable.concat(otherDurabilityMetricsSelectable);

```

**Prettier Similarity**: 0.00%


### typescript/assignment/issue-6783.ts

**Prettier Similarity**: 100.00%


### typescript/assignment/issue-8619.ts

**Prettier Similarity**: 100.00%


### typescript/assignment/issue-9172.ts

**Prettier Similarity**: 100.00%


### typescript/assignment/lone-arg.ts

**Prettier Similarity**: 100.00%


### typescript/assignment/parenthesized.ts

**Prettier Similarity**: 100.00%


### typescript/bigint/bigint.ts

**Prettier Similarity**: 100.00%


### typescript/break-calls/type_args.ts

**Prettier Similarity**: 100.00%


### typescript/call-signature/call-signature.ts

**Prettier Similarity**: 100.00%


### typescript/cast/as-const.ts

**Prettier Similarity**: 100.00%


### typescript/cast/assert-and-assign.ts

**Prettier Similarity**: 100.00%


### typescript/cast/generic-cast.ts

**Prettier Similarity**: 100.00%


### typescript/cast/hug-args.ts

**Prettier Similarity**: 100.00%


### typescript/cast/parenthesis.ts

**Prettier Similarity**: 100.00%


### typescript/cast/tuple-and-record.ts
```diff
 breakAfterCast = <PermissionsChecker<any> | undefined>(
   (<any>permissions)[receiverType]
 );
-breakAfterCast = <PermissionsChecker<any> | undefined>(
-  (<any>permissions)(#[receiverType])
-);
+breakAfterCast = <PermissionsChecker<any> | undefined>(<any>permissions)(#[receiverType]);
 
 testObjLiteral = <PermissionsChecker<any> | undefined>{ prop1: "myPropVal" };
-testObjLiteral = <PermissionsChecker<any> | undefined>#{ prop1: "myPropVal" };
+testObjLiteral =  <PermissionsChecker<any> | undefined>
+#
+{
+  prop1: "myPropVal";
+}

```

**Prettier Similarity**: 45.45%


### typescript/catch-clause/type-annotation.ts

**Prettier Similarity**: 100.00%


### typescript/chain-expression/call-expression.ts
```diff
 // Member expressions
 a?.b!();
 a?.b!();
 (a!?.b)();
 a.b?.c!();
 a.b?.c!();
 (a.b!?.c)();
 (a!.b?.c)();
 a?.b.c!();
 a?.b.c!();
-(a?.b!.c)();
+a?.b!.c();
 (a!?.b.c)();
 a[b?.c]!();
 a[b?.c]!();
 a[b?.c!]();
 a[b!?.c]();
 (a?.b).c!();
 (a?.b).c!();
 // FIXME: ((a?.b!).c)   ();
 (a!?.b).c();
 a[b?.()]!();
 a[b?.()]!();
 a[b?.()!]();
 a[b!?.()]();
 a![b?.()]();
 (a?.b).c!();
 (a?.b).c!();
 // FIXME: ((a?.b)!.c)   ();
 // FIXME: ((a?.b!).c)   ();
 (a!?.b).c();
 (a?.()).b!();
 (a?.()).b!();
-(a?.())!.b();
-(a?.())!.b();
+a?.()!.b();
+a?.()!.b();
 (a!?.()).b();
 
 // Call expressions
 a?.()!();
 a?.()!();
 (a!?.())();
 a.b.c?.()!();
 a.b.c?.()!();
 (a.b.c!?.())();
 a.b?.c()!();
 a.b?.c()!();
 (a.b!?.c())();
 a?.b.c()!();
 a?.b.c()!();
-(a?.b!.c())();
+a?.b!.c()();
 a(b?.c)!();
 a(b?.c)!();
 a(b?.c!)();
 (a?.b)()!();
 (a?.b)()!();
 a?.b!()();
 a?.b!()();
 (a?.())()!();
 (a?.())()!();
 a?.()!()();
 a?.()!()();
 (a!?.())()();
 
 // Not `.callee`
 foo(a?.b!);

```

**Prettier Similarity**: 93.75%


### typescript/chain-expression/member-expression.ts
```diff
 // Member expressions
-(a?.b)!.foo;
-(a?.b)!.foo;
+a?.b!.foo;
+a?.b!.foo;
 (a!?.b).foo;
-(a.b?.c)!.foo;
-(a.b?.c)!.foo;
+a.b?.c!.foo;
+a.b?.c!.foo;
 (a.b!?.c).foo;
 (a!.b?.c).foo;
-(a?.b.c)!.foo;
-(a?.b.c)!.foo;
-(a?.b!.c).foo;
+a?.b.c!.foo;
+a?.b.c!.foo;
+a?.b!.c.foo;
 (a!?.b.c).foo;
 a[b?.c]!.foo;
 a[b?.c]!.foo;
 a[b?.c!].foo;
 a[b!?.c].foo;
 (a?.b).c!.foo;
 (a?.b).c!.foo;
-(a?.b)!.c.foo;
+a?.b!.c.foo;
 (a!?.b).c.foo;
 a[b?.()]!.foo;
 a[b?.()]!.foo;
 a[b?.()!].foo;
 a[b!?.()].foo;
 a![b?.()].foo;
 (a?.b).c!.foo;
 (a?.b).c!.foo;
-(a?.b)!.c.foo;
-(a?.b)!.c.foo;
+a?.b!.c.foo;
+a?.b!.c.foo;
 (a!?.b).c.foo;
 (a?.()).b!.foo;
 (a?.()).b!.foo;
-(a?.())!.b.foo;
-(a?.())!.b.foo;
+a?.()!.b.foo;
+a?.()!.b.foo;
 (a!?.()).b.foo;
 
 // Call expressions
-(a?.())!.foo;
-(a?.())!.foo;
+a?.()!.foo;
+a?.()!.foo;
 (a!?.()).foo;
-(a.b.c?.())!.foo;
-(a.b.c?.())!.foo;
+a.b.c?.()!.foo;
+a.b.c?.()!.foo;
 (a.b.c!?.()).foo;
-(a.b?.c())!.foo;
-(a.b?.c())!.foo;
+a.b?.c()!.foo;
+a.b?.c()!.foo;
 (a.b!?.c()).foo;
-(a?.b.c())!.foo;
-(a?.b.c())!.foo;
-(a?.b!.c()).foo;
+a?.b.c()!.foo;
+a?.b.c()!.foo;
+a?.b!.c().foo;
 a(b?.c)!.foo;
 a(b?.c)!.foo;
 a(b?.c!).foo;
 (a?.b)()!.foo;
 (a?.b)()!.foo;
 a?.b!().foo;
 a?.b!().foo;
 (a?.())()!.foo;
 (a?.())()!.foo;
 a?.()!().foo;
 a?.()!().foo;
 (a!?.())().foo;
 
 // Not `.object`
 _[a?.b!](
   // Computed
   a?.b!,
 )[foo];

```

**Prettier Similarity**: 68.66%


### typescript/chain-expression/test.ts
```diff
-(a?.b)!.c;
-(a?.())!.b;
-(a?.b)!.c;
-(a?.())!.b;
+a?.b!.c;
+a?.()!.b;
+a?.b!.c;
+a?.()!.b;

```

**Prettier Similarity**: 0.00%


### typescript/chain-expression/test2.ts
```diff
 {
   {
-    const rotation1 = getTransformHandles(
-      arrow,
-      h.state.zoom,
-      "mouse",
-    ).rotation!;
+    const rotation1 = getTransformHandles(arrow, h.state.zoom, "mouse")
+      .rotation!;
     const rotation2 = getTransformHandles(
       arrow,
       h.state.zoom,
       "mouse",
     ).rotation;
     const rotation3 = getTransformHandles(
       arrow,
       h.state.zoom,
       "mouse",
     )?.rotation;
   }
 }

```

**Prettier Similarity**: 73.68%


### typescript/class-comment/class-implements.ts

**Prettier Similarity**: 100.00%


### typescript/class-comment/declare.ts

**Prettier Similarity**: 100.00%


### typescript/class-comment/generic.ts

**Prettier Similarity**: 100.00%


### typescript/class-comment/misc.ts

**Prettier Similarity**: 100.00%


### typescript/class/abstract-method.ts

**Prettier Similarity**: 100.00%


### typescript/class/constructor.ts
```diff
 class C {
   constructor(override a: number) {}
 }
 class D {
   constructor(private a: number) {}
 }
 class E {
   constructor(protected a: number) {}
 }
 class F {
   constructor(public a: number) {}
 }
 class G {
   constructor(readonly a: number) {}
 }
 
 class A {
-  "constructor": typeof A;
+  'constructor': typeof A
   static Foo() {
     return new A();
   }
 }
 
 class B {
   constructor<>() {}
 }

```

**Prettier Similarity**: 96.15%


### typescript/class/declare-readonly-field-initializer-w-annotation.ts

**Prettier Similarity**: 100.00%


### typescript/class/declare-readonly-field-initializer.ts

**Prettier Similarity**: 100.00%


### typescript/class/dunder.ts

**Prettier Similarity**: 100.00%


### typescript/class/empty-method-body.ts
```diff
 // #9324
 
 class foo1 {
-  bar /* bat */();
+  bar() /* bat */;
 }
 
 // #9367
 class Test {
-  foo /* 3 */(/* 2 */);
+  foo(/* 2 */) /* 3 */;
 }

```

**Prettier Similarity**: 80.00%


### typescript/class/extends_implements.ts

**Prettier Similarity**: 100.00%


### typescript/class/generics.ts

**Prettier Similarity**: 100.00%


### typescript/class/methods.ts

**Prettier Similarity**: 100.00%


### typescript/class/optional.ts

**Prettier Similarity**: 100.00%


### typescript/class/parameter-properties.ts

**Prettier Similarity**: 100.00%


### typescript/class/quoted-property.ts
```diff
 class User {
-  "username": string;
+  username: string;
 }

```

**Prettier Similarity**: 66.67%


### typescript/class/standard_private_fields.ts

**Prettier Similarity**: 100.00%


### typescript/classes/break-heritage.ts

**Prettier Similarity**: 100.00%


### typescript/classes/break.ts

**Prettier Similarity**: 100.00%


### typescript/comments-2/dangling.ts

**Prettier Similarity**: 100.00%


### typescript/comments-2/issues.ts

**Prettier Similarity**: 100.00%


### typescript/comments-2/last-arg.ts

**Prettier Similarity**: 100.00%


### typescript/comments/15707.ts

**Prettier Similarity**: 100.00%


### typescript/comments/16065-2.ts
```diff
 class Foo {
   // PropertyDefinition
-  @decorator /* comment */
-  readonly propertyDefinition;
+  @decorator
+  readonly /* comment */ propertyDefinition;
 
   // TSAbstractPropertyDefinition
-  @decorator /* comment */
-  abstract abstractPropertyDefinition;
+  @decorator
+  abstract /* comment */ abstractPropertyDefinition;
 
   // TSAbstractMethodDefinition
-  @decorator /* comment */
-  abstract abstractMethodDefinition;
+  @decorator
+  abstract /* comment */ abstractMethodDefinition;
 
   // MethodDefinition
-  @decorator /* comment */
-  private methodDefinition() {}
+  @decorator
+  private /* comment */ methodDefinition() {}
 
   // AccessorProperty
-  @decorator /* comment */
-  accessor accessorProperty = 3;
+  @decorator
+  accessor /* comment */ accessorProperty = 3;
 
   constructor(
     // TSParameterProperty
     @decorator
     readonly /* comment */ parameterProperty,
   ) {}
 }

```

**Prettier Similarity**: 62.96%


### typescript/comments/16065.ts
```diff
 class Foo {
   constructor(
     @decorator1
-    readonly // comment1
-    baz1: string,
+    // comment1
+    readonly baz1: string,
 
     @decorator2
-    private // comment2
-    baz2: string,
+    // comment2
+    private baz2: string,
   ) {}
 }

```

**Prettier Similarity**: 63.64%


### typescript/comments/abstract_class.ts

**Prettier Similarity**: 100.00%


### typescript/comments/abstract_methods.ts

**Prettier Similarity**: 100.00%


### typescript/comments/after_jsx_generic.tsx

**Prettier Similarity**: 100.00%


### typescript/comments/declare_function.ts
```diff
 declare function fn(
   currentRequest: { a: number },
   // TODO this is a very very very very long comment that makes it go > 80 columns
 ): number;
 
-declare function /* foo */ f(/* baz */ a /* taz */); /* bar */
+declare function /* foo */ f(/* baz */ a /* taz */) /* bar */;

```

**Prettier Similarity**: 83.33%


### typescript/comments/interface.ts

**Prettier Similarity**: 100.00%


### typescript/comments/issues.ts

**Prettier Similarity**: 100.00%


### typescript/comments/jsx.tsx

**Prettier Similarity**: 100.00%


### typescript/comments/location.ts

**Prettier Similarity**: 100.00%


### typescript/comments/mapped_types.ts
```diff
 type A = {
   // commentA
   [a in A]: string;
 };
 
 type B = {
   /* commentB */ [b in B]: string;
 };
 
 type C = {
   [/* commentC */ c in C]: string;
 };
 
 type D = {
   [d /* commentD */ in D]: string;
 };
 
 type E = {
   [e in /* commentE */ E]: string;
 };
 
 type F = {
   [f in F /* commentF */]: string;
 };
 
 type G = {
-  [g in G /* commentG */]: string;
+  [g in G] /* commentG */ : string;
 };
 
 type H = { [/* commentH */ h in H]: string };
 
 type I = { [/* commentI */ i in I]: string };
 
 type J = { [j /* commentJ */ in J]: string };
 
 type K = { [k in /* commentK */ K]: string };
 
 type L = { [l in L /* commentL */]: string };
 
-type M = { [m in M /* commentG */]: string };
+type M = { [m in M] /* commentG */ : string };

```

**Prettier Similarity**: 95.00%


### typescript/comments/method_types.ts
```diff
 interface foo1 {
   bar3 /* foo */(/* baz */); // bat
-  bar /* foo */ /* bar */?(/* baz */) /* bat */;
+  bar /* foo */?/* bar */ (/* baz */) /* bat */;
   bar2 /* foo */(/* baz */) /* bat */;
 }
 
 interface foo2 {
-  bar /* foo */?(/* bar */ bar: /* baz */ string): /* bat */ string;
+  bar /* foo */?/* bar */ (bar: /* baz */ string): /* bat */ string;
 }
 
 interface foo3 {
   /* foo */ (/* bar */): /* baz */ string;
 }
 
 interface foo4 {
   /* foo */ (bar: /* bar */ string): /* baz */ string;
 }
 
 interface foo5 {
-  /* foo */ new (/* bar */ a: /* baz */ string): /* bat */ string;
+  /* foo */ new /* bar */ (a: /* baz */ string): /* bat */ string;
 }
 
 interface foo6 {
-  /* foo */ new (/* baz */) /* bar */ : /* bat */ string;
+  /* foo */ new /* bar */ (/* baz */): /* bat */ string;
 }
 
 type foo7 = /* foo */ (/* bar */) /* baz */ => void;
 
 type foo8 = /* foo */ (a: /* bar */ string) /* baz */ => void;
 
-let foo9: new (/* bar */) /* foo */ /* baz */ => string;
+let foo9: new /* foo */ (/* bar */) /* baz */ => string;
 
-let foo10: new (/* foo */ a: /* bar */ string) /* baz */ => string;
+let foo10: new /* foo */ (a: /* bar */ string) /* baz */ => string;
 
 abstract class Test {
   abstract foo12 /* foo */(a: /* bar */ string): /* baz */ void;
 
   abstract foo13 /* foo */(/* bar */); /* baz */
 }

```

**Prettier Similarity**: 84.62%


### typescript/comments/methods.ts

**Prettier Similarity**: 100.00%


### typescript/comments/ts-parameter-proerty.ts

**Prettier Similarity**: 100.00%


### typescript/comments/type-parameters.ts
```diff
 functionName<A /* A comment */>();
 const a: T</* comment */> = 1;
 functionName</* comment */>();
 function foo</* comment */>() {}
 interface Foo {
-  </* comment */>(arg): any;
+ </* comment */>(arg): any;
 }
 type T = </* comment */>(arg) => any;
 
 functionName<A>(); // comment
 const a: T<
   // comment
 > = 1;
 functionName<
   // comment
 >();
 function foo<
   // comment
 >() {}
 interface Foo {
   <
     A, // comment
   >(
     arg,
   ): any;
 }
 type T = <
   // comment
->(
-  arg,
-) => any;
+>(arg) => any;

```

**Prettier Similarity**: 87.10%


### typescript/comments/type_literals.ts

**Prettier Similarity**: 100.00%


### typescript/comments/types.ts

**Prettier Similarity**: 100.00%


### typescript/comments/union.ts

**Prettier Similarity**: 100.00%


### typescript/compiler/ClassDeclaration22.ts

**Prettier Similarity**: 100.00%


### typescript/compiler/anyIsAssignableToObject.ts

**Prettier Similarity**: 100.00%


### typescript/compiler/castOfAwait.ts

**Prettier Similarity**: 100.00%


### typescript/compiler/castParentheses.ts

**Prettier Similarity**: 100.00%


### typescript/compiler/castTest.ts

**Prettier Similarity**: 100.00%


### typescript/compiler/checkInfiniteExpansionTermination.ts

**Prettier Similarity**: 100.00%


### typescript/compiler/commentInNamespaceDeclarationWithIdentifierPathName.ts

**Prettier Similarity**: 100.00%


### typescript/compiler/commentsInterface.ts

**Prettier Similarity**: 100.00%


### typescript/compiler/contextualSignatureInstantiation2.ts

**Prettier Similarity**: 100.00%


### typescript/compiler/declareDottedModuleName.ts

**Prettier Similarity**: 100.00%


### typescript/compiler/decrementAndIncrementOperators.ts

**Prettier Similarity**: 100.00%


### typescript/compiler/es5ExportDefaultClassDeclaration4.ts

**Prettier Similarity**: 100.00%


### typescript/compiler/functionOverloadsOnGenericArity1.ts

**Prettier Similarity**: 100.00%


### typescript/compiler/globalIsContextualKeyword.ts

**Prettier Similarity**: 100.00%


### typescript/compiler/indexSignatureWithInitializer.ts

**Prettier Similarity**: 100.00%


### typescript/compiler/mappedTypeWithCombinedTypeMappers.ts

**Prettier Similarity**: 100.00%


### typescript/compiler/modifiersOnInterfaceIndexSignature1.ts

**Prettier Similarity**: 100.00%


### typescript/compiler/privacyGloImport.ts

**Prettier Similarity**: 100.00%


### typescript/conditional-types/comments.ts

**Prettier Similarity**: 100.00%


### typescript/conditional-types/conditonal-types.ts

**Prettier Similarity**: 100.00%


### typescript/conditional-types/infer-type.ts

**Prettier Similarity**: 100.00%


### typescript/conditional-types/nested-in-condition.ts

**Prettier Similarity**: 100.00%


### typescript/conditional-types/new-ternary-spec.ts

**Prettier Similarity**: 100.00%


### typescript/conditional-types/parentheses.ts
```diff
 // #13275
 type Foo<T> = T extends ((...a: any[]) => infer R extends string) ? R : never;
-type Foo<T> = T extends (new (...a: any[]) => infer R extends string)
+type Foo<T> = T extends (new (
+  ...a: any[]
+) => infer R extends string)
   ? R
   : never;
 
 // #14275
-type Test<T> = T extends ((
+type Test<T> = T extends (
   token: TSESTree.Token,
-) => token is infer U extends TSESTree.Token)
+) => token is infer U extends TSESTree.Token
   ? U
   : TSESTree.Token;
 type Test<T> = T extends ((
   token: TSESTree.Token,
 ) => asserts token is infer U extends TSESTree.Token)
   ? U
   : TSESTree.Token;
 type Test<T> = T extends (new (
-  token: TSESTree.Token,
-) => token is infer U extends TSESTree.Token)
+  token: TSESTree.Token
+) => token is infer U extends TSESTree.Token
+)
   ? U
-  : TSESTree.Token;
+  : TSESTree.Token

```

**Prettier Similarity**: 64.00%


### typescript/conformance/ambient/ambientDeclarations.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/classes/abstract.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractAccessor.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractAsIdentifier.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractAssignabilityConstructorFunction.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractClinterfaceAssignability.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractConstructorAssignability.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractCrashedOnce.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractExtends.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractFactoryFunction.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractGeneric.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractImportInstantiation.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractInAModule.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractInheritance.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractInstantiations1.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractInstantiations2.ts
```diff
 class A {}
 
 abstract class B {
   foo(): number {
     return this.bar();
   }
   abstract bar(): number;
 }
 
 new B();
 
 var BB: typeof B = B;
 var AA: typeof A = BB;
 new AA();
 
 function constructB(Factory: typeof B) {
   new Factory();
 }
 
 var BB = B;
 new BB();
 
 var x: any = C;
 new x();
 
 class C extends B {}
 
 abstract class D extends B {}
 
 class E extends B {
   bar() {
     return 1;
   }
 }
 
 abstract class F extends B {
   abstract foo(): number;
   bar() {
     return 2;
   }
 }
 
 abstract class G {
   abstract qux(x: number): string;
   abstract qux(): number;
   y: number;
   abstract quz(x: number, y: string): boolean;
 
   abstract nom(): boolean;
   nom(x: number): boolean;
 }
 
 class H {
-  abstract baz(): number;
+  abstract baz() : number;
 }

```

**Prettier Similarity**: 98.18%


### typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractMethodInNonAbstractClass.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractMixedWithModifiers.ts
```diff
 abstract class A {
   abstract foo_a();
 
   public abstract foo_b();
   protected abstract foo_c();
   private abstract foo_d();
 
-  public abstract foo_bb();
-  protected abstract foo_cc();
-  private abstract foo_dd();
+  abstract public foo_bb();
+  abstract protected foo_cc();
+  abstract private foo_dd();
 
-  static abstract foo_d();
+  abstract static foo_d();
 
   static abstract foo_e();
 }

```

**Prettier Similarity**: 73.33%


### typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractOverloads.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractOverrideWithAbstract.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractProperties.ts
```diff
 abstract class A {
   abstract x: number;
   public abstract y: number;
   protected abstract z: number;
-  private abstract w: number;
+  private abstract w : number;
 
   abstract m: () => void;
 
   abstract foo_x(): number;
   public abstract foo_y(): number;
   protected abstract foo_z(): number;
-  private abstract foo_w(): number;
+  private abstract foo_w() : number;
 }

```

**Prettier Similarity**: 84.62%


### typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractSingleLineDecl.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractSuperCalls.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractUsingAbstractMethod1.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractUsingAbstractMethods2.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractWithInterface.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/classes/classDeclarations/classHeritageSpecification/classAppearsToHaveMembersOfObject.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/classes/classDeclarations/classHeritageSpecification/classExtendingClass.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/classes/classDeclarations/classHeritageSpecification/classExtendsItselfIndirectly.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/classes/classDeclarations/classHeritageSpecification/classIsSubtypeOfBaseType.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/classes/classDeclarations/classInsideBlock.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/classes/classExpression.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/classes/constructorDeclarations/constructorParameters/constructorDefaultValuesReferencingThis.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/classes/constructorDeclarations/constructorParameters/constructorImplementationWithDefaultValues.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/classes/constructorDeclarations/constructorParameters/constructorImplementationWithDefaultValues2.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/classes/constructorDeclarations/constructorParameters/constructorOverloadsWithDefaultValues.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/classes/constructorDeclarations/constructorParameters/constructorOverloadsWithOptionalParameters.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/classes/constructorDeclarations/constructorParameters/constructorParameterProperties.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/classes/constructorDeclarations/constructorParameters/constructorParameterProperties2.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/classes/constructorDeclarations/constructorParameters/declarationEmitReadonly.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/classes/constructorDeclarations/constructorParameters/readonlyConstructorAssignment.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/classes/constructorDeclarations/constructorParameters/readonlyInConstructorParameters.ts
```diff
 class C {
   constructor(readonly x: number) {}
 }
 new C(1).x = 2;
 
 class E {
-  constructor(public readonly x: number) {}
+  constructor(readonly public x: number) {}
 }
 
 class F {
   constructor(private readonly x: number) {}
 }
 new F(1).x;

```

**Prettier Similarity**: 92.31%


### typescript/conformance/classes/mixinAccessModifiers.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/classes/mixinClassesAnnotated.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/classes/mixinClassesAnonymous.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/classes/mixinClassesMembers.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/classes/nestedClassDeclaration.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/comments/comments.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/declarationEmit/typePredicates/declarationEmitThisPredicatesWithPrivateName01.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/es6/Symbols/symbolProperty15.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/es6/templates/templateStringWithEmbeddedTypeAssertionOnAdditionES6.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/expressions/asOperator/asOperatorContextualType.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/expressions/functionCalls/callWithSpreadES6.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/interfaces/interfaceDeclarations/interfaceWithMultipleBaseTypes2.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/internalModules/importDeclarations/circularImportAlias.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/internalModules/importDeclarations/exportImportAlias.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/internalModules/importDeclarations/exportInterface.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/internalModules/importDeclarations/importAliasIdentifiers.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/internalModules/importDeclarations/invalidImportAliasIdentifiers.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/internalModules/importDeclarations/shadowedInternalModule.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/parser/ecmascript5/Statements/parserES5ForOfStatement2.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/parser/ecmascript5/Statements/parserForInStatement2.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/ambient/ambientDeclarations.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/any/anyAsConstructor.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/any/anyAsFunctionCall.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/any/anyAsGenericFunctionCall.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/any/anyPropertyAccess.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/constKeyword/constKeyword.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/constructorType/cunstructorType.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/enumDeclaration/enumDeclaration.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/firstTypeNode/firstTypeNode.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/functions/TSFunctionTypeNoUnnecessaryParentheses.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/functions/functionImplementationErrors.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/functions/functionImplementations.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/functions/functionOverloadCompatibilityWithVoid01.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/functions/functionOverloadCompatibilityWithVoid02.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/functions/functionOverloadCompatibilityWithVoid03.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/functions/functionOverloadErrorsSyntax.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/functions/functionTypeTypeParameters.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/functions/parameterInitializersForwardReferencing.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/importEqualsDeclaration/importEqualsDeclaration.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/indexedAccesType/indexedAccesType.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/interfaceDeclaration/interfaceDeclaration.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/intersectionType/intersectionType.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/lastTypeNode/lastTypeNode.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/mappedType/mappedType.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/methodSignature/methodSignature.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/moduleDeclaration/kind-detection.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/moduleDeclaration/moduleDeclaration.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/namespaceExportDeclaration/exportAsNamespace.d.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/never/never.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/nonNullExpression/nonNullExpression.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/parameterProperty/parameterProperty.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/symbol/symbol.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/thisType/thisType.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/tuple/contextualTypeWithTuple.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/tuple/emptyTuples/emptyTuplesTypeAssertion02.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/tuple/indexerWithTuple.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/tuple/tupleElementTypes1.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/tuple/tupleElementTypes2.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/tuple/tupleElementTypes3.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/tuple/tupleElementTypes4.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/tuple/typeInferenceWithTupleType.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/tuple/wideningTuples1.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/tuple/wideningTuples2.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/tuple/wideningTuples3.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/tuple/wideningTuples4.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/tuple/wideningTuples5.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/tuple/wideningTuples6.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/tuple/wideningTuples7.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/typeOperator/typeOperator.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/typeParameter/typeParameter.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/typeParameters/typeParameterLists/innerTypeParameterShadowingOuterOne.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/typeParameters/typeParameterLists/innerTypeParameterShadowingOuterOne2.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/typeParameters/typeParameterLists/staticMembersUsingClassTypeParameter.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/typeParameters/typeParameterLists/typeParametersAvailableInNestedScope2.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/typeReference/typeReference.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/undefined/undefined.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/union/unionTypeCallSignatures.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/union/unionTypeCallSignatures3.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/union/unionTypeCallSignatures4.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/union/unionTypeConstructSignatures.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/union/unionTypeEquivalence.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/union/unionTypeFromArrayLiteral.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/union/unionTypeIndexSignature.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/union/unionTypePropertyAccessibility.ts

**Prettier Similarity**: 100.00%


### typescript/conformance/types/variableDeclarator/variableDeclarator.ts

**Prettier Similarity**: 100.00%


### typescript/const/initializer-ambient-context.ts

**Prettier Similarity**: 100.00%


### typescript/cursor/array-pattern.ts

**Prettier Similarity**: 100.00%


### typescript/cursor/arrow-function-type.ts

**Prettier Similarity**: 100.00%


### typescript/cursor/class-property.ts

**Prettier Similarity**: 100.00%


### typescript/cursor/function-return-type.ts

**Prettier Similarity**: 100.00%


### typescript/cursor/identifier-1.ts

**Prettier Similarity**: 100.00%


### typescript/cursor/identifier-2.ts

**Prettier Similarity**: 100.00%


### typescript/cursor/identifier-3.ts

**Prettier Similarity**: 100.00%


### typescript/cursor/method-signature.ts

**Prettier Similarity**: 100.00%


### typescript/cursor/property-signature.ts

**Prettier Similarity**: 100.00%


### typescript/cursor/rest.ts

**Prettier Similarity**: 100.00%


### typescript/custom/abstract/abstractNewlineHandling.ts

**Prettier Similarity**: 100.00%


### typescript/custom/abstract/abstractProperties.ts
```diff
 abstract class Foo {
-  private abstract a: 1;
+  abstract private a: 1;
   private abstract b: 2;
   static abstract c: 3;
-  private abstract ["g"];
-  private abstract ["h"];
-  static abstract ["i"];
+  abstract private ['g'];
+  private abstract ['h'];
+  static abstract ['i'];
 }

```

**Prettier Similarity**: 50.00%


### typescript/custom/call/callSignature.ts

**Prettier Similarity**: 100.00%


### typescript/custom/computedProperties/string.ts

**Prettier Similarity**: 100.00%


### typescript/custom/computedProperties/symbol.ts

**Prettier Similarity**: 100.00%


### typescript/custom/declare/declareModifier.d.ts

**Prettier Similarity**: 100.00%


### typescript/custom/modifiers/minustoken.ts

**Prettier Similarity**: 100.00%


### typescript/custom/modifiers/question.ts

**Prettier Similarity**: 100.00%


### typescript/custom/modifiers/readonly.ts

**Prettier Similarity**: 100.00%


### typescript/custom/module/global.ts

**Prettier Similarity**: 100.00%


### typescript/custom/module/moduleNamespace.ts

**Prettier Similarity**: 100.00%


### typescript/custom/module/nestedNamespace.ts

**Prettier Similarity**: 100.00%


### typescript/custom/new/newKeyword.ts

**Prettier Similarity**: 100.00%


### typescript/custom/stability/moduleBlock.ts

**Prettier Similarity**: 100.00%


### typescript/custom/typeParameters/callAndConstructSignatureLong.ts

**Prettier Similarity**: 100.00%


### typescript/custom/typeParameters/functionTypeLong.ts

**Prettier Similarity**: 100.00%


### typescript/custom/typeParameters/interfaceParamsLong.ts

**Prettier Similarity**: 100.00%


### typescript/custom/typeParameters/typeParametersLong.ts

**Prettier Similarity**: 100.00%


### typescript/custom/typeParameters/variables.ts

**Prettier Similarity**: 100.00%


### typescript/declare/declare-get-set-field.ts

**Prettier Similarity**: 100.00%


### typescript/declare/declare_class_fields.ts

**Prettier Similarity**: 100.00%


### typescript/declare/declare_enum.ts

**Prettier Similarity**: 100.00%


### typescript/declare/declare_function.ts

**Prettier Similarity**: 100.00%


### typescript/declare/declare_interface.ts

**Prettier Similarity**: 100.00%


### typescript/declare/declare_module.ts

**Prettier Similarity**: 100.00%


### typescript/declare/declare_namespace.ts

**Prettier Similarity**: 100.00%


### typescript/declare/declare_var.ts

**Prettier Similarity**: 100.00%


### typescript/declare/object-type-in-declare-function.ts
```diff
-declare function foo(this: {
-  a: boolean;
-  b: string;
-  c: number;
-}): Promise<Array<foo>>;
+declare function foo(this: { a: boolean; b: string; c: number }): Promise<
+  Array<foo>
+>;
 
 declare function bazFlip({
   a: boolean,
   b: string,
   c: number,
 }): Promise<Array<foo>>;
 
 declare function bar(
   ...{ a: boolean, b: string, c: number }
 ): Promise<Array<foo>>;
 
 declare function bar(
   ...x: { a: boolean; b: string; c: number }
 ): Promise<Array<foo>>;

```

**Prettier Similarity**: 73.68%


### typescript/declare/trailing-comma/function-rest-trailing-comma.ts
```diff
-declare function foo(...args: any[]);
-declare function foo(
-  ...long_long_long_long_long_long_long_long_long_long_long_long_long_long_long_args: any[]
-);
+declare function foo(...args: any[], )
+declare function foo(...long_long_long_long_long_long_long_long_long_long_long_long_long_long_long_args: any[], )

```

**Prettier Similarity**: 0.00%


### typescript/decorator-auto-accessors/decorator-auto-accessors-new-line.ts

**Prettier Similarity**: 100.00%


### typescript/decorator-auto-accessors/decorator-auto-accessors-type-annotations.ts

**Prettier Similarity**: 100.00%


### typescript/decorator-auto-accessors/no-semi/decorator-auto-accessor-like-property-name.ts

**Prettier Similarity**: 100.00%


### typescript/decorators-ts/accessor-decorator.ts

**Prettier Similarity**: 100.00%


### typescript/decorators-ts/angular.ts
```diff
 @Component({
   selector: "toh-hero-button",
-  template: `<button>{{ label }}</button>`,
+  template: `<button>{{label}}</button>`,
 })
 export class HeroButtonComponent {
   @Output() change = new EventEmitter<any>();
   @Input() label: string;
 }

```

**Prettier Similarity**: 87.50%


### typescript/decorators-ts/class-decorator.ts

**Prettier Similarity**: 100.00%


### typescript/decorators-ts/method-decorator.ts

**Prettier Similarity**: 100.00%


### typescript/decorators-ts/mobx.ts

**Prettier Similarity**: 100.00%


### typescript/decorators-ts/multiple.ts

**Prettier Similarity**: 100.00%


### typescript/decorators-ts/parameter-decorator.ts

**Prettier Similarity**: 100.00%


### typescript/decorators-ts/property-decorator.ts

**Prettier Similarity**: 100.00%


### typescript/decorators-ts/typeorm.ts
```diff
 @Entity()
 export class Board {
   @PrimaryGeneratedColumn()
   id: number;
 
   @Column()
   slug: string;
 
   @Column()
   name: string;
 
   @Column()
   theme: string;
 
   @Column()
   description: string;
 
-  @OneToMany((type) => Topic, (topic) => topic.board)
+  @OneToMany(
+    (type) => Topic,
+    (topic) => topic.board,
+  )
   topics: Topic[];
 }

```

**Prettier Similarity**: 82.61%


### typescript/decorators/accessor.ts

**Prettier Similarity**: 100.00%


### typescript/decorators/argument-list-preserve-line.ts

**Prettier Similarity**: 100.00%


### typescript/decorators/comments.ts

**Prettier Similarity**: 100.00%


### typescript/decorators/decorator-type-assertion.ts

**Prettier Similarity**: 100.00%


### typescript/decorators/decorators-comments.ts
```diff
 class Foo1 {
   @foo
   // comment
   async method() {}
 }
 
 class Foo2 {
   @foo
   // comment
   private method() {}
 }
 
 class Foo3 {
   @foo
   // comment
   *method() {}
 }
 
 class Foo4 {
   @foo
   // comment
   async *method() {}
 }
 
 class Something {
   @foo()
   // comment
   readonly property: Array<string>;
 }
 
 class Something2 {
   @foo()
-  // comment
-  abstract property: Array<string>;
+    // comment
+    abstract property: Array<string>
 }

```

**Prettier Similarity**: 94.29%


### typescript/decorators/decorators.ts

**Prettier Similarity**: 100.00%


### typescript/decorators/inline-decorators.ts

**Prettier Similarity**: 100.00%


### typescript/decorators/legacy.ts

**Prettier Similarity**: 100.00%


### typescript/decorators/mobx.ts

**Prettier Similarity**: 100.00%


### typescript/definite/asi.ts

**Prettier Similarity**: 100.00%


### typescript/definite/definite.ts

**Prettier Similarity**: 100.00%


### typescript/definite/without-annotation.ts
```diff
 class Foo {
-  a!;
-  #b!;
-  static c!;
-  [d]! = 1;
-  "e"!;
+  a!
+  #b!
+  static c!
+  [d]! = 1
+  'e'!
 }
 
-let a! = x;
-const b! = x;
-var c! /* */ = x;
-export const d! = x;
+let a! = x
+const b! = x
+var c/* */! = x
+export const d! = x

```

**Prettier Similarity**: 25.00%


### typescript/destructuring/destructuring.ts

**Prettier Similarity**: 100.00%


### typescript/end-of-line/multiline.ts

**Prettier Similarity**: 100.00%


### typescript/enum/computed-members.ts

**Prettier Similarity**: 100.00%


### typescript/enum/enum.ts

**Prettier Similarity**: 100.00%


### typescript/enum/multiline.ts

**Prettier Similarity**: 100.00%


### typescript/explicit-resource-management/await-using-with-type-declaration.ts

**Prettier Similarity**: 100.00%


### typescript/explicit-resource-management/using-with-type-declaration.ts

**Prettier Similarity**: 100.00%


### typescript/export-default/function_as.ts

**Prettier Similarity**: 100.00%


### typescript/export/comment.ts

**Prettier Similarity**: 100.00%


### typescript/export/default.ts

**Prettier Similarity**: 100.00%


### typescript/export/export-as-ns.ts

**Prettier Similarity**: 100.00%


### typescript/export/export-class.ts

**Prettier Similarity**: 100.00%


### typescript/export/export-type-star-from.ts

**Prettier Similarity**: 100.00%


### typescript/export/export.ts

**Prettier Similarity**: 100.00%


### typescript/function-type/consistent.ts

**Prettier Similarity**: 100.00%


### typescript/function-type/single-parameter.ts

**Prettier Similarity**: 100.00%


### typescript/function-type/type-annotation.ts

**Prettier Similarity**: 100.00%


### typescript/function/single_expand.ts

**Prettier Similarity**: 100.00%


### typescript/functional-composition/pipe-function-calls-with-comments.ts

**Prettier Similarity**: 100.00%


### typescript/functional-composition/pipe-function-calls.ts

**Prettier Similarity**: 100.00%


### typescript/generic/arrow-return-type.ts

**Prettier Similarity**: 100.00%


### typescript/generic/issue-6899.ts

**Prettier Similarity**: 100.00%


### typescript/generic/object-method.ts

**Prettier Similarity**: 100.00%


### typescript/generic/ungrouped-parameters.ts

**Prettier Similarity**: 100.00%


### typescript/import-export/empty-import.ts
```diff
 import type {} from "@types/googlemaps";
 import "a";
 import /* comment */ "a";
 import // comment
 "a";
+import type {/* comment */} from "a";
+import /* comment */ type {} from "a";
+import type {} /* comment */ from "a";
 import type {} from /* comment */ "a";
-import type {} from /* comment */ "a";
-import type {} from /* comment */ "a";
-import type {} from /* comment */ "a";
-import type {} from /* comment */ /* comment */ /* comment */ /* comment */ "a";
+import /* comment */ type {/* comment */} /* comment */ from /* comment */ "a";
+import type {
+  // comment
+} from "a";
+import // comment
+type {} from "a";
 import type {} from // comment
 "a";
 import type {} from // comment
 "a";
-import type {} from // comment
-"a";
-import type {} from // comment
-"a";
-import type {} from // comment
-// comment
-// comment
+import type // comment
+{
+  // comment
+} from // comment
 // comment
 "a";
 
 import // {} from
 "a";
 import type {} from // comment ends with from
 "a";
 import type {} from /* comment ends with from */ "a";
 import type {} from // comment not ends with from ___
 "a";
 import type {} from /* comment not ends with from ___ */ "a";
 
 import // comment ends with from
 "a";
 import /* comment ends with from */ "a";
 import // comment not ends with from ___
 "a";
 import /* comment not ends with from ___ */ "a";

```

**Prettier Similarity**: 68.29%


### typescript/import-export/type-modifier.ts
```diff
 export type { SomeThing };
 export type { A as B };
 export type { B as C } from "./a";
 export type { foo } from "bar";
 export type { foo };
 
 // this should be treated as a normal import statement
 import type from "./foo";
 
 import type { SomeThing } from "./some-module.js";
 import type { foo, bar } from "baz";
 import type { foo as bar } from "baz";
 import type * as foo from "./bar";
 import type foo from "bar";
-import type foo, { bar } from "bar";
+import type foo, { bar } from 'bar';

```

**Prettier Similarity**: 93.33%


### typescript/import-require/import_require.ts

**Prettier Similarity**: 100.00%


### typescript/import-require/type-imports.ts

**Prettier Similarity**: 100.00%


### typescript/import-type/import-type.ts

**Prettier Similarity**: 100.00%


### typescript/index-signature/index-signature.ts

**Prettier Similarity**: 100.00%


### typescript/index-signature/static.ts

**Prettier Similarity**: 100.00%


### typescript/infer-extends/basic.ts

**Prettier Similarity**: 100.00%


### typescript/instantiation-expression/basic.ts

**Prettier Similarity**: 100.00%


### typescript/instantiation-expression/binary-expr.ts

**Prettier Similarity**: 100.00%


### typescript/instantiation-expression/inferface-asi.ts

**Prettier Similarity**: 100.00%


### typescript/instantiation-expression/logical-expr.ts

**Prettier Similarity**: 100.00%


### typescript/instantiation-expression/new.ts

**Prettier Similarity**: 100.00%


### typescript/instantiation-expression/property-access.ts

**Prettier Similarity**: 100.00%


### typescript/instantiation-expression/typeof.ts

**Prettier Similarity**: 100.00%


### typescript/interface/comments-generic.ts

**Prettier Similarity**: 100.00%


### typescript/interface/comments.ts

**Prettier Similarity**: 100.00%


### typescript/interface/generic.ts

**Prettier Similarity**: 100.00%


### typescript/interface/ignore.ts

**Prettier Similarity**: 100.00%


### typescript/interface/long-extends.ts

**Prettier Similarity**: 100.00%


### typescript/interface/long-type-parameters/long-type-parameters.ts

**Prettier Similarity**: 100.00%


### typescript/interface/pattern-parameters.ts

**Prettier Similarity**: 100.00%


### typescript/interface/separator.ts

**Prettier Similarity**: 100.00%


### typescript/interface2/break/break.ts
```diff
-export interface Environment1
-  extends GenericEnvironment<SomeType, AnotherType, YetAnotherType> {
+export interface Environment1 extends GenericEnvironment<
+  SomeType,
+  AnotherType,
+  YetAnotherType,
+> {
   m(): void;
 }
 export class Environment2 extends GenericEnvironment<
   SomeType,
   AnotherType,
   YetAnotherType,
   DifferentType1,
   DifferentType2,
   DifferentType3,
-  DifferentType4
+  DifferentType4,
 > {
-  m() {}
+  m() {};
 }
 
 // Declare Interface Break
 declare interface ExtendsOne extends ASingleInterface {
   x: string;
 }
 
 declare interface ExtendsLarge
   extends ASingleInterfaceWithAReallyReallyReallyReallyLongName {
   x: string;
 }
 
 declare interface ExtendsMany
   extends Interface1,
     Interface2,
     Interface3,
     Interface4,
     Interface5,
     Interface6,
     Interface7 {
   x: string;
 }
 
 // Interface declaration break
 interface ExtendsOne extends ASingleInterface {
   x: string;
 }
 
 interface ExtendsLarge
   extends ASingleInterfaceWithAReallyReallyReallyReallyLongName {
   x: string;
 }
 
 interface ExtendsMany
   extends Interface1,
     Interface2,
     Interface3,
     Interface4,
     Interface5,
     Interface6,
     Interface7 {
   s: string;
 }
 
 // Generic Types
 interface ExtendsOne extends ASingleInterface<string> {
   x: string;
 }
 
 interface ExtendsLarge
   extends ASingleInterfaceWithAReallyReallyReallyReallyLongName<string> {
   x: string;
 }
 
 interface ExtendsMany
   extends ASingleGenericInterface<
     Interface1,
     Interface2,
     Interface3,
     Interface4,
     Interface5,
     Interface6,
     Interface7
   > {
   x: string;
 }
 
 interface ExtendsManyWithGenerics
   extends InterfaceOne,
     InterfaceTwo,
     ASingleGenericInterface<
       Interface1,
       Interface2,
       Interface3,
       Interface4,
       Interface5,
       Interface6,
       Interface7
     >,
     InterfaceThree {
   x: string;
 }
 
 export interface ExtendsLongOneWithGenerics
   extends Bar<
     SomeLongTypeSomeLongTypeSomeLongTypeSomeLongType,
     ToBreakLineToBreakLineToBreakLine
   > {}

```

**Prettier Similarity**: 93.33%


### typescript/interface2/comments-declare.ts

**Prettier Similarity**: 100.00%


### typescript/interface2/comments.ts

**Prettier Similarity**: 100.00%


### typescript/interface2/module.ts

**Prettier Similarity**: 100.00%


### typescript/intersection/consistent-with-flow/comment.ts

**Prettier Similarity**: 100.00%


### typescript/intersection/consistent-with-flow/intersection-parens.ts
```diff
 type A = (number | string) & boolean;
 type B = (number | string) & boolean;
 type C = (number | string) & boolean;
 type D = (number | string) & boolean;
 
 let b1: C;
 let b2: C;
 let b3: C;
 let b4: C;
 let b5: C;
 let b6: /*1*/ C;
 let b7: /*1*/ C;
 let b8: /*1*/ C;
 let b9: /*1*/ C;
 let b10: /*1*/ /*2*/ C;
 let b11: /*1*/ /*2*/ C;
 
 let bb1: /*1*/ /*2*/ C & D;
 let bb2: /*1*/ /*2*/ C & /*3*/ D;
 let bb3: /*1*/ /*2*/ C & /*3*/ D /*5*/;
 
 type B2 = C;
 type B3 = C;
 type B4 = C;
 type B5 = C;
-type B6 /*1*/ = C;
-type B7 /*1*/ = C;
-type B8 /*1*/ = C;
-type B9 /*1*/ = C;
-type B10 /*1*/ = /*2*/ C;
-type B11 /*1*/ = /*2*/ C;
-type B12 /*1*/ = C;
+type B6 = /*1*/ C;
+type B7 = /*1*/ C;
+type B8 = /*1*/ C;
+type B9 = /*1*/ C;
+type B10 = /*1*/ /*2*/ C;
+type B11 = /*1*/ /*2*/ C;
+type B12 = /*1*/ C;
 
 type Bb1 = /*1*/ /*2*/ C & D;
 type Bb2 = /*1*/ /*2*/ C & /*3*/ D;
 type Bb3 = /*1*/ /*2*/ C & /*3*/ D /*4*/;
 
-type D1 /*1*/ = a & b;
-type D2 /*1*/ = a & b;
-type D3 /*1*/ = a & b;
-type D4 /*1*/ = a & b;
-type D5 /*1*/ = a & b;
-type D6 /*0*/ /*1*/ = a & b;
+type D1 = /*1*/ a & b;
+type D2 = /*1*/ a & b;
+type D3 = /*1*/ a & b;
+type D4 = /*1*/ a & b;
+type D5 = /*1*/ a & b;
+type D6 /*0*/ = /*1*/ a & b;

```

**Prettier Similarity**: 69.77%


### typescript/intersection/type-arguments.ts

**Prettier Similarity**: 100.00%


### typescript/intrinsic/intrinsic.ts

**Prettier Similarity**: 100.00%


### typescript/key-remapping-in-mapped-types/key-remapping.ts

**Prettier Similarity**: 100.00%


### typescript/keyof/keyof.ts

**Prettier Similarity**: 100.00%


### typescript/keyword-types/conditional-types.ts

**Prettier Similarity**: 100.00%


### typescript/keyword-types/keyword-types-with-parens-comments.ts

**Prettier Similarity**: 100.00%


### typescript/keywords/keywords-2.ts

**Prettier Similarity**: 100.00%


### typescript/keywords/keywords.ts

**Prettier Similarity**: 100.00%


### typescript/keywords/module.ts

**Prettier Similarity**: 100.00%


### typescript/last-argument-expansion/break.ts

**Prettier Similarity**: 100.00%


### typescript/last-argument-expansion/decorated-function.tsx
```diff
-const Counter = decorator("my-counter")((props: {
-  initialCount?: number;
-  label?: string;
-}) => {
-  const p = useDefault(props, {
-    initialCount: 0,
-    label: "Counter",
-  });
+const Counter = decorator("my-counter")(
+  (props: { initialCount?: number; label?: string }) => {
+    const p = useDefault(props, {
+      initialCount: 0,
+      label: "Counter",
+    });
 
-  const [s, set] = useState({ count: p.initialCount });
-  const onClick = () => set("count", (it) => it + 1);
+    const [s, set] = useState({ count: p.initialCount });
+    const onClick = () => set("count", (it) => it + 1);
 
-  return () => (
-    <button onclick={onClick}>
-      {p.label}: {s.count}
-    </button>
-  );
-});
+    return () => (
+      <button onclick={onClick}>
+        {p.label}: {s.count}
+      </button>
+    );
+  },
+);
 
-const Counter2 = decorators.decorator("my-counter")((props: {
-  initialCount?: number;
-  label?: string;
-}) => {
-  return () => (
-    <button onclick={onClick}>
-      {p.label}: {s.count}
-    </button>
-  );
-});
+const Counter2 = decorators.decorator("my-counter")(
+  (props: { initialCount?: number; label?: string }) => {
+    return () => (
+      <button onclick={onClick}>
+        {p.label}: {s.count}
+      </button>
+    );
+  },
+);
 
-export default decorators.decorator("my-counter")((props: {
-  initialCount?: number;
-  label?: string;
-}) => {
-  return foo;
-});
+export default decorators.decorator("my-counter")(
+  (props: { initialCount?: number; label?: string }) => {
+    return foo;
+  },
+);
 
-export = decorators.decorator("my-counter")((props: {
-  initialCount?: number;
-  label?: string;
-}) => {
-  return foo;
-});
+export = decorators.decorator("my-counter")(
+  (props: { initialCount?: number; label?: string }) => {
+    return foo;
+  },
+);
 
-module.exports = decorators.decorator("my-counter")((props: {
-  initialCount?: number;
-  label?: string;
-}) => {
-  return foo;
-});
+module.exports = decorators.decorator("my-counter")(
+  (props: { initialCount?: number; label?: string }) => {
+    return foo;
+  },
+);
 
 const Counter = decorator("foo")(
   decorator("bar")(
     (props: {
       loremFoo1: Array<Promise<any>>;
       ipsumBarr2: Promise<number>;
     }) => {
       return <div />;
     },
   ),
 );

```

**Prettier Similarity**: 27.87%


### typescript/last-argument-expansion/edge_case.ts

**Prettier Similarity**: 100.00%


### typescript/last-argument-expansion/forward-ref.tsx

**Prettier Similarity**: 100.00%


### typescript/literal/multiline.ts

**Prettier Similarity**: 100.00%


### typescript/mapped-type/break-mode/break-mode.ts

**Prettier Similarity**: 100.00%


### typescript/mapped-type/intersection.ts

**Prettier Similarity**: 100.00%


### typescript/mapped-type/issue-11098.ts

**Prettier Similarity**: 100.00%


### typescript/mapped-type/mapped-type.ts

**Prettier Similarity**: 100.00%


### typescript/method-chain/comment.ts

**Prettier Similarity**: 100.00%


### typescript/method/issue-10352-consistency.ts

**Prettier Similarity**: 100.00%


### typescript/method/method-signature-with-wrapped-return-type.ts

**Prettier Similarity**: 100.00%


### typescript/method/method-signature.ts

**Prettier Similarity**: 100.00%


### typescript/method/semi.ts

**Prettier Similarity**: 100.00%


### typescript/method/type_literal_optional_method.ts

**Prettier Similarity**: 100.00%


### typescript/module/empty.ts

**Prettier Similarity**: 100.00%


### typescript/module/global.ts

**Prettier Similarity**: 100.00%


### typescript/module/keyword.ts

**Prettier Similarity**: 100.00%


### typescript/module/module_nested.ts

**Prettier Similarity**: 100.00%


### typescript/module/namespace_function.ts

**Prettier Similarity**: 100.00%


### typescript/module/namespace_nested.ts

**Prettier Similarity**: 100.00%


### typescript/multiparser-css/issue-6259.ts
```diff
 const yesFrame = (
   ...args: Interpolation<ThemedStyledProps<{}, Theme>>[]
 ) => css`
-  ${ChatRoot}[data-frame="yes"] & {
-    ${css({}, ...args)}
-  }
+    ${ChatRoot}[data-frame="yes"] & {
+        ${css({}, ...args)}
+    }
 `;

```

**Prettier Similarity**: 57.14%


### typescript/namespace/invalid-await.ts

**Prettier Similarity**: 100.00%


### typescript/never/type-argument.src.ts

**Prettier Similarity**: 100.00%


### typescript/new/new-signature.ts

**Prettier Similarity**: 100.00%


### typescript/no-semi/no-semi.ts

**Prettier Similarity**: 100.00%


### typescript/no-semi/non-null.ts

**Prettier Similarity**: 100.00%


### typescript/non-null/braces.ts

**Prettier Similarity**: 100.00%


### typescript/non-null/member-chain.ts

**Prettier Similarity**: 100.00%


### typescript/non-null/optional-chain.ts
```diff
 a?.b!.c;
 a?.b!.c.d;
 a?.b.c!.d;
 a!.b?.c;
 a?.b!?.c;
 a?.b!.c?.c;
-(a?.b)!.c;
-(a?.b)!.c;
+a?.b!.c;
+a?.b!.c;
 
 a?.().b!.c;
 a?.().b!.c.d;
 a?.().b.c!.d;
 a?.().b!?.c;
 a?.().b!.c?.c;
-(a?.().b)!.c;
-(a?.().b)!.c;
+a?.().b!.c;
+a?.().b!.c;
 
-(a?.b)![c?.d!];
+a?.b![c?.d!];

```

**Prettier Similarity**: 72.22%


### typescript/non-null/parens.ts

**Prettier Similarity**: 100.00%


### typescript/nosemi/index-signature.ts

**Prettier Similarity**: 100.00%


### typescript/nosemi/interface.ts

**Prettier Similarity**: 100.00%


### typescript/nosemi/type.ts

**Prettier Similarity**: 100.00%


### typescript/optional-call/type-parameters.ts

**Prettier Similarity**: 100.00%


### typescript/optional-method/optional-method.ts

**Prettier Similarity**: 100.00%


### typescript/optional-type/complex.ts

**Prettier Similarity**: 100.00%


### typescript/optional-type/simple.ts

**Prettier Similarity**: 100.00%


### typescript/optional-variance/basic.ts

**Prettier Similarity**: 100.00%


### typescript/optional-variance/with-jsx.tsx

**Prettier Similarity**: 100.00%


### typescript/override-modifiers/override-modifier.ts

**Prettier Similarity**: 100.00%


### typescript/override-modifiers/parameter-property.ts

**Prettier Similarity**: 100.00%


### typescript/predicate-types/predicate-types.ts

**Prettier Similarity**: 100.00%


### typescript/prettier-ignore/issue-14238.ts
```diff
-export const foo = // prettier-ignore
-  (bar as Baz).qux;
+export const foo =
+  (
+  // prettier-ignore
+  bar as Baz
+).qux;

```

**Prettier Similarity**: 0.00%


### typescript/prettier-ignore/mapped-types.ts
```diff
 type a = {
     // prettier-ignore
     [A in B]: C  |  D
   };
 
 type a = {
     [
       // prettier-ignore
       A in B
     ]: C  |  D
   };
 
 type a = {
-  [A in B]: C | D; // prettier-ignore
+  [A in B]: C | D;
 };
 
 type a = {
-  A in B: C | D; // prettier-ignore
+  [A in B]: // prettier-ignore
+  C | D;
 };
 
 type a = {
     [
       /* prettier-ignore */
       A in B
     ]: C  |  D
   };
 
 type a = {
-  [A /* prettier-ignore */ in B]: C | D;
+  [A in B]: C | D;
 };
 
 type a = {
-  A in B /* prettier-ignore */: C | D;
+  [A in B]: /* prettier-ignore */
+  C | D;
 };
 
 type a = {
     /* prettier-ignore */ [A in B]: C  |  D
   };
 
 type a = {
-    [/* prettier-ignore */ A in B ]: C  |  D
-  };
+  [/* prettier-ignore */ A in B]: C | D;
+};
 
 type a = {
   [A in /* prettier-ignore */ B]: C | D;
 };
 
 type a = {
-  [A in B /* prettier-ignore */]: C | D;
+  [A in B]: /* prettier-ignore */ C | D;
 };
 
 type a = {
     /* prettier-ignore */
     [A in B]: C  |  D
   };

```

**Prettier Similarity**: 84.21%


### typescript/prettier-ignore/prettier-ignore-nested-unions.ts
```diff
 export type a =
   // foo
   | (foo1 & foo2)
   // bar
   | (bar1 & bar2)
   // prettier-ignore
-  | (| aaaaaaaaaaaaa&1
-    // b
-    | bbbbbbbbbbbbb&2)
+  | (
+      | (aaaaaaaaaaaaa & 1)
+      // b
+      | (bbbbbbbbbbbbb & 2)
+    )
   // baz
   | (baz1 & baz2);
 
 export type b =
   // foo
   | (foo1 & foo2)
   // bar
   | (bar1 & bar2)
   // prettier-ignore
-  | (| aaaaaaaaaaaaa&1
-    // b
-    | bbbbbbbbbbbbb&2)
+  | (
+      | (aaaaaaaaaaaaa & 1)
+      // b
+      | (bbbbbbbbbbbbb & 2)
+    )
   // baz
   | (baz1 & baz2);

```

**Prettier Similarity**: 62.96%


### typescript/prettier-ignore/prettier-ignore-parenthesized-type.ts
```diff
 type Foo =
   // prettier-ignore
-  aa;
+  (
+    aa
+  );

```

**Prettier Similarity**: 40.00%


### typescript/private-fields-in-in/basic.ts

**Prettier Similarity**: 100.00%


### typescript/quote-props/types.ts

**Prettier Similarity**: 100.00%


### typescript/range/export-assignment.ts

**Prettier Similarity**: 100.00%


### typescript/range/issue-4926.ts

**Prettier Similarity**: 100.00%


### typescript/range/issue-7148.ts

**Prettier Similarity**: 100.00%


### typescript/readonly/array.ts

**Prettier Similarity**: 100.00%


### typescript/readonly/readonly.ts

**Prettier Similarity**: 100.00%


### typescript/rest-type/complex.ts

**Prettier Similarity**: 100.00%


### typescript/rest-type/infer-type.ts

**Prettier Similarity**: 100.00%


### typescript/rest-type/simple.ts

**Prettier Similarity**: 100.00%


### typescript/rest/rest.ts

**Prettier Similarity**: 100.00%


### typescript/satisfies-operators/argument-expansion.ts

**Prettier Similarity**: 100.00%


### typescript/satisfies-operators/assignment.ts

**Prettier Similarity**: 100.00%


### typescript/satisfies-operators/basic.ts

**Prettier Similarity**: 100.00%


### typescript/satisfies-operators/comments-unstable.ts
```diff
 const t1 = {
   prop1: 1,
   prop2: 2,
   prop3: 3,
-} satisfies Record<string, number>; // Comment
+} satisfies
+  // Comment
+  Record<string, number>;

```

**Prettier Similarity**: 57.14%


### typescript/satisfies-operators/comments.ts

**Prettier Similarity**: 100.00%


### typescript/satisfies-operators/export-default-as.ts

**Prettier Similarity**: 100.00%


### typescript/satisfies-operators/expression-statement.ts

**Prettier Similarity**: 100.00%


### typescript/satisfies-operators/gt-lt.ts

**Prettier Similarity**: 100.00%


### typescript/satisfies-operators/hug-args.ts

**Prettier Similarity**: 100.00%


### typescript/satisfies-operators/lhs.ts

**Prettier Similarity**: 100.00%


### typescript/satisfies-operators/nested-await-and-satisfies.ts

**Prettier Similarity**: 100.00%


### typescript/satisfies-operators/non-null.ts

**Prettier Similarity**: 100.00%


### typescript/satisfies-operators/satisfies.ts

**Prettier Similarity**: 100.00%


### typescript/satisfies-operators/template-literal.ts

**Prettier Similarity**: 100.00%


### typescript/satisfies-operators/ternary.ts

**Prettier Similarity**: 100.00%


### typescript/satisfies-operators/types-comments.ts

**Prettier Similarity**: 100.00%


### typescript/semi/no-semi.ts

**Prettier Similarity**: 100.00%


### typescript/static-blocks/multiple.ts

**Prettier Similarity**: 100.00%


### typescript/static-blocks/nested.ts

**Prettier Similarity**: 100.00%


### typescript/static-blocks/static-blocks.ts

**Prettier Similarity**: 100.00%


### typescript/symbol/symbol.ts

**Prettier Similarity**: 100.00%


### typescript/template-literal-types/template-literal-types.ts

**Prettier Similarity**: 100.00%


### typescript/template-literals/as-expression.ts

**Prettier Similarity**: 100.00%


### typescript/template-literals/expressions.ts

**Prettier Similarity**: 100.00%


### typescript/ternaries/indent.ts

**Prettier Similarity**: 100.00%


### typescript/test-declarations/test_declarations.ts
```diff
-test("does something really long and complicated so I have to write a very long name for the test", <T>(done) => {
+test("does something really long and complicated so I have to write a very long name for the test", <
+  T,
+>(done) => {
   console.log("hello!");
 });

```

**Prettier Similarity**: 40.00%


### typescript/trailing-comma/arrow-functions.tsx

**Prettier Similarity**: 100.00%


### typescript/trailing-comma/trailing.ts
```diff
 export class BaseSingleLevelProfileTargeting<
   T extends ValidSingleLevelProfileNode,
 > {}
 
 enum Enum {
   x = 1,
   y = 2,
 }
 
 const {
   longKeySoThisWillGoOnMultipleLines,
   longKeySoThisWillGoOnMultipleLines2,
   longKeySoThisWillGoOnMultipleLines3,
-  ...rest
+  ...rest,
 } = something;

```

**Prettier Similarity**: 93.33%


### typescript/trailing-comma/type-arguments.ts

**Prettier Similarity**: 100.00%


### typescript/trailing-comma/type-parameters-vs-arguments.ts
```diff
 class FooClass<A, B, C> {
   a: A;
   b: B;
   c: C;
 }
 
 const instance = new FooClass<
-  boolean,
-  number,
-  string // [ts] Trailing comma not allowed.
->();
+	boolean,
+	number,
+	string, // [ts] Trailing comma not allowed.
+	>();

```

**Prettier Similarity**: 63.64%


### typescript/tsx/generic-component.tsx

**Prettier Similarity**: 100.00%


### typescript/tsx/keyword.tsx

**Prettier Similarity**: 100.00%


### typescript/tsx/member-expression.tsx

**Prettier Similarity**: 100.00%


### typescript/tsx/not-react.ts

**Prettier Similarity**: 100.00%


### typescript/tsx/react.tsx

**Prettier Similarity**: 100.00%


### typescript/tsx/this.tsx

**Prettier Similarity**: 100.00%


### typescript/tsx/type-parameters.tsx

**Prettier Similarity**: 100.00%


### typescript/tsx/url.tsx

**Prettier Similarity**: 100.00%


### typescript/tuple/dangling-comments.ts

**Prettier Similarity**: 100.00%


### typescript/tuple/trailing-comma-for-empty-tuples.ts

**Prettier Similarity**: 100.00%


### typescript/tuple/trailing-comma-trailing-rest.ts

**Prettier Similarity**: 100.00%


### typescript/tuple/trailing-comma.ts

**Prettier Similarity**: 100.00%


### typescript/tuple/tuple-labeled.ts

**Prettier Similarity**: 100.00%


### typescript/tuple/tuple-rest-not-last.ts

**Prettier Similarity**: 100.00%


### typescript/tuple/tuple.ts

**Prettier Similarity**: 100.00%


### typescript/type-alias/conditional.ts
```diff
-type FallbackFlags<F extends Flags | undefined> =
-  Equals<NonNullableFlag<F>["flags"], {}> extends true
-    ? Dict<any>
-    : NonNullableFlag<F>["flags"];
+type FallbackFlags<F extends Flags | undefined> = Equals<
+  NonNullableFlag<F>["flags"],
+  {}
+> extends true
+  ? Dict<any>
+  : NonNullableFlag<F>["flags"];
 
-export type UnPromise<Type extends Promise<unknown>> =
-  Type extends Promise<infer Generic> ? Generic : never;
+export type UnPromise<Type extends Promise<unknown>> = Type extends Promise<
+  infer Generic
+>
+  ? Generic
+  : never;
 
-export type Equals<X, Y> =
-  (<T>() => T extends X ? 1 : 2) extends <T>() => T extends Y ? 1 : 2
-    ? true
-    : false;
+export type Equals<X, Y> = (<T>() => T extends X ? 1 : 2) extends <
+  T,
+>() => T extends Y ? 1 : 2
+  ? true
+  : false;
 
-export type _Repeat<A extends any, N extends number, L extends List = []> =
-  __Repeat<N, A, L> extends infer X ? Cast<X, List> : never;
+export type _Repeat<
+  A extends any,
+  N extends number,
+  L extends List = [],
+> = __Repeat<N, A, L> extends infer X ? Cast<X, List> : never;
 
 export type Repeat<
   A extends any,
   N extends number,
   L extends List = [],
 > = N extends unknown ? (L extends unknown ? _Repeat<A, N, L> : never) : never;
 
 export type Intersect<U1 extends any, U2 extends any> = U1 extends unknown
   ? U2 extends unknown
     ? { 1: U1; 0: never }[Equals<U1, U2>]
     : never
   : never;

```

**Prettier Similarity**: 41.67%


### typescript/type-alias/issue-100857.ts

**Prettier Similarity**: 100.00%


### typescript/type-alias/issue-9874.ts

**Prettier Similarity**: 100.00%


### typescript/type-alias/pattern-parameter.ts

**Prettier Similarity**: 100.00%


### typescript/type-arguments-bit-shift-left-like/1.ts

**Prettier Similarity**: 100.00%


### typescript/type-arguments-bit-shift-left-like/2.ts

**Prettier Similarity**: 100.00%


### typescript/type-arguments-bit-shift-left-like/4.ts

**Prettier Similarity**: 100.00%


### typescript/type-arguments-bit-shift-left-like/6.ts

**Prettier Similarity**: 100.00%


### typescript/type-member-get-set/type-member-get-set.ts

**Prettier Similarity**: 100.00%


### typescript/type-only-module-specifiers/basic.ts

**Prettier Similarity**: 100.00%


### typescript/typeof-this/decorators.ts

**Prettier Similarity**: 100.00%


### typescript/typeof-this/typeof-this.ts

**Prettier Similarity**: 100.00%


### typescript/typeof/typeof.ts

**Prettier Similarity**: 100.00%


### typescript/typeparams/class-method.ts

**Prettier Similarity**: 100.00%


### typescript/typeparams/consistent/flow-only.ts

**Prettier Similarity**: 100.00%


### typescript/typeparams/consistent/issue-9501.ts

**Prettier Similarity**: 100.00%


### typescript/typeparams/consistent/simple-types.ts

**Prettier Similarity**: 100.00%


### typescript/typeparams/consistent/template-literal-types.ts

**Prettier Similarity**: 100.00%


### typescript/typeparams/consistent/typescript-only.ts

**Prettier Similarity**: 100.00%


### typescript/typeparams/const.ts
```diff
 function a<const T>() {}
 function b<const T extends U>() {}
 function c<T, const U>() {}
 declare function d<const T>();
 <const T,>() => {};
 <const T extends U>() => {};
 (function <const T>() {});
 (function <const T extends U>() {});
 (function <T, const U>() {});
 
 class A<const T> {}
 class B<const T extends U> {}
 class C<T, const U> {}
 class D<const in T> {}
 class E<const in T> {}
 (class<const T> {});
 (class<const T extends U> {});
 (class<T, const U> {});
 (class<const in T> {});
 (class<const in T> {});
 
 interface I<const T> {}
 interface J<const T extends U> {}
 interface K<T, const U> {}
-interface L<const in T> {}
+interface L<in const T> {}
 interface M<const in T> {}
 
 class _ {
   method<const T>() {}
   method<const T extends U>() {}
   method<T, const U>() {}
 }

```

**Prettier Similarity**: 96.88%


### typescript/typeparams/empty-parameters-with-arrow-function/issue-13817.ts
```diff
-const xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx: xxxxxxxxxxxxxxxxxxxxxx<> = (
-  arg,
-) => null;
+const xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx: xxxxxxxxxxxxxxxxxxxxxx<> =
+  arg => null;
 
 const xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx: xxxxxxxxxxxxxxxxxxxxxx</* comment */> =
-  (arg) => null;
+  arg => null;
 
 const xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx: xxxxxxxxxxxxxxxxxxxxxx<
   // comment
-> = (arg) => null;
+> =
+  arg => null;

```

**Prettier Similarity**: 50.00%


### typescript/typeparams/line-breaking-after-extends-2.ts
```diff
 a = {
   parseFunctionBodyAndFinish<
     T extends
       | N.Function
       | N.TSDeclareMethod
       | N.TSDeclareFunction
       | N.ClassPrivateMethod,
   >() {},
 };
 
 function parseFunctionBodyAndFinish<
   T extends
     | N.Function
     | N.TSDeclareMethod
     | N.TSDeclareFunction
     | N.ClassPrivateMethod
     | Foo
     | Bar
     | Baz,
 >();
 
 function parseFunctionBodyAndFinish<
   T extends // comment
-    N.Function | N.TSDeclareMethod | Baz,
+  N.Function | N.TSDeclareMethod | Baz,
 >();
 
 function makeChainWalker<
   ArgT extends {
     options: ValidatedOptions;
     dirname: string;
     filepath?: string;
   },
 >() {}

```

**Prettier Similarity**: 96.97%


### typescript/typeparams/line-breaking-after-extends.ts

**Prettier Similarity**: 100.00%


### typescript/typeparams/long-function-arg.ts

**Prettier Similarity**: 100.00%


### typescript/typeparams/print-width-120/issue-7542.tsx

**Prettier Similarity**: 100.00%


### typescript/typeparams/tagged-template-expression.ts

**Prettier Similarity**: 100.00%


### typescript/typeparams/trailing-comma/type-paramters.ts

**Prettier Similarity**: 100.00%


### typescript/union/comments.ts

**Prettier Similarity**: 100.00%


### typescript/union/consistent-with-flow/comment.ts

**Prettier Similarity**: 100.00%


### typescript/union/consistent-with-flow/comments.ts

**Prettier Similarity**: 100.00%


### typescript/union/consistent-with-flow/prettier-ignore.ts
```diff
 export type a =
   // foo
   | (foo1 & foo2)
   // bar
   | (bar1 & bar2)
   // prettier-ignore
-  | (qux1&qux2);
+  | qux1&qux2;
 
 export type b =
   // foo
   | (foo1 & foo2)
   // bar
   | (bar1 & bar2)
   // prettier-ignore
-  | (qux1&qux2)
+  | qux1&qux2
   // baz
   | (baz1 & baz2);
 
 export type c =
   // prettier-ignore
-  | (foo1&foo2)
+  | foo1&foo2
   // bar
   | (bar1 & bar2)
   // qux
   | (qux1 & qux2);

```

**Prettier Similarity**: 88.00%


### typescript/union/consistent-with-flow/single-type.ts

**Prettier Similarity**: 100.00%


### typescript/union/consistent-with-flow/within-tuple.ts

**Prettier Similarity**: 100.00%


### typescript/union/inlining.ts

**Prettier Similarity**: 100.00%


### typescript/union/single-type/single-type.ts
```diff
-type A1 /* 2 */ = /* 1 */ /* 3 */ /* 4 */ {
-  key: string;
-};
+type A1 =
+  /* 1 */ /* 2 */
+  /* 3 */ /* 4 */ {
+    key: string;
+  };

```

**Prettier Similarity**: 0.00%


### typescript/union/union-parens.ts
```diff
 export type A =
   | aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
   | bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb;
 
 export type B =
   | aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
   | bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb;
 
 export type C =
   | aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
   | bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb;
 
 export type D =
   | aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
   | bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb;
 
 export type Multi = (string | number)[];
 
 function f(): string | number {}
 
 var x: string | number;
 var y: string | number;
 
 class Foo<T extends string | number> {}
 
 interface Interface {
   i: (X | Y) & Z;
   j: Partial<X | Y>;
 }
 
 type State = {
   sharedProperty: any;
 } & (
   | { discriminant: "FOO"; foo: any }
   | { discriminant: "BAR"; bar: any }
   | { discriminant: "BAZ"; baz: any }
 );
 
 const foo1 = [abc, def, ghi, jkl, mno, pqr, stu, vwx, yz] as (
   | string
   | undefined
 )[];
 
 const foo2: (
   | AAAAAAAAAAAAAAAAAAAAAA
   | BBBBBBBBBBBBBBBBBBBBBB
   | CCCCCCCCCCCCCCCCCCCCCC
   | DDDDDDDDDDDDDDDDDDDDDD
 )[] = [];
 
 const foo3: keyof (
   | AAAAAAAAAAAAAAAAAAAAAA
   | BBBBBBBBBBBBBBBBBBBBBB
   | CCCCCCCCCCCCCCCCCCCCCC
   | DDDDDDDDDDDDDDDDDDDDDD
 ) = bar;
 
 const foo4:
   | foo
   | (
       | AAAAAAAAAAAAAAAAAAAAAA
       | BBBBBBBBBBBBBBBBBBBBBB
       | CCCCCCCCCCCCCCCCCCCCCC
       | DDDDDDDDDDDDDDDDDDDDDD
     ) = bar;
 
 let a1: C;
 let a2: C;
 let a3: C;
 let a4: C;
 let a5: C;
 let a6: /*1*/ C;
 let a7: /*1*/ C;
 let a8: /*1*/ C;
 let a9: /*1*/ C;
 let a10: /*1*/ /*2*/ C;
 let a11: /*1*/ /*2*/ C;
 
 let aa1: /*1*/ /*2*/ C | D;
 let aa2: /*1*/ /*2*/ C | /*3*/ D;
 let aa3: /*1*/ /*2*/ C | /*3*/ D /*4*/;
 
 type A1 = C;
 type A2 = C;
 type A3 = C;
 type A4 = C;
 type A5 = C;
-type A6 /*1*/ = C;
-type A7 /*1*/ = C;
-type A8 /*1*/ = C;
-type A9 /*1*/ = C;
-type A10 /*1*/ = /*2*/ C;
-type A11 /*1*/ = /*2*/ C;
-type A12 /*1*/ = C;
+type A6 = /*1*/ C;
+type A7 = /*1*/ C;
+type A8 = /*1*/ C;
+type A9 = /*1*/ C;
+type A10 = /*1*/ /*2*/ C;
+type A11 = /*1*/ /*2*/ C;
+type A12 = /*1*/ C;
 type A13 = /*1*/ C;
 
 type Aa1 = /*1*/ /*2*/ C | D;
 type Aa2 = /*1*/ /*2*/ C | /*3*/ D;
 type Aa3 = /*1*/ /*2*/ C | /*3*/ D /*4*/;
 
 type C1 = /*1*/ a | b;
 type C2 = /*1*/ a | b;
 type C3 = /*1*/ a | b;
-type C4 /*1*/ = a | b;
+type C4 = /*1*/ a | b;
 type C5 = /*1*/ a | b;
 type C6 /*0*/ = /*1*/ a | b;
 
 type Ctor = (new () => X) | Y;

```

**Prettier Similarity**: 92.59%


### typescript/union/with-type-params.ts

**Prettier Similarity**: 100.00%


### typescript/unique-symbol/unique-symbol.ts

**Prettier Similarity**: 100.00%


### typescript/unknown/unknown.ts

**Prettier Similarity**: 100.00%


### typescript/update-expression/update-expressions.ts

**Prettier Similarity**: 100.00%


### typescript/webhost/webtsc.ts

**Prettier Similarity**: 100.00%


