// Short, textual children can collapse onto the same line as self-closing
// elements if they start or end with a single character word or a space.
// Elements containing children will also remove all empty lines within them.
// This tests various permutations of that.

<>
  a<div />b
</>;
<>
  a<div />bb
</>;
<>
  aa<div />b
</>;
<>
  aa<div />bb
</>;

// As long as the first/last word of the text has a single character,
// it can stay on the same line.
<>
  a b<div />c
</>;
<>
  a bb<div />c
</>;
<>
  aa b<div />c
</>;
<>
  aa bb<div />c
</>;
<>
  a<div />b c
</>;
<>
  a<div />b ccc
</>;
<>
  a<div />bb cc
</>;
<>
  aa<div />b c
</>;
<>
  aa<div />b ccc
</>;
<>
  aa<div />bb cc
</>;
<>
  longword doesntmatter a<div />b
</>;
<>
  a<div />b longword doesntmatter
</>;


// Any character counts
<>
  1<div />b
</>;
<>
  11<div />b
</>;
<>
  ม<div />b
</>;
<>
  มม<div />b
</>;
<>
  !<div />b
</>;
<>
  !!<div />b
</>;

// Spaces also count
<>
  a <div />b
</>;
<>
  aa    <div />b
</>;

// Blank lines aren't kept if the children contain meaningful text
<>
line


2
</>;
<>
first


<div>second</div>


<div>third</div>
</>;