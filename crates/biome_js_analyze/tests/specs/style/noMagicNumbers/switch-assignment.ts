type Mode = "first" | "second" | "arithmetic" | "indexing";

function example(mode: Mode) {
	switch (mode) {
		case "first":
			firstValue = 23;
			firstValue = 25;
			secondValue = 145;
			ratioValue = 4.5;
			break;
		case "second":
			firstValue = 23;
			secondValue = 125;
			ratioValue = 4;
			break;
		case "arithmetic":
			result = 31 + 32;
			result = 33 - 34;
			result = 35 * 36;
			result = 37 / 38;
			result = 39 ** 40;
			result = 41 % 42;
			result += 49;
			result -= 50;
			result *= 51;
			result /= 52;
			result **= 53;
			result %= 54;
			result = Math.sqrt(43);
			result = Math.cbrt(44);
			result = log(45, 46);
			result = root(47, 48);
			break;
		case "indexing":
			result = values.at(5);
			result = values.get(6);
			result = values[7 + 8];
			break;
	}
}
