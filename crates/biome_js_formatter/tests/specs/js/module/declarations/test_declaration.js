describe("test", () => {
  it(``, async () => {
  });
});
  
describe("test", () => {
  it("wrooooooooooooooooooooooooooooooooooooooooooooong string" +
    "second string", async () => {
  });
});

it(`${foo + bar}
  handles
  some
    newlines does something really long and complicated 
    so I have to write a very long name for the test`, () => {
});

describe(`${foo + bar}`, 
  () => {}
);

describe(`${foo + bar} wroooooooooooooooooooooooooooooong string`, 
  () => {}
);

// TestOptions pattern (Vitest)
describe("with retry option", { retry: 2 }, () => {
  it("does something", () => {});
});

it("with timeout option", { timeout: 5000 }, () => {
  console.log("test");
});
