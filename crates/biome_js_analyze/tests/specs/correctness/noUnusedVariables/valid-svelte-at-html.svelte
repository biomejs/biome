<!-- should not generate diagnostics: {@html expr} marks the variable as used -->
<script lang="ts">
  interface Props {
    html: string;
    content: string;
    markup: string;
  }
  const { html, content, markup }: Props = $props();
</script>

{@html html}
{@html content.trim()}
{@html markup}
