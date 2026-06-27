<!-- should not generate diagnostics -->

<!-- Fully dynamic alt: variable name is irrelevant, runtime value is unknown -->
<img alt={photo} />
<img alt={image} />
<img alt="{altText}" />
<!-- Static prefix contains no redundant words -->
<img alt="portrait of {person}" />
<img alt="{description} taken in {year}" />
