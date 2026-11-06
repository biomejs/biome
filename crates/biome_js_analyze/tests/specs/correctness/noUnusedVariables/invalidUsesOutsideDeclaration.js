/* should generate diagnostics */

// Reads whose value is discarded and updates in statement position are not uses.
let onlyDiscarded = 0;
onlyDiscarded;
(onlyDiscarded, 1);
for (onlyDiscarded; ; ) {}

let onlyUpdated = 0;
onlyUpdated++;
--onlyUpdated;
onlyUpdated += 1;
(onlyUpdated = 2), 0;

// References inside the declaration only.
function onlyRecursive() {
    return onlyRecursive();
}

const onlySelfScheduling = () => {
    setTimeout(onlySelfScheduling);
};

class OnlySelfReferencing {
    clone() {
        return new OnlySelfReferencing();
    }
}

// An arrow function parameter without parentheses, discarded or not read.
[1].map(bareUnused => 0);
[1].map(bareDiscarded => {
    bareDiscarded;
});
