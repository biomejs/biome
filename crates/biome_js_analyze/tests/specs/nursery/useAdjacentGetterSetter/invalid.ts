class User {
    get name() {
        return 'John Doe';
    }
    set #age(age) {}
    get #age() {
        return 30;
    }
    set name(value) {}
}

const user = {
    get age() {
        return 30;
    },
    set name(value) {},
    get name() {
        return 'John Doe';
    },
    set age(age) {},
};

type UserType = {
    set age(age: number);
    set name(value: string);
    get name(): string;
    get age(): number;
}

interface UserInterface {
    set name(value: string);
    set age(age: number);
    get age(): number;
    get name(): string;
}

declare module 'module' {
    export class User {
        set name(value: string);
        get name(): string;
        set #age(age: number);
        get #age(): number;
    }
}
