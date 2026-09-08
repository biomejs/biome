// should not generate diagnostics

type Flag = boolean;

let runningAs = true;
while (runningAs as Flag) {
    runningAs = false;
}

let runningAssertion = true;
while (<Flag>runningAssertion) {
    runningAssertion = false;
}

let runningSatisfies = true;
while (runningSatisfies satisfies Flag) {
    runningSatisfies = false;
}
