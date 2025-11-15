/* expect_diagnostic */
const listWithIncludes = {
    items: [1, 2, 3],
    includes(searchElement: number) {
        return this.items.includes(searchElement);
    },
    indexOf(searchElement: number) {
        return this.items.indexOf(searchElement);
    }
};
listWithIncludes.indexOf(2) !== -1;

/* expect_diagnostic */
const anotherListWithIncludes = {
    items: [1, 2, 3],
    includes(searchElement: number) {
        return this.items.includes(searchElement);
    },
    indexOf(searchElement: number) {
        return this.items.indexOf(searchElement);
    }
};
anotherListWithIncludes.indexOf(4) === -1;

/* expect_diagnostic */
/a/.test("abc");

/* expect_diagnostic */
/a/i.test("ABC");
