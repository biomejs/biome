// should generate diagnostics
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

/a/.test("abc");
