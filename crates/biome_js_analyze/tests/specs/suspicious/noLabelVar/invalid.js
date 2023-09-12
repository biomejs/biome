const x1 = "test";
x1: expr;

// nested scope
function test() {
  {
    x1: for (let i = 0; i < 10; i++) {
    }
  }
}
