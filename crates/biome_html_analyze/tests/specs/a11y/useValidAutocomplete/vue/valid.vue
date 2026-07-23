<!-- should not generate diagnostics -->
<input type="text" />
<input type="text" autocomplete="name" />
<input type="text" autocomplete="" />
<input type="text" autocomplete="off" />
<input type="text" autocomplete="on" />
<input type="text" autocomplete="billing family-name" />
<input type="text" autocomplete="section-blue shipping street-address" />
<input type="text" autocomplete="section-somewhere shipping work email" />
<input type="text" autocomplete />
<Foo autocomplete="bar"></Foo>
<Input type="text" autocomplete="name" />
<Input type="text" autocomplete="baz" />
<input type="text" v-bind:autocomplete="autocompleteValue" />
<input type="text" :autocomplete="autocompleteValue" />
<!-- Static Vue bindings with valid autocomplete values are valid -->
<input type="text" :autocomplete="'name'" />
<input type="text" v-bind:autocomplete="'email'" />
