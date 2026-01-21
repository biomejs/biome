// Invalid cases for useArraySome

// .filter().length
[1, 2, 3].filter((x) => x > 1).length > 0;
[1, 2, 3].filter((x) => x > 1).length !== 0;
[1, 2, 3].filter((x) => x > 1).length != 0;
[1, 2, 3].filter((x) => x > 1).length >= 1;
!![1, 2, 3].filter((x) => x > 1).length;

// .find() / .findLast()
[1, 2, 3].find((x) => x > 1) !== undefined;
[1, 2, 3].find((x) => x > 1) != null;
[1, 2, 3].findLast((x) => x > 1) !== undefined;
[1, 2, 3].findLast((x) => x > 1) != null;

// .findIndex() / .findLastIndex()
[1, 2, 3].findIndex((x) => x > 1) !== -1;
[1, 2, 3].findIndex((x) => x > 1) > -1;
[1, 2, 3].findLastIndex((x) => x > 1) !== -1;
[1, 2, 3].findLastIndex((x) => x > 1) > -1;
