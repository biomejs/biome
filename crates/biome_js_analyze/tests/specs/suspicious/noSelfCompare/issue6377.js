function sum(...args) {
	let result = 0;
	for (const value of args) {
		result += value;
	}
	return result;
}

// False positive ❌
sum(1) === sum(1, 2);
sum(1) !== sum(1, 2);

// True positives ✅
sum(1) === sum(1);
sum(1) !== sum(1);

// True negatives ✅
sum(1) === sum(2);
sum(1) !== sum(2);
