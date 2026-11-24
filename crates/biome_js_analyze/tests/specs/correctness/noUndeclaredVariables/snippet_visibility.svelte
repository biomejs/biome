<!-- should generate diagnostics -->
<script module lang="ts">
  type OnlyType = string;
  console.log(c\u0061rd, banner, m\u0069ssing);
  console.log(independent, captured, indirect, subscribed);
</script>
<script lang="ts">
  import { store } from './stores';
  let message = 'hello';
  console.log(independent, captured, indirect);
</script>
{#snippet independent(message)}{message} {store}{/snippet}
{#snippet card()}ok{/snippet}
{#snippet b\u0061nner()}ok{/snippet}
{#snippet subscribed()}{$store}{/snippet}
{#snippet captured()}{message}{/snippet}
{#snippet indirect()}{@render captured()}{/snippet}
{@render missing()}
{OnlyType}
