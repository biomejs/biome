const arr: string[] = ["a", "b"];
arr.find((item) => item === "a") !== undefined;

const roArr: readonly string[] = ["a", "b"];
roArr.find((item) => item === "a") !== undefined;

const items: number[] = [1, 2, 3];
if (items.find((x) => x > 1)) {}

class Container {
    readonly items: string[] = [];
    has(id: string): boolean {
        return this.items.find((item) => item === id) !== undefined;
    }
}
