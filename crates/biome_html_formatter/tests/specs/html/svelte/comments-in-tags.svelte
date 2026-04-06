<!-- Block comments between attributes (already working) -->
<div    /* block comment */     class="foo"    /* block comment */     >text</div>
<div    /* block comment */     class="foo"  data-foo="info"   /* block comment */   data-bar="info"  >text</div>

<!-- Single-line comment between attributes — was producing invalid syntax before this fix -->
<div    // comment
class="foo"
>text</div>

<!-- // comment as the first attribute (no preceding node) -->
<div
// first comment
class="foo"
>text</div>

<!-- // comment as the last attribute before > -->
<div
class="foo"
// last comment
>text</div>

<!-- Multiple consecutive // comments -->
<div
// first line comment
// second line comment
class="foo"
>text</div>

<!-- // comment before a Svelte directive attribute -->
<div
// comment before directive
on:click={handler}
>text</div>

<!-- biome-ignore suppression comment on an attribute -->
<div
// biome-ignore format: reason
class   =   "foo"
data-bar   =   "baz"
>text</div>

<!-- // comment on a self-closing Svelte component -->
<Component
// comment
prop="value"
/>

<!-- /* block comment */ on a self-closing Svelte component -->
<Component
/* block comment */
prop="value"
/>

<!-- // biome-ignore suppression comment on a component attribute -->
<Component
// biome-ignore format: reason
prop="value"   data-bar="baz"
/>
