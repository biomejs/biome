<!-- should not generate diagnostics -->
<script lang="ts">
import { Component, FooEnum } from './types';
</script>
<main>
  <Component />
  {FooEnum.Foo}
</main>
