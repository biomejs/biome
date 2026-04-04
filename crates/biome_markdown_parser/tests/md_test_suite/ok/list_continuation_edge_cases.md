# 1) Quote-only blank line inside list item

- > quoted line
  >
  > continued quote line

- sibling item

# 2) Marker-only item followed by blank line

- 

Outside paragraph after marker-only list item.

# 3) Nested list with continuation requiring virtual line restore

- outer item
  - inner item
    inner continuation
  outer continuation at parent indentation

- next outer item

# 4) Lazy continuation with insufficient indent after paragraph

- first line in item
lazy continuation line
- next item
