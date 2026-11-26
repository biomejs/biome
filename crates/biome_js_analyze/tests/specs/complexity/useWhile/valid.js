/* should not generate diagnostics */
for (let i = 0; i < 16; ++i) {}

let i = 0;
for (; i < 16; ++i) {}

for (let i = 0; i < 16;) {
    i++;
}

for(x of [0]) {}

let k = 0;
while(k < 10) {
    k++;
}
