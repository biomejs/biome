interface Catalog extends Named, Tagged {
    item: Item;
    selected?: Item | null;
    state: boolean | "pending";
    empty: null;
    label: 'ready';
    find?(candidate: Item | null, state?: boolean | "pending"): Item | null;
    update: (item: Item, label?: "ready") => void;
}
interface Item {
    owner?: Catalog;
    title: string;
}
interface Named { name: string; }
interface Tagged { tag: "tag"; }
interface Catalog { count: number; }
interface Unselected { name: string; }
