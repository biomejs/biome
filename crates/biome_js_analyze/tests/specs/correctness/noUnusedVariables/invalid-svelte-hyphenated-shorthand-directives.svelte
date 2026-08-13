<script>
const active = false;
const color = "red";
const condition = true;
const actualColor = "blue";
</script>

<!-- Hyphenated shorthands cannot name a binding, so they must not register a
     reference. `active` and `color` stay unused: the property is a single
     token and is never split on the hyphen. -->
<div class:is-active class:active-state style:background-color style:color-scheme>Text</div>

<!-- Shorthands are case-sensitive: `Active` must not resolve to `active`. -->
<div class:Active>Cased</div>

<!-- Longhand directive property names are not references; only their
     initializers are. -->
<div class:active={condition} style:color={actualColor}>Identifier longhand</div>

<!-- CSS-only longhand properties remain complete literals. -->
<div
    class:is-active={condition}
    style:background-color={actualColor}
    style:--theme-color={actualColor}
>CSS properties</div>
