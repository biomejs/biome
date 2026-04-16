1. Fork the repo and
    FF

    FF

2. Item two.
   Soft-wrap continuation at the required indent stays put.

3. Single extra space on a soft-wrap gets stripped.
   aligned line
    one-extra-space line

4. Two or more extra spaces are preserved (intentional alignment).
     deeply indented continuation

- a
   b

- outer
    - nested sub item
    - another nested sub

- item with hard line break  
  normal continuation after hard break

1. loose list paragraph with single-space excess

    exactly one extra space before this paragraph

1. loose list paragraph with multi-space excess

      lots of leading spaces here stays aligned

10. Multi-digit marker: required continuation indent is 4.
    aligned at 4 spaces
     one-extra-space continuation gets stripped
      two-extra-space preserved

    loose paragraph aligned at 4

     loose paragraph with single excess

       loose paragraph with multi excess

100. Triple-digit marker: required continuation indent is 5.
     aligned at 5 spaces
      one-extra-space continuation gets stripped
       two-extra-space preserved

     loose paragraph aligned at 5

      loose paragraph with single excess

        loose paragraph with multi excess
