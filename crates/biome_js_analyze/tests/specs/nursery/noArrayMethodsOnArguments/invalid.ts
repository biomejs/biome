/* should generate diagnostics */
function asAny() { (arguments as any).map(() => {}); }
function nonNull() { arguments!.map(() => {}); }
function typeAssertion() { (<any>arguments).map(() => {}); }
function satisfies() { (arguments satisfies IArguments as any).map(() => {}); }
