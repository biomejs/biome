<!-- should not generate diagnostics -->
<script>
  import { actions, Catalog, price, title } from './catalog';
</script>
<Catalog.Row {title} />
<div use:actions.highlight>{$price}</div>
