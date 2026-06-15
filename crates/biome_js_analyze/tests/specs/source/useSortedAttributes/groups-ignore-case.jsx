// ignoreCase: true, sortScope: "global" (flat sort, case-insensitive)

// Unsorted: uppercase Z before lowercase a without ignoreCase would be fine,
// but with ignoreCase "Zip" should sort after "name"
<Hello Zip="NY" name="John" tel={5555555} />;

// Correctly sorted case-insensitively: "age", "Name", "zip"
<Hello age={30} Name="John" zip="NY" />;

// Unsorted: "Bbb" should come before "ccc" case-insensitively
<Hello ccc="" Bbb="" aaa="" />;
