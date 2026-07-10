[1, 2, 3].filter(x => x > 1)[0];
const found1 = [1, 2, 3].filter(x => x > 1)[0];

[1, 2, 3].filter(x => x > 1).at(0);
const found2 = [1, 2, 3].filter(x => x > 1).at(0);

[1, 2, 3].concat([56, 76, 4543]).filter(x => x > 1)[0].toString();
[1, 2, 3].concat([56, 76, 4543]).filter(x => x > 1).at(0)?.toString();
