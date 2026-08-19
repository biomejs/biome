<template>
<div    class="a"   >formatted as markup</div>
</template>

<template lang="pug">
  .test
    #foo
  .bla
</template>

<template lang="">
<div    class="b"   >an empty lang names no preprocessor, so this is markup</div>
</template>

<template lang>
<div    class="c"   >and so does no value at all</div>
</template>

<docs>
This block is prose, and it may mention a `<my-component>` without closing it.
</docs>

<i18n>
{
  "en":   { "hello":   "Hello" }
}
</i18n>

<custom   lang="javascript"  >
const foo =            "</"
</custom>
