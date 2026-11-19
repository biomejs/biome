<!-- should generate diagnostics -->

<!-- Static alt text with redundant words is flagged in Svelte files -->
<img alt="Photo of friend." />
<img alt="Image of cool person" aria-hidden="false" />
<!-- Redundant word in the static portion of a template value -->
<img alt="image of {person}" />
<img alt="photo by {photographer}" />
<img alt="picture: {description}" />
