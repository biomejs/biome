/* should not generate diagnostics */
async function returnsPromise(): Promise<string> {
  return 'value';
}

let target: Promise<string> | undefined;
// Assigning a floating promise to a variable is considered handled (the
// caller presumably awaits/uses `target` later); this must not be flagged,
// and must not require type inference to determine that -- see the early
// bail-out for assignment expressions in `NoFloatingPromises::run`.
target = returnsPromise();
