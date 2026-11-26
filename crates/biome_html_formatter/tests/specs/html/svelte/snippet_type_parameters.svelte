<script lang="ts">
</script>

{#snippet row<T   extends string>(item:T)}
  <li>{item}</li>
{/snippet}

{#snippet pair<const A,B=A>(a:A, b:B)}
  {a}{b}
{/snippet}

{#snippet table<TFirstVeryLongTypeParameter extends Record<string, unknown>, TSecondVeryLongTypeParameter>(rows: TFirstVeryLongTypeParameter[], columns: TSecondVeryLongTypeParameter[])}
  {rows.length}
{/snippet}
