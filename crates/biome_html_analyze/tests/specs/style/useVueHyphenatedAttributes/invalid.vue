<!-- should generate diagnostics -->

<!-- Plain HTML attributes: these should be flagged -->
<div someAttr="x"></div>
<div SomeAttr="x"></div>
<div SOME_ATTR="x"></div>
<div some_attr="x"></div>

<!-- Vue v-bind: long-form and shorthand with non-hyphenated names -->
<div v-bind:fooBar="true"></div>
<div :fooBar="true"></div>

<div v-model:fooBar="true"></div>

<!-- Custom component props written in non-hyphenated forms -->
<MyComp someProp="x" AnotherProp="y"></MyComp>
<MyComp someProp="x" AnotherProp="y" />
