<!-- should not generate diagnostics -->
<script>
const declared = 1;
</script>

<!-- A shorthand directive whose property matches no binding must stay silent:
     the shorthand produces no embedded JavaScript snippet, so there is no
     reference for this rule to resolve. -->
<div class:declared>Declared</div>
<div class:neverDeclared>Undeclared</div>
<div style:neverDeclaredEither>Undeclared</div>
