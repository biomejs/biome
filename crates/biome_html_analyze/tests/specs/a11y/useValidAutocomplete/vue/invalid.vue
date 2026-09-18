<!-- should generate diagnostics -->
<input type="text" autocomplete="foo" />
<input type="text" autocomplete="name invalid" />
<input type="text" autocomplete="invalid name" />
<input type="text" autocomplete="home url" />
<Bar autocomplete="baz"></Bar>
<Input type="text" autocomplete="baz" />
<!-- static Vue bindings with invalid autocomplete values should also be flagged -->
<input type="text" :autocomplete="'foo'" />
<input type="text" v-bind:autocomplete="'invalid name'" />
