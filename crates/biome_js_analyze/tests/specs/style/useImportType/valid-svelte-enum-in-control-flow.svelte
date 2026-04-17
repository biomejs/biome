<!-- should not generate diagnostics -->
<script lang="ts">
  import { IfEnum, ElseIfEnum, EachEnum, EachKeyEnum, KeyEnum, AwaitEnum } from './models.ts';

  interface Props {
    foo: IfEnum;
    bar: ElseIfEnum;
    baz: EachEnum;
    bap: EachKeyEnum;
    qux: KeyEnum;
    zap: AwaitEnum;
  }
  let { foo }: Props = $props();
</script>

{#if foo === IfEnum.private}
  private
{:else if foo === ElseIfEnum.public}
  public
{/if}

{#each EachEnum.Foo as item (EachKeyEnum[item])}
  {item.name}
{/each}

{#key KeyEnum.Foo}
  <Component />
{/key}

{#await AwaitEnum.Foo}
  loading
{:then data}
  {data}
{/await}
