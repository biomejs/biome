<script>
let source = 1;
</script>

{let value = source}
{  let spaced = source}
{const first = 1, second = { value: "}" }}
{const regex = /[}]/}
{const comment = /* } */ { nested: true }}
{const template = `}`}

{value}
{first}
{second}
{regex}
{comment}
{template}
{letter}
{constellation}
{var invalid = 1}
