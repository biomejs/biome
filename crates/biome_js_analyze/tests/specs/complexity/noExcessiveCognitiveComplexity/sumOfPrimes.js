function sumOfPrimes(max) {
    let total = 0;
    OUT: for (let i = 1; i <= max; ++i) { // +1
        for (let j = 2; j < i; ++j) {     // +2
            if (i % j == 0) {             // +3
                continue OUT;             // +1
            }
        }
        total += 1;
    }
    return total;
}
