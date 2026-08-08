<script>
const is = 1;
const background = 2;
const myclass = 3;
const isActive = false;
</script>

<!-- Hyphenated shorthands cannot name a binding, so they must not register a
     reference. `is` and `background` stay unused: the property is a single
     token and is never split on the hyphen. -->
<div class:is-active style:background-color>Text</div>

<!-- Shorthands are case-sensitive: `myClass` must not resolve to `myclass`. -->
<div class:myClass>Cased</div>

<div class:enabled={isActive}>Enabled</div>
