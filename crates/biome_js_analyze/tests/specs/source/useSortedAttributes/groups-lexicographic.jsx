// sortScope: "group", sortOrder: "lexicographic", groups: [:RESERVED:, :REST:]
// Lexicographic (byte) order applies within each group: "item10" sorts before
// "item2" (unlike natural order), and uppercase sorts before lowercase.

// Unsorted REST: lexicographic puts Zebra before apple, and item10 before item2
<Hello item2="a" key="1" item10="b" apple="c" Zebra="d" />;

// Correctly ordered: reserved key, then REST in lexicographic order
<Hello key="1" Zebra="d" apple="c" item10="b" item2="a" />;
