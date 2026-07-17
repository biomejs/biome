<!-- should not generate diagnostics: {@html expr} marks the variable as used -->
<script lang="ts">
  interface Props {
    html: string;
    content: string;
    markup: string;
    preContent: string;
    preInline: string;
    preNested: string;
  }
  const { html, content, markup, preContent, preInline, preNested }: Props = $props();
</script>

{@html html}
{@html content.trim()}
{@html markup}

<!-- {@html expr} inside <pre> should also mark the variable as used -->
<pre>{@html preContent}</pre>

<!-- inline usage inside <pre> -->
<pre class="code">{@html preInline}</pre>

<!-- regular interpolation inside <pre> should also work -->
<pre>{preNested}</pre>
