// ignoreCase: true, sortScope: "group" with default groups
// Case-insensitive comparison applies within each group independently.
// ASCII case-sensitive: uppercase letters sort before lowercase ('Z' < 'a').
// Case-insensitive: 'Z'/'z' > 'a', so "Zebra" comes after "ant".

// Unsorted: REST group has Bbb before aaa
// (case-sensitive 'B' < 'a', but case-insensitive 'b' > 'a')
<Hello Bbb="" aaa="" disabled key="1" onChange={fn} />;

// Unsorted: IMPLICIT group has Zebra before active
// (case-sensitive 'Z' < 'a', but case-insensitive 'z' > 'a')
<Hello Zebra active key="1" />;

// Correctly sorted (case-insensitive):
// IMPLICIT [active, Zebra], RESERVED [key], REST [aaa, Bbb], CALLBACK [onChange]
<Hello active Zebra key="1" aaa="" Bbb="" onChange={fn} />;
