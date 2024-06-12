/* should not generate diagnostics */
function fooTs() {
  let a;
}

const barTs = () => {
  let b;
}


function fooWithCommentTS() {
  // should work
}

const barWithCommentTs = () => {
  // should work
}

function fooWithMultilineCommentTS() {
  /**
   * this should also work
   */
}

const barWithMultilineCommentTs = () => {
  /**
   * this should also work
   */
}

let fooVarTs;
if (fooVarTs) {
  // empty
}

while (fooVarTs) {
  /* empty */
}

const doSomethingTs = () => null;
try {
  doSomethingTs();
} catch (ex) {
  // continue regardless of error
}

try {
  doSomethingTs();
} finally {
  /* continue regardless of error */
}

class FooTs {
  static {
      bar();
  }
}

class FoozTs {
  static {
      // comment
  }
}

// biome-ignore lint/suspicious/noEmptyBlockStatements: this should be allowed
function shouldNotFailTs() {

}

// This is using parameter properties, and the empty constructor should be allowed
export class FooBar {
  constructor(
    private foo: string,
  ) { }
}

class FooBarPrivate {
  private constructor() { }
}
