<!-- biome-ignore format: preserve heading spacing -->
#    Preserved     heading

#    Formatted heading

<!-- biome-ignore format: preserve list spacing -->
-    preserved item

<!-- biome-ignore lint: keep this attached to the list -->
-    formatted item

Mixed    paragraph <!-- biome-ignore format: preserve the following heading -->
###    preserved after inline suppression

- parent

  <!-- biome-ignore format: preserve nested heading spacing -->
  ##    nested heading

> parent
>
> <!-- biome-ignore format: preserve quoted heading spacing -->
> #    quoted heading

> <!--
> biome-ignore format: preserve multiline quoted heading spacing
> -->
> ##    multiline quoted heading

<!-- biome-ignore format: preserve raw HTML -->
<!-- ordinary -->*literal*

<!-- biome-ignore format: preserve indented raw HTML -->
  <!-- ordinary -->*literal*

> Text before <!--
> biome-ignore format: preserve heading after an inline comment
> -->
> ##    inline quoted heading
