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


### js/arrows/newline-before-arrow/newline-before-arrow.js
```diff
-async (x) => x;
+async;
+x;
+=> x

```

**Prettier Similarity**: 0.00%


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


### js/comments/html-like/comment.js
```diff
 <!--
-alert(1); 
+alert(1)
 -->

```

**Prettier Similarity**: 66.67%


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


### js/explicit-resource-management/valid-await-using-binding-escaped.js
```diff
 async function f() {
-  await using ab = c;
+  await using \u0061b = c;
 }

```

**Prettier Similarity**: 66.67%


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


### js/explicit-resource-management/valid-using-binding-escaped.js
```diff
 {
-  using ab = c;
+  using \u0061b = c;
 }

```

**Prettier Similarity**: 66.67%


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


### js/import-assertions/bracket-spacing/empty.js
```diff
-export * as bar from "bar.json" assert {};
+export * as bar from "bar.json";

```

**Prettier Similarity**: 0.00%


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


### js/import-attributes/bracket-spacing/empty.js
```diff
-export * as bar from "bar.json" with {};
+export * as bar from "bar.json";

```

**Prettier Similarity**: 0.00%


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


### js/objects/assignment-expression/object-property.js
```diff
 a = {
-  [(this.resource = resource)]: 1,
+  [this.resource = resource]: 1,
 };

```

**Prettier Similarity**: 66.67%


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


### js/range/whitespace.js
```diff
- 

```

**Prettier Similarity**: 0.00%


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


### js/sloppy-mode/function-declaration-in-while.js
```diff
-while (false) function foo() {}
+while (false) function foo(){}

```

**Prettier Similarity**: 0.00%


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


### js/test-declarations/optional.js
```diff
-describe?.(
-  "some string some string some string some string some string some string some string some string",
-  (done) => {},
-);
+describe?.("some string some string some string some string some string some string some string some string", (done) => {});

```

**Prettier Similarity**: 0.00%


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


### typescript/assignment/issue-5370.ts
```diff
-const durabilityMetricsSelectable: Immutable.OrderedSet<SomeReportingMetric> =
-  myExperienceSelectable.concat(otherDurabilityMetricsSelectable);
+const durabilityMetricsSelectable: Immutable.OrderedSet<
+  SomeReportingMetric,
+> = myExperienceSelectable.concat(otherDurabilityMetricsSelectable);

```

**Prettier Similarity**: 0.00%


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


### typescript/class/quoted-property.ts
```diff
 class User {
-  "username": string;
+  username: string;
 }

```

**Prettier Similarity**: 66.67%


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


